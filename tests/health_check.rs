fn spawn_app() {
  let server = zero2prod::run().expect("Failed to bind address"); 
  // Launch the server as a background task 
  // tokio::spawn returns a handle to the spawned future, 
  // but we have no use for it here, hence the non-binding let 
  let _ = tokio::spawn(server);
}

#[tokio::test] 
async fn health_check_works() {
  // spawning app in the background
  spawn_app();  

  // create a new client to connect to the server
  let client = reqwest::Client::new();

  // send a GET request to the health_check endpoint
  let response = client.get("http://127.0.0.1:8000/health_check") .send() .await .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success()); assert_eq!(Some(0), response.content_length());
}