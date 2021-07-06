// You can inspect what code gets generated using
//`cargo expand --test health_check`(<- name of the test file)

#[actix_rt::test]
async fn health_check_works() {
    let client = reqwest::Client::new();
    spawn_app();

    //Act
    let response = client 
    .get("http://127.0.0.1:8080/health_check")
    .send()
    .await
    .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    
}

//Launch our application in the background

 fn spawn_app() {
    let server = z2p::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}

