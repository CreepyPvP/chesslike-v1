use error::AppError;
use service::start_service;

use crate::server::start_server;

mod error;
mod room;
mod server;
mod service;

fn main() -> Result<(), AppError> {
    let srv = start_server()?;
    let port = 3000;
    let service_thread = start_service(port, srv).expect("Could not create service thread");
    println!("Server listening on port: *{}", port);

    let _ = service_thread.join();

    Ok(())
}
