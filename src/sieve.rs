//! Sieve of Erastothenes

use core::iter::range_step;
use collections::dlist::DList;
use collections::deque::Deque;

/// A prime number sieve suitable for factoring.
pub struct Sieve {
    sieve: Vec<uint>
}

impl Sieve {
    /// Sieves primes up to `n`. Internally stores the largest prime factor of every number.
    pub fn sieve_to(n: uint) -> Sieve {
        let mut s = Sieve {sieve: Vec::from_elem(n+1, 0u)};
        for p in range(2,n+1) {
            if *s.sieve.get(p) == 0 {
                for i in range_step(p, n+1, p) {
                    *s.sieve.get_mut(i) = p;
                }
            }
        }
        return s;
    }

    /// Sieves primes up to `n`. Returns the sieve as well as a list of primes.
    pub fn sieve_to_list(n: uint) -> (Sieve, DList<uint>) {
        let mut s = Sieve {sieve: Vec::from_elem(n+1,0u)};
        let mut primes = DList::new();
        for p in range(2,n+1) {
            if *s.sieve.get(p) == 0 {
                primes.push_back(p);
                for i in range_step(p, n+1, p) {
                    *s.sieve.get_mut(i) = p;
                }
            }
        }
        return (s, primes);
    }

    /// A wrapper to sieve_to.
    pub fn new(n: uint) -> Sieve {
        Sieve::sieve_to(n)
    }

    /// Factors n. Returns a list of (p,k) pairs for every prime power p^k dividing `n`. For `n` = 1 returns the empty list. Fails when `n` is too large or 0.
    pub fn factor(&self, n: uint) -> DList<(uint,uint)> {
        if n >= self.sieve.len() {
            fail!("Trying to factor {} while the size of the sieve is {}", n, self.sieve.len() - 1);
        }
        if n == 0 {
            fail!("Trying to factor 0.");
        }
        let mut n = n;
        let mut res: DList<(uint,uint)> = DList::new();
        while n > 1 {
            let p = *self.sieve.get(n);
            let mut k = 0;
            while n % p == 0 {
                n /= p;
                k+=1;
            }
            res.push_front((p,k));
        }
        return res;
    }
    
    /// Finds divisors of `n`.
    pub fn divisors(&self, mut n: uint) -> DList<uint> {
        if n >= self.sieve.len() {
            fail!("Trying to find divisors of {} while the size of the sieve is {}", n, self.sieve.len() - 1);
        }
        if n == 0 {
            fail!("Trying to find divisors of 0.");
        }
        if n == 1 {
            let mut l = DList::new();
            l.push_front(1);
            return l;
        }
        let p = *self.sieve.get(n);
        let mut k = 0;
        while n % p == 0 {
            n/=p;
            k+=1;
        }
        let l = self.divisors(n);
        let mut res = DList::new();
        for d in l.iter() {
            let mut pp = 1;
            for _ in range(0,k+1) {
                res.push_back(d*pp);
                pp*=p;
            }
        }
        return res;
    }
}


