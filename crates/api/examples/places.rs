use luduvo_api::places::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new(None);

    match client.get_places("test".to_string(), None).await {
        Ok(friends) => {
            println!("places data: {:#?}", friends);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get places`: '{:#?}'",
                e
            );
        }
    }
}
