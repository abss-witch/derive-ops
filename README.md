# impl-ops
Custom derive macros to implement component wise opperations.
```rust
use impl_ops::CmpOps;
#[derive(CmpOps)]
pub struct Point{
    a: f32,
    b: i8,
    c: i16
}
```

Alternatively you can include individual opperations.
```rust
use impl_ops::CmpOps;
#[derive(CmpRemAssign, CmpMul)]
pub struct Point{
    a: f32,
    b: i8,
    c: i16
}

```

Includes derives for add, add assign, sub, sub assign, mul, mul assign, div, div assign, rem, rem assign and neg