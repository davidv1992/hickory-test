#[tokio::test]
async fn test() {
    use hickory_resolver::{TokioResolver};
    let mut builder = TokioResolver::builder_tokio().unwrap();
    builder.options_mut().validate = true;
    let resolver = builder.build().unwrap();
    resolver.srv_lookup("_ntske._tcp.localhost").await;
}
