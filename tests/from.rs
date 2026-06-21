use riemann_client::proto::Query;

#[test]
fn query_from_str() {
    let _ = Query::from("hello world");
}

#[test]
fn query_from_string() {
    let _ = Query::from("hello world".to_string());
}

#[test]
fn query_from_query() {
    let _ = Query::from(Query::new());
}
