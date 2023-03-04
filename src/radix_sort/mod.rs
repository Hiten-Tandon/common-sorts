//!Radix Sort is a sorting algorithm, particulalry useful when sorting, large sets of data.
//!It is an extension of the counting sort algorithm, and is a stable sorting algorithm.
//!Radix sort has many implementations, that vary based on the type[ LSD / MSD ], Radix of the sort
//!and the implementation method, i.e. counting sort or bucket sort.
//!The one used here is LSD base - 255 radix sort, using counting sort.
mod internals;
mod lib;
mod slice_implementations;
mod vec_implementations;

pub trait RadixSort {
    fn radix_sort(&mut self);
}
