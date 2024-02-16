#![feature(is_sorted)]
#![feature(exclusive_range_pattern)]

trait InsertionSort<T> {
    fn insertion_sort(&mut self);
    fn insertion_sort_by(&mut self, by: FnMustSwap<T>);
}

type FnMustSwap<T> = fn(a: &T, b: &T) -> bool;

impl<T> InsertionSort<T> for Vec<T> where T: PartialOrd {
    fn insertion_sort(&mut self) {
        self.insertion_sort_by(|a, b| a > b)
    }

    fn insertion_sort_by(&mut self, by: FnMustSwap<T>) {
        let mut i = 1;
        while i < self.len() {
            let mut j = i;
            while j != 0 {
                if by(&self[j - 1], &self[j]) {
                    self.swap(j, j - 1);
                }
                j = j - 1;
            }
            i = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::InsertionSort;

    #[test]
    fn test_insertion_sort() {
        let mut input: Vec<u32> = vec![];
        input.reverse();
        input.insertion_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![1];
        input.reverse();
        input.insertion_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![2, 1];
        input.reverse();
        input.insertion_sort();
        assert!(input.is_sorted());

        let mut input: Vec<u32> = vec![0, 1, 2, 3];
        input.reverse();
        input.insertion_sort();
        assert!(input.is_sorted());
    }
}
