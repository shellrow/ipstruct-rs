[crates-badge]: https://img.shields.io/crates/v/ipstruct.svg
[crates-url]: https://crates.io/crates/ipstruct
[license-badge]: https://img.shields.io/crates/l/ipstruct.svg
[examples-url]: https://github.com/shellrow/ipstruct-rs/tree/main/examples
[doc-url]: https://docs.rs/ipstruct/latest/ipstruct
[ipstruct-github-url]: https://github.com/shellrow/ipstruct-rs

# ipstruct [![Crates.io][crates-badge]][crates-url] ![License][license-badge]
Rust client library for accessing the ipstruct.com Web API. With this library, you can easily retrieve information related to public IP addresses. It supports both IPv4 and IPv6, and provides asynchronous and synchronous request capabilities.

## Key Features
- Retrieve information related to public IP addresses.
    - Public IP Address
    - Country
    - ASN
    - AS Name (ISP or network operator)
- Supports both IPv4 and IPv6.
- Provides asynchronous and synchronous request capabilities.
- Proxy support with customizable options (default or user-specified).

## Usage
Add `ipstruct` to your dependencies  
```toml:Cargo.toml
[dependencies]
ipstruct = "0.2.0"
```

For more details, see [examples][examples-url] or [doc][doc-url].
