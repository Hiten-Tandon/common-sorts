use common_sorts::radix_sort::RadixSort;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

const SIZE: usize = 10_000_000;

fn u8_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u8/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u8> = (0..count).map(|_| rand::random::<u8>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn i8_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i8/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i8> = (0..count).map(|_| rand::random::<i8>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn u16_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u16/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u16> = (0..count).map(|_| rand::random::<u16>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn i16_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i16/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i16> = (0..count).map(|_| rand::random::<i16>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn u32_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u32/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u32> = (0..count).map(|_| rand::random::<u32>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn i32_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i32/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i32> = (0..count).map(|_| rand::random::<i32>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn u64_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u64/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u64> = (0..count).map(|_| rand::random::<u64>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn i64_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i64/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i64> = (0..count).map(|_| rand::random::<i64>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn u128_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u128/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u128> = (0..count).map(|_| rand::random::<u128>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn i128_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i128/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i128> = (0..count).map(|_| rand::random::<i128>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn usize_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("usize/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<usize> = (0..count).map(|_| rand::random::<usize>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn isize_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("isize/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<isize> = (0..count).map(|_| rand::random::<isize>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn bool_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("bool/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<bool> = (0..count).map(|_| rand::random::<bool>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn char_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("char/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<char> = (0..count).map(|_| rand::random::<char>()).collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn string_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= 10000usize {
        c.bench_function(&format!("string/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<String> = (0..1000)
                    .map(|_| {
                        let iter = 0..=rand::thread_rng().gen_range(0..=100);
                        iter.map(|_| rand::random::<char>()).collect()
                    })
                    .collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn f32_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("f32/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<f32> = (0..count)
                    .map(|_| rand::random::<f32>() * rand::random::<i32>() as f32)
                    .collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

fn f64_radix_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("f64/RadixSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<f64> = (0..count)
                    .map(|_| rand::random::<f64>() * rand::random::<i64>() as f64)
                    .collect();
                arr.radix_sort()
            })
        });
        count *= 10;
    }
}

criterion_group!(
    radix_benches,
    u8_radix_sort_benchmark,
    i8_radix_sort_benchmark,
    u16_radix_sort_benchmark,
    i16_radix_sort_benchmark,
    u32_radix_sort_benchmark,
    i32_radix_sort_benchmark,
    u64_radix_sort_benchmark,
    i64_radix_sort_benchmark,
    u128_radix_sort_benchmark,
    i128_radix_sort_benchmark,
    usize_radix_sort_benchmark,
    isize_radix_sort_benchmark,
    f32_radix_sort_benchmark,
    f64_radix_sort_benchmark,
    bool_radix_sort_benchmark,
    char_radix_sort_benchmark,
    string_radix_sort_benchmark,
);

fn u8_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u8/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u8> = (0..count).map(|_| rand::random::<u8>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn i8_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i8/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i8> = (0..count).map(|_| rand::random::<i8>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn u16_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u16/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u16> = (0..count).map(|_| rand::random::<u16>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn i16_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i16/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i16> = (0..count).map(|_| rand::random::<i16>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn u32_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u32/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u32> = (0..count).map(|_| rand::random::<u32>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn i32_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i32/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i32> = (0..count).map(|_| rand::random::<i32>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn u64_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u64/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u64> = (0..count).map(|_| rand::random::<u64>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn i64_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i64/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i64> = (0..count).map(|_| rand::random::<i64>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn u128_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("u128/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<u128> = (0..count).map(|_| rand::random::<u128>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn i128_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("i128/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<i128> = (0..count).map(|_| rand::random::<i128>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn usize_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("usize/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<usize> = (0..count).map(|_| rand::random::<usize>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn isize_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("isize/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<isize> = (0..count).map(|_| rand::random::<isize>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn f32_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("f32/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<f32> = (0..count)
                    .map(|_| rand::random::<f32>() * rand::random::<i32>() as f32)
                    .collect();
                arr.sort_by(|a, b| a.partial_cmp(b).unwrap())
            })
        });
        count *= 10;
    }
}

fn f64_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("f64/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<f64> = (0..count)
                    .map(|_| rand::random::<f64>() * rand::random::<i64>() as f64)
                    .collect();
                arr.sort_by(|a, b| a.partial_cmp(b).unwrap())
            })
        });
        count *= 10;
    }
}

fn bool_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("bool/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<bool> = (0..count).map(|_| rand::random::<bool>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn char_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= SIZE {
        c.bench_function(&format!("char/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<char> = (0..count).map(|_| rand::random::<char>()).collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

fn string_sort_benchmark(c: &mut Criterion) {
    let mut count = 1;
    while count <= 10000usize {
        c.bench_function(&format!("string/TimSort/{count}"), |b| {
            b.iter(|| {
                let mut arr: Vec<String> = (0..1000)
                    .map(|_| {
                        let iter = 0..=rand::thread_rng().gen_range(0..=100);
                        iter.map(|_| rand::random::<char>()).collect()
                    })
                    .collect();
                arr.sort();
            })
        });
        count *= 10;
    }
}

criterion_group!(
    std_sort_benches,
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
    f32_sort_benchmark,
    f64_sort_benchmark,
    bool_sort_benchmark,
    char_sort_benchmark,
    string_sort_benchmark,
);

criterion_main!(radix_benches, std_sort_benches);
