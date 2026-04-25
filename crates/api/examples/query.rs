use luduvo_api::users::query::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let username = if args.len() < 2 {
        println!("no username supplied, getting profile data for username `Luduvo`");
        
        "Luduvo"
    } else {
        &args[1]
    }.to_string();

    let mut client = Client::new(None);

    match client.get_user(username.clone(), None).await {
        Ok(profile) => {
            println!("profile for username `{username}`: {:#?}", profile);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get profile for id `{username}`: '{:#?}'",
                e
            );
        }
    }
}
