# search-sort

Implementation of few searching and sorting algorithms. This crate is currently
WIP, and supports only few of them.

Searching algorithms to be implemented:

- [x] linear search
- [x] binary search
- [x] jump search
- [x] exponential search

Sorting algorithms to be implemented:

- [x] bubble sort
- [x] quick sort
  - [ ] parallel quick sort
- [ ] insertion sort
- [ ] merge sort
- [ ] heap sort
- [ ] radix sort

## Quick example

Add this to your `Cargo.toml` file:

```toml
[dependencies]
search-sort = "0.2"
```

This code sorts the `slice` and searches for elements in it:

```rust
use search_sort::{search, sort};

let mut slice = [5, 1, 91, -45, 11, 5];
sort::quick(&mut slice);
assert_eq!(slice, [-45, 1, 5, 5, 11, 91]);

assert_eq!(Some(2), search::binary_first(&slice, &5));
assert_eq!(None, search::binary_first(&slice, &42));
```

## License

This code is released under the
[MIT license](https://opensource.org/licenses/MIT). See the
[LICENSE.md](LICENSE.md) file.
