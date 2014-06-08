//! Modular arithmetic

use core::num::Zero;
use core::num::One;
use num::Integer;



/// For given integers `a` and `b` returns three integers `(x, y, g)` that satisfy ax + by = g and g=gcd(a,b).
pub fn bezout<T: Integer>(a: T, b: T) -> (T, T, T) {
    if a == Zero::zero() {
        return (Zero::zero(), One::one(), b);
    }
    if b <= a {
        return bezout(b, a);
    }
    let d=b/a;
    let (x, y, g) = bezout(b%a, a);
    return (y - d*x, x, g);
}

/// For integers `a` and `m` computes an integer `x` such that ax = 1 (mod m), if such an integer exists.
pub fn invert_mod<T: Integer>(a: T, m: T) -> Option<T> {
    let (x,_,g) = bezout(a,m);
    if g == One::one() {
        Some(x)
    } else {
        None
    }
}

