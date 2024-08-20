#[cfg(test)]
mod unit_tests {
    use super::*;
    use warp::test::request;
    use warp::Filter;

    // Example function to create a simple Warp filter
    fn hello_filter() -> impl warp::Filter<Extract = (String,), Error = warp::Rejection> + Clone {
        warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name))
    }

    #[test]
    fn test_unit() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_hello() {
        let api = hello_filter();

        // Simulate a request to the `/hello/world` route
        let response = request()
            .path("/hello/world")
            .reply(&api);

        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "Hello, world!");
    }
}
