//! Combinatorial functions

/// Calculates the binomial coefficient `m` choose `n`.
pub fn binomial(m : uint, n : uint) -> uint {
    if n > m {
        return 0;
    }
    let mut n = n;
    if n > m/2 {
        n = m-n;
    }
    let mut res=1;
    for i in range(m-n+1,m+1) {
        res*=i;
    }
    for i in range(1,n+1) {
        res/=i;
    }
    return res;
}

