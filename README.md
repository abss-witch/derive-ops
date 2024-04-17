Custom derive macros to implement component wise opperations.
```rust
use derive_cmp_ops::CmpOps;
#[derive(CmpOps)]
struct Point{
    a: f32,
    b: i8,
    c: i16
}
```
Alternatively you can include individual opperations.
```rust
use derive_cmp_ops::{CmpRemAssign, CmpMul};
#[derive(CmpRemAssign, CmpMul)]
struct Point{
    a: f32,
    b: i8,
    c: i16
}
```
Includes derives for add, add assign, sub, sub assign, mul, mul assign, div, div assign, rem, rem assign and neg