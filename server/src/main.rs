use error::AppError;
use service::start_service;

mod error;
mod service;

fn main() -> Result<(), AppError> {
    let port = 3000;
    let service_thread = start_service(port).expect("Could not create service thread");
    println!("Server listening on port: *{}", port);

    let _ = service_thread.join();

    Ok(())
}
