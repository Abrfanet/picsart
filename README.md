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
```

### Output Files
* PNG: Supports transparency (alpha channel) also reduce additionals meta and overlaps on PNG
* WEBP: Slight quality, General web images and modern formats
* AVIF: Maximum compression with good quality (Ultra encode method like 256 codec)

