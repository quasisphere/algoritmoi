//! Functions related to Möbius inversion

extern crate core;
extern crate num;

use num::Integer;
use core::num::One;
use core::num::Zero;
use core::iter::range_step;

fn isqrt(n: uint) -> uint {
    let nf=n.to_f64().unwrap();
    let r=nf.sqrt();
    let mut ri: uint=FromPrimitive::from_f64(r).unwrap();
    while ri*ri <= n {
        ri=ri+1u;
    }
    while ri*ri > n {
        ri=ri-1u;
    }
    ri
}

/// Calculates $f(n)$, when given the sums $g(m)=\sum_{k=1}^m f(\left\lfloor \frac{n}{k} \right\rfloor)$.
pub fn moebius_inversion<T: Integer + Clone + FromPrimitive>(n: uint, g: |&T|: -> T) -> T {
    let sqrtn = isqrt(n);
    let mut lowarray: Vec<T> = Vec::from_elem(sqrtn+1, Zero::zero());
    let mut higharray: Vec<T> = Vec::from_elem(sqrtn+1, Zero::zero());

    *lowarray.get_mut(1) = g(&One::one());
    for m in range_step(2,sqrtn+1,1) {
        *lowarray.get_mut(m) = g(&FromPrimitive::from_uint(m).unwrap()) - g(&FromPrimitive::from_uint(m/2).unwrap());
        let mut k=3;
        while k <= m {
            let mut nextk = m/(m/k) + 1;
            nextk|=1;
            let tmp: T = FromPrimitive::from_uint((nextk - k)/2).unwrap();
            *lowarray.get_mut(m)=*lowarray.get(m) - tmp * *lowarray.get(m/k);
            k=nextk;
        }
    }

    let top: int = if sqrtn&1 == 0 { (sqrtn-1) as int } else { sqrtn as int };
    for md in range_step(top, 0, -2) {
        let md = md as uint;
        let m = n/md;
        *higharray.get_mut(md) = g(&FromPrimitive::from_uint(m).unwrap()) - g(&FromPrimitive::from_uint(m/2).unwrap());
        let mut k=3;
        while k*md <= sqrtn {
            let mut nextk = m/(m/k) + 1;
            nextk|=1;
            let tmp: T = FromPrimitive::from_uint((nextk - k)/2).unwrap();
            *higharray.get_mut(md)=*higharray.get(md) - tmp * *higharray.get(k*md);
            k=nextk;
        }
        while k <= m {
            let mut nextk = m/(m/k) + 1;
            nextk|=1;
            let tmp: T = FromPrimitive::from_uint((nextk - k)/2).unwrap();
            *higharray.get_mut(md)=*higharray.get(md) - tmp * *lowarray.get(m/k);
            k=nextk;
        }
    }

    let res: T = higharray.get(1).clone();

    return res;
}

