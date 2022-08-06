use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

const SIZE : usize = 10_000_000;

fn u8_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u8> = black_box((0..SIZE).map(|_| rand::random::<u8>()).collect());
    c.bench_function(&format!("Sorting Vec<u8> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn i8_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i8> = black_box((0..SIZE).map(|_| rand::random::<i8>()).collect());
    c.bench_function(&format!("Sorting Vec<i8> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn u16_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u16> = black_box((0..SIZE).map(|_| rand::random::<u16>()).collect());
    c.bench_function(&format!("Sorting Vec<u16> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn i16_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i16> = black_box((0..SIZE).map(|_| rand::random::<i16>()).collect());
    c.bench_function(&format!("Sorting Vec<i16> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn u32_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u32> = black_box((0..SIZE).map(|_| rand::random::<u32>()).collect());
    c.bench_function(&format!("Sorting Vec<u32> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn i32_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i32> = black_box((0..SIZE).map(|_| rand::random::<i32>()).collect());
    c.bench_function(&format!("Sorting Vec<i32> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn u64_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u64> = black_box((0..SIZE).map(|_| rand::random::<u64>()).collect());
    c.bench_function(&format!("Sorting Vec<u64> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn i64_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i64> = black_box((0..SIZE).map(|_| rand::random::<i64>()).collect());
    c.bench_function(&format!("Sorting Vec<i64> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn u128_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<u128> = black_box((0..SIZE).map(|_| rand::random::<u128>()).collect());
    c.bench_function(&format!("Sorting Vec<u128> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn i128_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<i128> = black_box((0..SIZE).map(|_| rand::random::<i128>()).collect());
    c.bench_function(&format!("Sorting Vec<i128> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn usize_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<usize> = black_box((0..SIZE).map(|_| rand::random::<usize>()).collect());
    c.bench_function(&format!("Sorting Vec<usize> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn isize_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<isize> = black_box((0..SIZE).map(|_| rand::random::<isize>()).collect());
    c.bench_function(&format!("Sorting Vec<isize> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn bool_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<bool> = black_box((0..SIZE).map(|_| rand::random::<bool>()).collect());
    c.bench_function(&format!("Sorting Vec<bool> of size {SIZE}"), |b| b.iter(|| arr.sort()));
}

fn char_sort_benchmark(c : &mut Criterion){
    let mut arr : Vec<char> = black_box((0..SIZE).map(|_| rand::random::<char>()).collect());
    c.bench_function(&format!("Sorting Vec<char> of size {SIZE}"), |b| b.iter(|| arr.sort()));
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
    );
criterion_main!(benches);

