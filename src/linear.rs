//! Linear algebra

use core::num::Zero;
use core::num::One;
use core::fmt::{Show,Formatter,Result};

/// A matrix with elements of type `T`.
#[deriving(Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: uint,
    cols: uint
}

macro_rules! coord(
    ($r:ident, $c:ident) => (self.rows*$r + $c))

impl<T: Clone + Zero> Matrix<T> {
    /// Constructs a new matrix of dimensions `rows`x`cols` filled with zeros.
    pub fn new(rows: uint, cols: uint) -> Matrix<T> {
        Matrix { data: Vec::from_elem(rows*cols, Zero::zero()), rows: rows, cols: cols }
    }

    /// Returns a reference to an element at location (`r`,`c`)
    pub fn get<'a>(&'a self, r: uint, c: uint) -> &'a T {
        if r >= self.rows || c >= self.cols {
            fail!("Trying to get element at {},{} but matrix has size {}x{}", r, c, self.rows, self.cols);
        }
        self.data.get(coord!(r,c))
    }

    /// Returns a mutable reference to an element at location (`r`,`c`)
    pub fn get_mut<'a>(&'a mut self, r: uint, c: uint) -> &'a mut T {
        if r >= self.rows || c >= self.cols {
            fail!("Trying to get element at {},{} but matrix has size {}x{}", r, c, self.rows, self.cols);
        }
        self.data.get_mut(coord!(r,c))
    }
}

impl<T: Show + Zero + Clone> Show for Matrix<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let mut s = String::new();
        for r in range(0,self.rows) {
            for c in range(0,self.cols) {
                s = s.append(format!("{}\t", self.get(r,c)).as_slice());
            }
            s = s.append("\n");
        }
        fmt.pad(s.as_slice())
    }
}

impl<T: Clone + Zero + One> Matrix<T> {
    /// Returns a `size`x`size` identity matrix.
    pub fn identity(size: uint) -> Matrix<T> {
        let mut m = Matrix::new(size,size);
        for i in range(0,size) {
            *m.get_mut(i,i) = One::one();
        }
        m
    }
}

impl<T: Clone + Zero + One + PartialEq + Div<T,T> + Neg<T>> Matrix<T> {
    /// Performs row operations on the matrix, bringing it to row-echelon form, and does the same operations to `augmented`.
    ///
    /// Returns the rank of the matrix.
    pub fn gaussian_elimination(&mut self, augmented: &mut Matrix<T>) -> uint {
        if augmented.rows != self.rows {
            fail!("Trying to perform Gaussian elimination on two matrices with different number of rows.");
        }
        let mut c = 0;
        let mut rank = self.rows;
        for r in range(0,self.rows) {
            let mut rr = self.rows;
            while c < self.cols && rr == self.rows {
                rr=r;
                while rr < self.rows && *self.get(rr,c) == Zero::zero() {
                    rr+=1;
                }
                c+=1;
            }
            c-=1;
            if rr == self.rows {
                rank = r-1;
                break;
            }
            //Found pivot at (rr,c)
            if rr > r {
                //Swap the two rows
                for i in range(0,self.cols) {
                    let tmp = self.get(rr,i).clone();
                    *self.get_mut(rr,i) = self.get(r,i).clone();
                    *self.get_mut(r,i)  = tmp;
                }
                for i in range(0,augmented.cols) {
                    let tmp = augmented.get(rr,i).clone();
                    *augmented.get_mut(rr,i) = augmented.get(r,i).clone();
                    *augmented.get_mut(r,i) = tmp;
                }
            }
            //Pivot is now at (r,c)
            
            let pivot=self.get(r,c).clone();
            for i in range(0,self.cols) {
                *self.get_mut(r,i) = *self.get(r,i)/pivot;
            }
            for i in range(0,augmented.cols) {
                *augmented.get_mut(r,i) = *augmented.get(r,i)/pivot;
            }
            for rr in range(0,self.rows) {
                if rr == r {
                    continue;
                }
                let coeff = -*self.get(rr,c);
                for i in range(0,self.cols) {
                    *self.get_mut(rr,i) = *self.get(rr,i) + coeff * *self.get(r,i);
                }
                for i in range(0,augmented.cols) {
                    *augmented.get_mut(rr,i) = *augmented.get(rr,i) + coeff * *augmented.get(r,i);
                }
            }
        }
        rank
    }

