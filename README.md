# Theme Parks Wiki Rust Client

A Rust client for the [Theme Parks Wiki V1 API](https://api.themeparks.wiki/docs/v1/).

## Documentation

- [themeparks-rust crates.io](https://crates.io/crates/themeparks)

## Usage

```rust
use themeparks::themeparks::Client;

let client = Client::new();
let destinations = client.destinations().list_destinations().unwrap().data.unwrap();
```

## Contributing

Contibutions are welcomed. Please open an issue to discuss the changes before opening a PR.

## License

This is Free Software distributed under the MIT license.