#![feature(is_sorted)]
#![feature(exclusive_range_pattern)]

pub type FnMustSwap<T> = fn(a: &T, b: &T) -> bool;

pub fn asc<T>(a: &T, b: &T) -> bool where T: PartialOrd {
    a > b
}

pub fn desc<T>(a: &T, b: &T) -> bool where T: PartialOrd {
    a < b
}

pub trait SelectionSort<T> {
    fn selection_sort(&mut self);
    fn selection_sort_by(&mut self, by: FnMustSwap<T>);
}

impl<T> SelectionSort<T> for Vec<T> where T: PartialOrd {
    fn selection_sort(&mut self) {
        self.selection_sort_by(asc)
    }

    fn selection_sort_by(&mut self, by: FnMustSwap<T>) {
        let size = self.len();
        for i in 0..size {
            let mut mm_index = i;
            for j in (i + 1)..size {
                if by(&self[mm_index], &self[j]) {
                    mm_index = j;
                }
            }
            self.swap(i, mm_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SelectionSort, desc};

    #[test]
    fn test_selection_sort() {
        let mut input: Vec<u32> = vec![];
        input.reverse();
        input.selection_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![1];
        input.reverse();
        input.selection_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![2, 1];
        input.reverse();
        input.selection_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![0, 1, 2, 3];
        input.reverse();
        input.selection_sort();
        println!("{:?}", input);
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![1, 1, 2, 3];
        input.reverse();
        input.selection_sort();
        println!("{:?}", input);
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![1, 1, 2, 3];
        input.selection_sort_by(desc);
        input.reverse();
        println!("{:?}", input);
        assert!(input.is_sorted());
    }
}