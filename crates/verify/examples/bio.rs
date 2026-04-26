use luduvo_verify::{Client, DiscordUser, Settings, codegen::CodeComplexity};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let settings = Settings::new(None, None);
    let mut client = Client::new(Some(settings));

    let code = client.generate_code(Some(CodeComplexity::Low)).to_string();
    let user = DiscordUser { id: 0 };

    println!("verification code: {}", code);

    loop {
        print!("press enter if the code is in your bio");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match client
            .is_verified("primiti_ve".to_string(), user, code.clone())
            .await
        {
            Ok(true) => {
                println!("user is verified!");

                break;
            }

            Ok(false) => println!("not verified yet, try again:\n"),
            Err(e) => eprintln!("error checking verification: {}\n", e),
        }
    }
}
