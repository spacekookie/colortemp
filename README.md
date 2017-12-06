# colortemp.rs

Simple functions to calculate color temperatures and RGB values.

```rust
extern crate colortemp;

let mut rgb = colortemp::temp_to_rgb(2000);
println!("{:?}", rgb);
```