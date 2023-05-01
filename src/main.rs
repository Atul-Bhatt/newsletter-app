use newsletter_app::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
                                        .expect("Failed to bind to port.");
    run(listener)?.await  
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;

//     #[actix_rt::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }