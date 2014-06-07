extern crate core;
extern crate num;

use core::num::Zero;
use core::num::One;
use num::Integer;
use core::fmt;
use core::fmt::Show;

pub struct ModInt<T> {
    value : T,
    modulus : T
}

impl<T: Integer> ModInt<T> {
    fn new(v : T, m : T) -> ModInt<T> {
        let r=v%m;
        if r >= Zero::zero() {
            ModInt {value : r, modulus : m}
        } else {
            ModInt {value : r + m, modulus : m}
        }
    }
    fn new_nocheck(v : T, m : T) -> ModInt<T> {
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

impl<T: Clone + Integer + Show> Div<ModInt<T>,ModInt<T>> for ModInt<T> {
    fn div(&self, other : &ModInt<T>) -> ModInt<T> {
        let inv = mod_inverse(other.value.clone(), self.modulus.clone());
        match inv {
            Some(i) => ModInt::new_nocheck(self.value * i % self.modulus, self.modulus.clone()),
            None    => fail!("{} is not a unit (mod {})", other.value, self.modulus)
        }
    }
}

fn bezout<T: Integer + Clone>(a : T, b : T) -> (T, T, T) {
    if a == Zero::zero() {
        return (Zero::zero(), One::one(), b);
    }
    if b <= a {
        return bezout(b, a);
    }
    let (x, y, g) = bezout(b%a, a.clone());
    return (y - (b/a)*x, x, g);
}

fn mod_inverse<T: Integer + Clone>(a : T, b : T) -> Option<T> {
    let (x,_,g) = bezout(a, b);
    if g == One::one() {
        Some(x)
    } else {
        None
    }
}

