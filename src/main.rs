use derive_ops::{CmpAdd, CmpAddAssign, CmpDiv, CmpDivAssign, CmpMul, CmpMulAssign, CmpNeg, CmpOps, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign};
#[derive(PartialEq, Eq, Debug)]
#[derive(CmpAdd, CmpSub, CmpMul, CmpDiv, CmpRem, CmpAddAssign, CmpSubAssign, CmpMulAssign, CmpDivAssign, CmpRemAssign, CmpNeg)]
struct Foo{
    x: i16,
    b: i32
}

#[derive(CmpOps)]
struct Bar{
    a: i16,
    b: f64
}
fn main(){
    
}
#[test]
fn add(){
    assert_eq!(Foo{ x: 1, b: 9 } + Foo{ x: -4, b: 0 }, Foo{ x: -3,  b: 9 });
}
#[test]
fn sub(){
    assert_eq!(Foo{ x: 1, b: 9 } - Foo{ x: -4, b: 0 }, Foo{ x: 5,  b: 9 });
}
#[test]
fn mul(){
    assert_eq!(Foo{ x: 1, b: 9 } * Foo{ x: -4, b: 0 }, Foo{ x: -4,  b: 0 });
}
#[test]
fn div(){
    assert_eq!(Foo{ x: 6, b: 9 } / Foo{ x: 3, b: 1 }, Foo{ x: 2,  b: 9 });
}
#[test]
fn rem(){
    assert_eq!(Foo{ x: 3, b: 2 } % Foo{ x: 2, b: 1 }, Foo{ x: 1, b: 0 });
}
#[test]
fn neg(){
    assert_eq!(-Foo{ x: 2, b: 1 }, Foo{ x: -2, b: -1 });
}