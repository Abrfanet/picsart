# Picsart
Picsart is a Rust library that scrapes, optimize and merge an income photos. Use self responsibly.
 
## Features
- Query for exported image
- Collect all options from cli and main class
- Collect unlimited images
- Multiple themes for exported image
- Totally async

## Get started
### Add picsart to your Cargo.toml 🦀
```toml
picsart = "^0.0.1"
```
Supported features are:
- `no-log`: disable logging
- `rustls`: use rustls for reqwest


### Picsart Guide
```
    use picsart::PicsArt;
