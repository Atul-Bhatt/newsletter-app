// !tests/health_check.rs

use newsletter_app::run;
use reqwest;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    
    let client = reqwest::Client::new();
    
    let response = client
                    .get(&format!("{}/health_check", &address))
                    .send()
                    .await
                    .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_a_valid_test_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=Atul%20Bhatt%20email=atulbhatt61%40gmail.com";

    let response = client
                            .post(&format!("{}/subscriptions", &address))
                            .header("Content-Type", "applicaiton/x-www-form-urlencoded")
                            .body(body)
                            .send()
                            .await
                            .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_for_a_invalid_test_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Atul%20Bhatt", "Missing Email Address."),
        ("email=atulbhatt61%40gmail.com", "Missing Name."),
        ("", "Missing Email Address and Name.")
        ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message for test failure
            "The API did not fail with 400 bad request when the payload was {}",
            error_message
        );
            
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bin address.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}