    /// Returns the inverse of the matrix, or None if it does not exist.
    pub fn invert(&self) -> Option<Matrix<T>> {
        if self.rows != self.cols {
            return None;
        }
        let mut id=Matrix::identity(self.rows);
        let mut tmp=self.clone();
        let rank = tmp.gaussian_elimination(&mut id);
        if rank == self.rows { Some(id) } else { None }
    }
}

impl<T: Clone + Zero + Add<T,T>> Add<Matrix<T>,Matrix<T>> for Matrix<T> {
    fn add(&self, other: &Matrix<T>) -> Matrix<T> {
        if self.rows != other.rows || self.cols != other.cols {
            fail!("Trying to add matrices of dimensions {}x{} and {}x{}", self.rows, self.cols, other.rows, other.cols)
        }
        let mut res = Matrix::new(self.rows, self.cols);
        for r in range(0,self.rows) {
            for c in range(0,self.cols) {
                *res.data.get_mut(coord!(r,c)) = *self.data.get(coord!(r,c)) + *other.data.get(coord!(r,c));
            }
        }
        res
    }
}

impl<T: Clone + Zero + Sub<T,T>> Sub<Matrix<T>,Matrix<T>> for Matrix<T> {
    fn sub(&self, other: &Matrix<T>) -> Matrix<T> {
        if self.rows != other.rows || self.cols != other.cols {
            fail!("Trying to add matrices of dimensions {}x{} and {}x{}", self.rows, self.cols, other.rows, other.cols)
        }
        let mut res = Matrix::new(self.rows, self.cols);
        for r in range(0,self.rows) {
            for c in range(0,self.cols) {
                *res.data.get_mut(coord!(r,c)) = *self.data.get(coord!(r,c)) - *other.data.get(coord!(r,c));
            }
        }
        res
    }
}

impl<T: Clone + Zero + Add<T,T> + Mul<T,T>> Mul<Matrix<T>,Matrix<T>> for Matrix<T> {
    fn mul(&self, other: &Matrix<T>) -> Matrix<T> {
        if self.cols != other.rows {
            fail!("Trying to add matrices of dimensions {}x{} and {}x{}", self.rows, self.cols, other.rows, other.cols)
        }
        let mut res: Matrix<T> = Matrix::new(self.rows, other.cols);
        for r in range(0,self.rows) {
            for c in range(0,other.cols) {
                for k in range(0,self.cols) {
                    *res.data.get_mut(coord!(r,c)) = *res.data.get(coord!(r,c)) + *self.data.get(coord!(r,k)) * *other.data.get(coord!(k,c));
                }
            }
        }
        res
    }
}

impl<T: Clone + PartialEq + Zero + One + Add<T,T> + Mul<T,T> + Div<T,T> + Neg<T>> Matrix<T> {
    /// Raises the matrix to the power `exp`. Returns None if this is not possible (matrix is not a square matrix or `exp` is negative and matrix is not invertible).
    pub fn pow(&self, mut exp: int) -> Matrix<T> {
        if self.rows != self.cols { fail!("Tried to take a power of a non-square matrix!"); }
        if exp < 0 {
            let inv=self.invert();
            match inv {
                Some(inv) => return inv.pow(-exp),
                _         => fail!("Tried to take a negative power of a non-invertible matrix!")
            }
        }

        let mut ret = Matrix::identity(self.rows);
        let mut base : Matrix<T> = self.clone();
        while exp > 0 {
            if (exp&1) == 1 {
                ret = ret*base;
            }
            base = base*base;
            exp = exp >> 1;
        }
        ret
    }
}

