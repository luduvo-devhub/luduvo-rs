use luduvo_api::users::friends::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let id = if args.len() < 2 {
        println!("no id supplied, getting profile data for id `1`");

        "1"
    } else {
        &args[1]
    }.to_string();

    let mut client = Client::new(None);

    match client.get_friends(id.clone()).await {
        Ok(friends) => {
            println!("friends for id `{id}`: {:#?}", friends);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get friends for id `{id}`: '{:#?}'",
                e
            );
        }
    }
}
