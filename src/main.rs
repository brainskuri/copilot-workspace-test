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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use reqwest::Response;

    async fn mock_request(url: &str) -> Result<Response, reqwest::Error> {
        reqwest::get(url).await
    }

    #[tokio::test]
    async fn test_valid_url() {
        let url = "https://www.example.com";
        let response = mock_request(url).await.unwrap();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let url = "https://invalid.url";
        let response = mock_request(url).await;
        assert!(response.is_err());
    }
}
