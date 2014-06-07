//! A Fenwick tree implementation

#![allow(unsigned_negate)]

use core::num::Zero;

/// A Fenwick tree with elements of type `T`.
pub struct FenwickTree<T> {
    array : Vec<T>
}

impl<T : Zero + Clone + Add<T,T>> FenwickTree<T> {
    /// Constructs a new Fenwick tree of given `size`, with elements initialized to 0.
    /// The elements will be indexed from 0 to `size`-1.
    pub fn new(size : uint) -> FenwickTree<T> {
        FenwickTree {array : Vec::from_elem(size+1, Zero::zero())}
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> uint {
        self.array.len() - 1
    }

    /// Increments the element at `index` by `amount`.
    pub fn increment(&mut self, index : uint, amount : T) {
        let mut i = index+1;
        while i <= self.len() {
            let newamount = *self.array.get(i)+amount;
            *self.array.get_mut(i) = newamount;
            i+=i&(-i);
        }
    }

    /// Returns the sum of elements from 0 to index-1.
    pub fn sum_to(&self, index : uint) -> T {
        let mut i = if index > self.len() { self.len() } else { index };
        let mut total : T = Zero::zero();
        while i > 0 {
            total=total + *self.array.get(i);
            i-=i&(-i);
        }
        return total;
    }

    /// Returns the sum of all of the elements in the tree.
    pub fn sum(&self) -> T {
        self.sum_to(self.array.len())
    }
}

impl<T: Zero + Clone + Add<T,T> + Sub<T,T>> FenwickTree<T> {
    /// Returns the sum of the elements with indices in the range [`start`,`end`).
    pub fn sum_from_to(&self, start : uint, end : uint) -> T {
        self.sum_to(end) - self.sum_to(start)
    }

    /// Returns the element at `index`.
    pub fn get(&self, index : uint) -> T {
        if index >= self.len() {
            Zero::zero()
        } else {
            self.sum_from_to(index, index+1)
        }
    }

    /// Sets the element at `index` to `value`.
    pub fn set(&mut self, index : uint, value : T) {
        let inc = value - self.get(index);
        self.increment(index, inc);
    }
}

