use std::error::Error;

mod server;

fn main() -> Result<(), Box<dyn Error>> {
    let server = server::Server::new("token.secret")?;
    server.run()
}
