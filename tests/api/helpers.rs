use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    //pub port: u16,
    pub api: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    // binding to port 0 requests a random, available port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = movie_nights::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);

    let client = reqwest::Client::builder().build().unwrap();

    TestApp {
        address: format!("http://localhost:{}", port),
        //port: port,
        api: client,
    }
}
