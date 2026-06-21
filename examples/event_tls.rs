use riemann_client::proto::Event;
use riemann_client::Client;

fn main() {
    let mut client =
        Client::connect_tls("myhost_name", 5554, "myCA.pem", "mycert.pem", "mykey").unwrap();

    client
        .event({
            let mut event = Event::new();
            event.service = Some("rust-riemann_client".to_string());
            event.state = Some("ok".to_string());
            event.metric_d = Some(128.128);
            event
        })
        .unwrap();

    // client.event(riemann_client::Event {
    //     service: Some("rust-riemann_client"),
    //     state: Some("ok"),
    //     metric_d: Some(128.128)
    //     ..Event::new()
    // }).unwrap()
}
