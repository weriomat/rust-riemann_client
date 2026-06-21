# riemann_client [![](https://img.shields.io/crates/v/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/crates/l/riemann_client.svg)](https://crates.io/crates/riemann_client) [![](https://img.shields.io/travis/borntyping/rust-riemann_client.svg)](https://travis-ci.org/borntyping/rust-riemann_client)

A [Riemann](http://riemann.io/) client library and command line interface.

- [Source on GitHub](https://github.com/borntyping/rust-riemann_client)
- [Packages on Crates.io](https://crates.io/crates/riemann_client)
- [Builds on Travis CI](https://travis-ci.org/borntyping/rust-riemann_client)

## Usage

```bash
$ riemann-cli send --service riemann_cli --state ok --metric-d 11
--> { Event { time: None, state: Some("ok"), service: Some("riemann_cli"), host: None, description: None, tags: [], ttl: None, attributes: [], time_micros: None, metric_sint64: None, metric_d: Some(11.0), metric_f: None, special_fields: SpecialFields { unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 } } } }
<-- { Msg { ok: Some(true), error: None, states: [], query: MessageField(None), events: [], special_fields: SpecialFields { unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 } } } }
```

```bash
$ riemann-cli query 'service = "riemann_cli"'
HOSTNAME   TIME       SERVICE              METRIC     STATE
           1432128319 riemann_cli          11         ok
```

Run `riemann-cli --help` for a list of options available for the command line interface.

See the `examples` directory for examples of querying and sending events with the library.

## Development

To build the library alone, without the command line interface and it's dependencies, run `cargo build --lib --no-default-features`.

The protocol buffer definition can be updated by replacing `src/proto/mod.proto` with the [latest definition from the Riemann source](https://raw.githubusercontent.com/riemann/riemann-java-client/refs/heads/main/riemann-java-client/src/main/proto/riemann/proto.proto) and running `make`. You will need to have `protoc` and `protoc-gen-rust` installed. `protoc` is provided by the `protobuf-compiler` package on Debian based systems. Instructions for installing `protoc-gen-rust` this are available in the [README for rust-protobuf](https://github.com/stepancheg/rust-protobuf).

### mTLS

To create the test certificates the following commands were used.

```bash
cd test_certs

# create the CA
openssl req -x509 -newkey rsa:2048 -keyout ca.key -out ca.pem -nodes -days 999999 -subj "/CN=TestCA"

# create the client cert
openssl req -newkey rsa:2048 -keyout client.key -out client.csr -nodes -subj "/CN=client"
openssl x509 -req -in client.csr -CA ca.pem -CAkey ca.key -CAcreateserial -out client.pem -days 999999
```

## Licence

`riemann_client` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

It was also directly inspired by the Python [riemann-client](http://github.com/borntyping/python-riemann-client) by the same author.

## Authors

Written by [Sam Clements](sam@borntyping.co.uk).
