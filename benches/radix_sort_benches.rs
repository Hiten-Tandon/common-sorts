use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use rand::Rng;
use common_sorts::radix_sort::RadixSort;

const SIZE : usize = 10_000_000;

fn u8_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u8> = ((0..SIZE).map(|_| rand::random::<u8>()).collect());
    c.bench_function(&format!("Sorting Vec<u8> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn i8_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i8> = ((0..SIZE).map(|_| rand::random::<i8>()).collect());
    c.bench_function(&format!("Sorting Vec<i8> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn u16_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u16> = ((0..SIZE).map(|_| rand::random::<u16>()).collect());
    c.bench_function(&format!("Sorting Vec<u16> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn i16_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i16> = ((0..SIZE).map(|_| rand::random::<i16>()).collect());
    c.bench_function(&format!("Sorting Vec<i16> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn u32_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u32> = ((0..SIZE).map(|_| rand::random::<u32>()).collect());
    c.bench_function(&format!("Sorting Vec<u32> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn i32_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i32> = ((0..SIZE).map(|_| rand::random::<i32>()).collect());
    c.bench_function(&format!("Sorting Vec<i32> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn u64_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u64> = ((0..SIZE).map(|_| rand::random::<u64>()).collect());
    c.bench_function(&format!("Sorting Vec<u64> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn i64_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i64> = ((0..SIZE).map(|_| rand::random::<i64>()).collect());
    c.bench_function(&format!("Sorting Vec<i64> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn u128_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u128> = ((0..SIZE).map(|_| rand::random::<u128>()).collect());
    c.bench_function(&format!("Sorting Vec<u128> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn i128_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i128> = ((0..SIZE).map(|_| rand::random::<i128>()).collect());
    c.bench_function(&format!("Sorting Vec<i128> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn usize_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<usize> = ((0..SIZE).map(|_| rand::random::<usize>()).collect());
    c.bench_function(&format!("Sorting Vec<usize> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn isize_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<isize> = ((0..SIZE).map(|_| rand::random::<isize>()).collect());
    c.bench_function(&format!("Sorting Vec<isize> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn bool_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<bool> = ((0..SIZE).map(|_| rand::random::<bool>()).collect());
    c.bench_function(&format!("Sorting Vec<bool> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

fn char_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<char> = ((0..SIZE).map(|_| rand::random::<char>()).collect());
    c.bench_function(&format!("Sorting Vec<char> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}
fn string_sort_benchmark(c : &mut Criterion){
        let mut arr : Vec<String> = (0..100_000).map(|_| {
            let iter = 0..=rand::thread_rng().gen_range(0..=100);
            iter.map(|_|rand::random::<char>()).collect()
        }).collect();
    c.bench_function(&format!("Sorting Vec<String> of size {SIZE}"), |b| b.iter(|| arr.radix_sort()));
}

criterion_group!(
    benches, 
    u8_sort_benchmark, 
    i8_sort_benchmark, 
    u16_sort_benchmark,
    i16_sort_benchmark,
    u32_sort_benchmark,
    i32_sort_benchmark,
    u64_sort_benchmark,
    i64_sort_benchmark,
    u128_sort_benchmark,
    i128_sort_benchmark,
    usize_sort_benchmark,
    isize_sort_benchmark,
    bool_sort_benchmark, 
    char_sort_benchmark,
    string_sort_benchmark,
    );
criterion_main!(benches);

