use server::Server;

mod protocol;
mod server;

#[tokio::main]
async fn main() {
    let server = Server;
    server.start(3030).await.unwrap();
}
