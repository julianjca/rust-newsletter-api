use std::net::TcpListener;
fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0") .expect("Failed to bind random port"); 
  // We retrieve the port assigned to us by the OS 
  let port = listener.local_addr().unwrap().port(); let server = zero2prod::run(listener).expect("Failed to bind address"); 
  let _ = tokio::spawn(server); 
  // We return the application address to the caller! 
  format!("http://127.0.0.1:{}", port)
}

#[tokio::test] 
async fn health_check_works() {
  // spawning app in the background
  let address = spawn_app();

  // create a new client to connect to the server
  let client = reqwest::Client::new();

  // send a GET request to the health_check endpoint
  let response = client.get(&format!("{}/health_check", &address)) .send() .await .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success()); assert_eq!(Some(0), response.content_length());
}