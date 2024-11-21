use server::Server;

mod locator;
mod protocol;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = Server::new();
    server.start(3030).await.unwrap();
}
