# csselection_sort

Selection sort algorithm implementation over a vector.

Use this create to add a ``selection_sort`` and ``selection_sort_by`` methods
to Vec data type.

```rust
    use csselection_sort::SelectionSort; 
    
    let mut input: Vec<u32> = vec![2, 1];

    // this will sort items in ascending order 
    // using selection sort algorithm
    input.selection_sort();
    // input is now [1,2]
```

Use ``selection_sort_by`` to customize how items are compared. 
As second argument pass a function with the form: 
```rust 
fn(a: &T, b: &T) -> bool;
```
This function must return ``true`` when ``a`` must be most appear at the left of ``b``.
For example, to sort a list of number in descending order ``by`` parameter can be: 
```rust
fn desc(a: u32, b: u32) -> bool {
    a < b
}
```
This example return ``true`` when ``a=1`` and ``b==5`` in order to 
ensure that ``5`` is located at the left o ``1``.

For convenience a ``asc`` and ``desc`` functions are provided. 

```rust
    use csselection_sort::{SelectionSort, des}; 
    
    let mut input: Vec<u32> = vec![1, 2];

    // this will sort items in descending order 
    // using selection sort algorithm
    input.selection_sort_by(desc);

    // input is now [2,1]
```

To call ``selection_sort`` is equivalent to call ``selection_sort_by(asc)``.**