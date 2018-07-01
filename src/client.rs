use reqwest::header::{Accept, Authorization as AuthHeader, ContentType, Headers};
use reqwest::{self, Client as HttpClient, Response as HttpResponse, Url};
use serde::de::DeserializeOwned;
use std::io::Write;

use super::Result;
use auth::{Authorization, Credentials};
use data::{Reply, Response, Source, Sources, Submission, Submissions, User};
use error::{Error, ErrorKind};

pub struct Client {
    url_base: Url,
    http: HttpClient,
    credentials: Credentials,
    auth: Option<Authorization>,
}

impl Client {
    pub fn new(url_base: Url, credentials: Credentials) -> Result<Self> {
        let mut client = Self {
            url_base: url_base,
            http: HttpClient::new(),
            credentials: credentials,
            auth: None,
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
            Some(Authorization::Token(ref token)) => {
                headers.set(AuthHeader(format!("Token {}", token)))
            }
            None => (),
        }
    }

    pub fn reauthorize(&mut self, credentials: Credentials) -> Result<()> {
        self.credentials = credentials;
        self.auth = None;
        self.authorize()
    }

    fn authorize(&mut self) -> Result<()> {
        let resp = self
            .http
            .post(self.url("token"))
            .headers(self.headers())
            .json(&self.credentials)
            .send();
        let auth = Self::parse_json(resp)?;
        self.auth = Some(Authorization::Token(auth));
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

    pub fn sources(&self) -> Result<Sources> {
        let resp = self
            .http
            .get(self.url("sources"))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    pub fn source(&self, filesystem_id: &str) -> Result<Source> {
        let resp = self
            .http
            .get(self.url(&format!("sources/{}", filesystem_id)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    pub fn source_submissions(&self, filesystem_id: &str) -> Result<Submissions> {
        let resp = self
            .http
            .get(self.url(&format!("sources/{}/submissions", filesystem_id)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

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

    pub fn reply_to_source(&self, filesystem_id: &str, reply: Reply) -> Result<Response> {
        let resp = self
            .http
            .post(self.url(&format!("sources/{}/reply", filesystem_id)))
            .headers(self.headers())
            .json(&reply)
            .send();
        Self::parse_json(resp)
    }

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

    pub fn delete_submissions(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .delete(self.url(&format!("sources/{}/submissions", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    pub fn star_source(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .post(self.url(&format!("sources/{}/star", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    pub fn unstar_source(&self, filesystem_id: &str) -> Result<Response> {
        let resp = self
            .http
            .delete(self.url(&format!("sources/{}/star", filesystem_id,)))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }

    pub fn user(&self) -> Result<User> {
        let resp = self
            .http
            .get(self.url("user"))
            .headers(self.headers())
            .send();
        Self::parse_json(resp)
    }
}
