extern crate core;

use core::num::Zero;

pub struct FenwickTree<T> {
    array : Vec<T>
}

impl<T : Zero + Clone + Add<T,T>> FenwickTree<T> {
    pub fn new(size : uint) -> FenwickTree<T> {
        FenwickTree {array : Vec::from_elem(size+1, Zero::zero())}
    }

    pub fn len(&self) -> uint {
        self.array.len() - 1
    }

    pub fn increment(&mut self, index : uint, amount : T) {
        let mut i = index+1;
        while i <= self.len() {
            let newamount = *self.array.get(i)+amount;
            *self.array.get_mut(i) = newamount;
            i+=i&(-i);
        }
    }

    pub fn sum_to(&self, index : uint) -> T {
        let mut i = if index > self.len() { self.len() } else { index };
        let mut total : T = Zero::zero();
        while i > 0 {
            total=total + *self.array.get(i);
            i-=i&(-i);
        }
        return total;
    }

    pub fn sum(&self) -> T {
        self.sum_to(self.array.len())
    }
}

impl<T: Zero + Clone + Add<T,T> + Sub<T,T>> FenwickTree<T> {
    pub fn sum_from_to(&self, start : uint, end : uint) -> T {
        self.sum_to(end) - self.sum_to(start)
    }

    pub fn get(&self, index : uint) -> T {
        if index >= self.len() {
            Zero::zero()
        } else {
            self.sum_from_to(index, index+1)
        }
    }

    pub fn set(&mut self, index : uint, value : T) {
        let inc = value - self.get(index);
        self.increment(index, inc);
    }
}

