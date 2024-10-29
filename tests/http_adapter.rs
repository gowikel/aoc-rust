use aoc::providers::http::{AOCUrl, HTTPAdapter, HTTPError, HTTPProvider};

struct URLMock {
    url: String,
}

impl URLMock {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl AOCUrl for URLMock {
    fn url(&self) -> String {
        self.url.clone()
    }
}

#[test]
fn test_get_endpoint() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/test-endpoint")
        .match_header("Cookie", "TEST-COOKIE")
        .with_status(200)
        .with_body("TEST-SUCCESS")
        .create();

    let mut adapter = HTTPAdapter::default();
    adapter.set_cookie("TEST-COOKIE".to_string());

    let url = URLMock::new(server.url() + "/test-endpoint");
    let result = adapter.get(&url);

    mock.assert();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "TEST-SUCCESS");
}

#[test]
fn test_cookie_is_expected() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/test-endpoint")
        .match_header("Cookie", "TEST-COOKIE")
        .with_status(200)
        .with_body("TEST-SUCCESS")
        .create();

    let adapter = HTTPAdapter::default();
    let url = URLMock::new(server.url() + "/test-endpoint");
    let result = adapter.get(&url);

    assert!(!mock.matched()); // We didn't call the server with an invalid value
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        HTTPError::MissingEnvVarError("AOC_COOKIE not set".into())
    );
}

#[test]
fn test_fetch_error() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/test-endpoint")
        .match_header("Cookie", "TEST-COOKIE")
        .with_status(400)
        .with_body("FAILED")
        .create();

    let mut adapter = HTTPAdapter::default();
    adapter.set_cookie("TEST-COOKIE".to_string());
    let url = URLMock::new(server.url() + "/test-endpoint");
    let result = adapter.get(&url);

    mock.assert();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        HTTPError::FetchError(format!(
            "HTTP status client error (400 Bad Request) for url ({})",
            server.url() + "/test-endpoint"
        ))
    );
}
