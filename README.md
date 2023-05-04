

# Roxy

Simple proxy auto formatter for Rust.

<img src="https://n.opnxng.com/pic/orig/media%2FErdsQfpW4AAV6t4.jpg" style="border-radius:10px">

___

# Installation

To start using Roxy add dependency in your `Cargo.toml`:

```toml
[dependencies]
roxy = { version = "0.0.1", git = "http://github.com/kyoukisu/roxy" }
```

# Example


Basic example:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>>{
    // doesn't matter if login:password or ip:port go first
    let proxy = roxy::new("http://login:password@example.com:80")?;
    println!("{}",proxy.lpip());
    println!("{}",proxy.iplp());

    // it can even understand this:
    let webshare_proxy = roxy::new("example.com:80:login:password")?;
    println!("{}",webshare_proxy.lpip());
    println!("{}",webshare_proxy.iplp());

    Ok(())
}
```