#[cfg(test)]
mod integration_tests {
    use super::*;
    use warp::Filter;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_integration() {
        // Define a simple route for testing
        let route = warp::path!("test").map(|| warp::reply::with_status("OK", StatusCode::OK));

        // Simulate a request to the test route
        let response = warp::test::request()
            .method("GET")
            .path("/test")
            .reply(&route)
            .await;

        // Check if the response is as expected
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), "OK");
    }
}
