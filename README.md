SNIP
======

Capture/take a screenshot.

## Requirements

- Rust 1.50+

## Usage
```toml
[dependencies]
snip = { git = "https://github.com/marirs/snip-rs" }
```
and then

```rust
use snip::screen_capture;

fn main() {
	let s = screen_capture(0).unwrap();

	println!("{} x {}", s.width(), s.height());

	image::save_buffer(
        &Path::new("test.png"),
		s.as_slice(), 
        s.width() as u32, 
        s.height() as u32, 
        image::ColorType::Rgba8
    )
	.unwrap();
}
```


---
