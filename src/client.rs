//! API client.

use reqwest::header::{Accept, Authorization as AuthHeader, ContentType, Headers};
use reqwest::{self, Client as HttpClient, Response as HttpResponse, Url};
use serde::de::DeserializeOwned;
use std::io::Write;

use super::Result;
use auth::{Authorization, Credentials};
use data::{Reply, Response, Source, Sources, Submission, Submissions, User};
use error::{Error, ErrorKind};

/// A client used to interact with the SecureDrop API. This client handles authentication and
/// retries.
pub struct Client {
    url_base: Url,
    http: HttpClient,
    auth: Authorization,
}

impl Client {
    /// Construct a new `Client` from a URL base (e.g., `http://localhost:8081` or
    /// `https://someonionservice.onion/some/path/`) and a set of credentialized used to acquire
    /// and initial auth token.
    ///
    /// Creation of a client will return an `Err` if it fails to authenticate.
    pub fn new<C>(url_base: Url, credentials: C) -> Result<Self>
    where
        C: Into<Credentials>,
    {
        let mut client = Self {
            url_base: url_base,
            http: HttpClient::new(),
            auth: Authorization::Credentials(credentials.into()),
        };
        client.authorize()?;
        Ok(client)
    }

    fn url(&self, path: &str) -> Url {
        let mut url = self.url_base.clone();
        url.set_path(&format!("api/v1/{}", path));
        url
    }

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::json());
        headers.set(Accept::json());
        self.auth_header(&mut headers);
        headers
    }

    fn auth_header(&self, headers: &mut Headers) {
        match self.auth {
            Authorization::Token(ref token) => headers.set(AuthHeader(format!("Token {}", token))),
            Authorization::Credentials(_) => (),
        }
    }

    /// Reauthorize the client using a new set of credentials. This may need to be done if a client
    /// suffers network errors and loses authentication. This will return an `Err` if if fails to
    /// authenticate.
    pub fn reauthorize<C>(&mut self, credentials: C) -> Result<()>
    where
        C: Into<Credentials>,
    {
        self.auth = Authorization::Credentials(credentials.into());
        self.authorize()
    }

    fn authorize(&mut self) -> Result<()> {
        let url = self.url("token");
        let headers = self.headers();
        let resp = match self.auth {
            Authorization::Credentials(ref creds) => {
                self.http.post(url).headers(headers).json(&creds).send()
            }
            Authorization::Token(_) => self.http.post(url).headers(headers).send(),
        };
        let auth = Self::parse_json(resp)?;
        self.auth = Authorization::Token(auth);
        Ok(())
    }

    fn parse_json<T>(resp: ::std::result::Result<HttpResponse, reqwest::Error>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Self::parse_req(resp, |resp| match resp.json::<T>() {
            Ok(a) => Ok(a),
            Err(e) => Err(ErrorKind::ProgrammingError(e.to_string()).into()),
        })
    }

    fn parse_req<T, F>(
        mut resp: ::std::result::Result<HttpResponse, reqwest::Error>,
        func: F,
    ) -> Result<T>
    where
        F: FnOnce(&mut HttpResponse) -> Result<T>,
    {
        match resp {
            Ok(ref mut resp) if resp.status().is_success() => func(resp),
            Ok(mut resp) => {
                if resp.status().is_server_error() {
                    Err(ErrorKind::ServerError.into())
                } else if resp.status().is_client_error() {
                    let err = match resp.json() {
                        Ok(err) => err,
                        Err(_) => {
                            return Err(ErrorKind::ProgrammingError("Parse failure.".into()).into())
                        }
                    };
                    Err(ErrorKind::ClientError(err).into())
                } else {
                    Err(ErrorKind::UnknownError.into())
                }
            }
            Err(err) => {
                if !err.is_http() {
                    Err(ErrorKind::NetworkError.into())
                } else if err.is_server_error() {
                    Err(ErrorKind::ServerError.into())
                } else {
                    Err(ErrorKind::UnknownError.into())
                }
            }
        }
    }

    /// Retrieve all sources the logged in user is permitted to view.
    ///
    /// Corresponds to `GET /api/v1/sources`.
    pub fn sources(&self) -> Result<Sources> {
        let resp = self
            .http
            .get(self.url("sources"))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Retrieve one source by ID.
    ///
    /// Corresponds to `GET /api/v1/source/<str:filesystem_id>`.
    pub fn source(&self, filesystem_id: &str) -> Result<Source> {
        let resp = self
            .http
            .get(self.url(&format!("sources/{}", filesystem_id)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Retrieve all submissions for a given source.
    ///
    /// Corresponds to `GET /api/v1/source/<str:filesystem_id>/submissions`.
    pub fn source_submissions(&self, filesystem_id: &str) -> Result<Submissions> {
        let resp = self
            .http
            .get(self.url(&format!("sources/{}/submissions", filesystem_id)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Retrieve one submission from a given source.
    ///
    /// Corresponds to `GET /api/v1/soruces/<str:filesystem_id>/submissions/<int:submission_id>`.
    pub fn source_submission(&self, filesystem_id: &str, submission_id: u32) -> Result<Submission> {
        let resp = self
            .http
            .get(self.url(&format!(
                "sources/{}/submissions/{}",
                filesystem_id, submission_id
            )))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Send a pre-encrypted reply to the given source.
    ///
    /// Corresponds to `POST /api/v1/sources/<str:filesystem_id>/reply`.
    pub fn reply_to_source(&self, filesystem_id: &str, reply: &Reply) -> Result<Response> {
        let resp = self
            .http
            .post(self.url(&format!("sources/{}/reply", filesystem_id)))
            .headers(self.headers())
            .json(&reply)
            .send();
        Self::parse_json(resp)
    }

    /// Delete one submission for a given source.
    ///
    /// Corresponds to `DELETE /api/v1/sources/<str:filesystem_id>/submissions/<int:submission_id>`.
    pub fn delete_source_submission(
        &self,
        filesystem_id: &str,
        submission_id: u32,
    ) -> Result<Submission> {
        let resp = self
            .http
            .delete(self.url(&format!(
                "sources/{}/submissions/{}",
                filesystem_id, submission_id
            )))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Download one submission to a sink (`Write`).
    ///
    /// Corresponds to `GET
    /// /api/v1/sources/<str:filesystem_id>/submissions/<int:submission_id>/download`.
    pub fn download_submission<W>(
        &self,
        filesystem_id: &str,
        submission_id: u32,
        mut write: W,
    ) -> Result<()>
    where
        W: Write,
    {
        let mut headers = Headers::new();
        headers.set(ContentType("appication/pgp-encrypted".parse().unwrap()));
        self.auth_header(&mut headers);
        let resp = self
            .http
            .get(self.url(&format!(
                "sources/{}/submissions/{}/download",
                filesystem_id, submission_id
            )))
            .headers(headers)
            .send();
        Self::parse_req(resp, move |resp| {
            resp.copy_to(&mut write)
                .map_err(|e| Error::new(ErrorKind::IO(format!("{:?}", e))))?;
            Ok(())
        })
    }

    /// Delete a source and all submissions.
    ///
    /// Corresponds to `DELETE /api/v1/sources/<str:filesystem_id>/submissions>`.
    pub fn delete_submissions(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .delete(self.url(&format!("sources/{}/submissions", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Add a star to a source.
    ///
    /// Corresponds to `POST /api/v1/soruces/<str:filesystem_id>/star`.
    pub fn star_source(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .post(self.url(&format!("sources/{}/star", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Remove a star from a source.
    ///
    /// Corresponds to `DELETE /api/v1/soruces/<str:filesystem_id>/star`.
    pub fn unstar_source(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .delete(self.url(&format!("sources/{}/star", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    /// Retrieve information about the logged in user.
    ///
    /// Corresponds to `GET /api/v1/user`.
    pub fn user(&self) -> Result<User> {
        let resp = self
            .http
            .get(self.url("user"))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }
}
