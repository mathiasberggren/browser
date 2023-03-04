use rush::{window};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let window = window::create_window();
    window.run();

    Ok(())
}