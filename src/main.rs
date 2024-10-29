use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    match reqwest::get(url).await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => println!("{}", text),
                Err(err) => eprintln!("Error reading response text: {}", err),
            }
        }
        Err(err) => eprintln!("Error making request: {}", err),
    }
}
