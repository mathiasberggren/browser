use html5ever::tendril::TendrilSink;
use rush::{layout::print_to_terminal, net, parser::parse_html, window};
use std::env;
use url::Url;

#[tokio::main]
// TODO (alivenotions): Add a custom error here
async fn main() -> Result<(), reqwest::Error> {
    let url = match env::args().nth(1) {
        Some(url) => parse_url(&url),
        None => {
            println!("No URL provided, using default.");
            // TODO (alivenotions): replace with rush's homepage 🙃
            "https://alivenotions.com".into()
        }
    };

    window::create_window();

    println!("url: {}", url);
    let doc = net::get_webpage(&url).await?;
    let html = parse_html()
        .from_utf8()
        // How is a mut ref possible to an immutable value?/?
        .read_from(&mut doc.as_bytes())
        .unwrap();
    print_to_terminal(&html.document);
    Ok(())
}

fn parse_url(url: &str) -> String {
    if let Ok(res) = Url::parse(url) {
        res.to_string()
    } else {
        let search_engine = Url::parse_with_params("https://duckduckgo.com/", [("q", url)]);
        search_engine.unwrap().to_string()
    }
}
