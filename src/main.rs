use html5ever::tendril::TendrilSink;
use rush::{layout::print_to_terminal, net, parser::parse_html};
use std::env;

#[tokio::main]
// TODO (alivenotions): Add a custom error here
async fn main() -> Result<(), reqwest::Error> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No URL provided, using default.");
            // TODO (alivenotions): replace with rush's homepage 🙃
            "https://alivenotions.com".into()
        }
    };

    let doc = net::get_webpage(&url).await?;
    // How is a mut ref possible to an immutable value?/?
    let html = parse_html()
        .from_utf8()
        .read_from(&mut doc.as_bytes())
        .unwrap();
    print_to_terminal(&html.document);
    Ok(())
}
