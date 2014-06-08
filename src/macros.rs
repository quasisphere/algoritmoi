//! Various macros

/// Generates an integer type `Name` where computations are done modulo `m`
#[macro_export]
macro_rules! define_modulo_ring(
($Name:ident, $m:expr) => (
#[deriving(PartialEq,Clone)]
struct $Name {
    value: i64
}
impl $Name {
    fn new(v: i64) -> $Name {
        let x=v%$m;
        if x < 0 {
            $Name { value: x + $m }
        } else {
            $Name { value: x }
        }
    }
}
impl core::fmt::Show for $Name {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{} (mod {})", self.value, $m)
    }
}
impl Add<$Name,$Name> for $Name {
    fn add(&self, other: &$Name) -> $Name {
        $Name { value: (self.value + other.value)%$m }
    }
}
impl Sub<$Name,$Name> for $Name {
    fn sub(&self, other: &$Name) -> $Name {
        $Name { value: (self.value - other.value)%$m }
    }
}
impl Mul<$Name,$Name> for $Name {
    fn mul(&self, other: &$Name) -> $Name {
        $Name { value: (self.value*other.value)%$m }
    }
}
impl Div<$Name,$Name> for $Name {
    fn div(&self, other: &$Name) -> $Name {
        let inv=algoritmoi::modint::invert_mod(other.value, $m);
        match inv {
            Some(i) => $Name::new(self.value*i),
            _ => fail!("Trying to divide by {} modulo {}", other.value, $m)
        }
    }
}
impl core::num::One for $Name {
    fn one() -> $Name {
        $Name { value: core::num::One::one() }
    }
}
impl core::num::Zero for $Name {
    fn zero() -> $Name {
        $Name { value: core::num::Zero::zero() }
    }
    fn is_zero(&self) -> bool {
        self.value%$m == 0
    }
}
impl FromPrimitive for $Name {
    fn from_i64(n: i64) -> Option<$Name> {
        Some($Name { value: n%$m })
    }
    fn from_u64(n: u64) -> Option<$Name> {
        Some($Name { value: (n%$m) as i64 })
    }
}
)
)

