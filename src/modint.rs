//! Number theory

extern crate core;
extern crate num;

use core::num::Zero;
use core::num::One;
use num::Integer;
use core::fmt;
use core::fmt::Show;

/// A modular integer type
pub struct ModInt<T> {
    value : T,
    modulus : T
}

impl<T: Integer> ModInt<T> {
    /// Construct a new modular integer with given value and modulus
    pub fn new(v : T, m : T) -> ModInt<T> {
        let r=v%m;
        if r >= Zero::zero() {
            ModInt {value : r, modulus : m}
        } else {
            ModInt {value : r + m, modulus : m}
        }
    }

    /// Construct a new modular integer with given value and modulus
    ///
    /// Doesn't normalize v to the range [0,m).
    pub fn new_nocheck(v : T, m : T) -> ModInt<T> {
        ModInt {value : v, modulus : m}
    }
}

impl<T: Show> Show for ModInt<T> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulus)
    }
}

impl<T: Clone + Integer> Add<ModInt<T>,ModInt<T>> for ModInt<T> {
    fn add(&self, other : &ModInt<T>) -> ModInt<T> {
        ModInt::new_nocheck((self.value + other.value) % self.modulus, self.modulus.clone())
    }
}

impl<T: Clone + Integer> Sub<ModInt<T>,ModInt<T>> for ModInt<T> {
    fn sub(&self, other : &ModInt<T>) -> ModInt<T> {
        ModInt::new_nocheck((self.value - other.value) % self.modulus, self.modulus.clone())
    }
}

impl<T: Clone + Integer> Mul<ModInt<T>,ModInt<T>> for ModInt<T> {
    fn mul(&self, other : &ModInt<T>) -> ModInt<T> {
        ModInt::new_nocheck((self.value * other.value) % self.modulus, self.modulus.clone())
    }
}

impl<T: Clone + Integer> ModInt<T> {
    /// Calculates `self` to the power `exp`.
    pub fn pow(&self, mut exp : uint) -> ModInt<T> {
        let mut base = ModInt::new(self.value.clone(), self.modulus.clone());
        if exp == 1 {
            base
        } else {
            let mut acc = ModInt::new(One::one(), self.modulus.clone());
            while exp > 0 {
                if (exp & 1) == 1 {
                    acc = acc * base;
                }
                base = base*base;
                exp = exp >> 1;
            }
            acc
        }
    }
}

impl<T: Clone> Clone for ModInt<T> {
    fn clone(&self) -> ModInt<T> {
        ModInt{value: self.value.clone(), modulus: self.modulus.clone()}
    }
}

impl<T: Clone + Integer + Show> Div<ModInt<T>,ModInt<T>> for ModInt<T> {
    fn div(&self, other : &ModInt<T>) -> ModInt<T> {
        let inv = mod_inverse(other.value.clone(), self.modulus.clone());
        match inv {
            Some(i) => ModInt::new(self.value * i % self.modulus, self.modulus.clone()),
            None    => fail!("{} is not a unit (mod {})", other.value, self.modulus)
        }
    }
}

impl<T: PartialEq> PartialEq for ModInt<T> {
    fn eq(&self, other: &ModInt<T>) -> bool {
        self.value == other.value && self.modulus == other.modulus
    }
}

/// For given integers `a` and `b` returns three integers `(x, y, g)` that satisfy ax + by = g and g=gcd(a,b).
pub fn bezout<T: Integer + Clone>(a : T, b : T) -> (T, T, T) {
    if a == Zero::zero() {
        return (Zero::zero(), One::one(), b);
    }
    if b <= a {
        return bezout(b, a);
    }
    let (x, y, g) = bezout(b%a, a.clone());
    //x*(b - [b/a]a) + y*a == g
    //(y - (b/a)*x)*a + x*b == g
    return (y - (b/a)*x, x, g);
}

/// For integers `a` and `m` computes an integer `x` such that ax = 1 (mod m), if such an integer exists.
pub fn mod_inverse<T: Integer + Clone>(a : T, m : T) -> Option<T> {
    let (x,_,g) = bezout(a, m.clone());
    if g == One::one() {
        Some(x%m)
    } else {
        None
    }
}

