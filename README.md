# endify

Effortlessly convert structs between endianess-formats.

## Usage
```rust
use endify::Endify;

#[repr(C)]
#[derive(Debug, Endify)]
struct MyStruct {
    a: u32,
    b: u16,
    c: u8,
}

fn main() {
    // stored on disk as `little-endian` format.
    let my_struct = read_from_disk();

    // convert all fields to `native-endian` format.
    let my_struct_native = my_struct.from_le();
}
```
