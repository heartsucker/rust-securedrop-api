extern crate env_logger;
extern crate securedrop_api;

use securedrop_api::auth::UserPassTotp;
use securedrop_api::data::Reply;
use securedrop_api::{Client, Result};

fn main() {
    run_main().unwrap();
}

fn run_main() -> Result<()> {
    env_logger::init();

    let creds = UserPassTotp::new(
        "journalist".into(),
        "WEjwn8ZyczDhQSK24YKM8C9a".into(),
        // b32: JHCOGO7VCER3EJ4L
        "49C4E33BF51123B2278B".into(),
    );

    println!("Initializing client with creds: {:?}\n", creds);
    let client = Client::new("http://localhost:8081".parse().unwrap(), creds.into())?;

    let user = client.user()?;
    println!("Current user is: {:?}\n", user);

    let sources = client.sources()?;
    println!("There are {} sources.\n", sources.sources().len());

    let source = client.source(sources.sources()[0].filesystem_id())?;
    println!("The first source is: {:?}\n", source);

    let submissions = client.source_submissions(source.filesystem_id())?;
    let mut buf = Vec::new();
    client.download_submission(
        source.filesystem_id(),
        submissions.submissions()[0].submission_id(),
        &mut buf,
    )?;
    println!(
        "Downloaded {} ({} bytes)\n",
        submissions.submissions()[0].filename(),
        buf.len()
    );

    let reply_str =
        "-----BEGIN PGP MESSAGE-----\nshould be encrypted :(\n-----END PGP MESSAGE-----";
    let reply = Reply::new(reply_str);
    client.reply_to_source(source.filesystem_id(), reply)?;
    println!("Reply sent: \n{}\n", reply_str);
    println!("Done");
    Ok(())
}
