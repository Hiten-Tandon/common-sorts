pub trait RadixSort {
    fn radix_sort(&mut self);
}

const RADIX : usize = 256;
//Radix sort can only sort unsigned integers
fn u8_sorting_routine(arr : &mut [u8], len : usize,mut max_ele : u8){ 
    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<u8> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn u16_sorting_routine(arr : &mut [u16], len : usize, mut max_ele : u16) {
    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<u16> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn u32_sorting_routine(arr : &mut [u32], len : usize, mut max_ele : u32) {

    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<u32> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn u64_sorting_routine(arr : &mut [u64], len : usize, mut max_ele : u64) {
    
    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<u64> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn u128_sorting_routine(arr : &mut [u128], len : usize, mut max_ele : u128){

    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<u128> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn usize_sorting_routine(arr : &mut[usize], len : usize, mut max_ele : usize){

    let mut exp : u8 = 0;
   
    while max_ele > 0 {
        let mut count : [usize; RADIX] = [0; RADIX];
        let mut temp  : Vec<usize> = vec![0; len];
        arr.iter()
            .for_each(|&x| count[(x >> (exp << 3)) as usize & (RADIX - 1)] += 1);//counting each digit

        count [0] = count[0].wrapping_sub(1);//reducing count[0] by 1 so that we get 0-based index after prefix-sum or rolling sum of the array

        for i in 1..RADIX {
            count[i] += count[i - 1];//performing the prefix-sum or rolling sum of the array
        }

        (0..len).rev().for_each(|i| {
            let idx = &mut count[(arr[i] >> (exp << 3)) as usize & (RADIX - 1)];
            temp[*idx] = arr[i];
            if *idx > 0 {
                *idx -= 1
            };
        });

        arr.copy_from_slice(&temp);
        max_ele = max_ele.wrapping_shr(8);
        exp += 1;
    }
}

fn char_sorting_routine(arr : &mut[char], len : usize, mut max_ele : char){
    let max_ele : u32 = max_ele as u32;
    let mut arr_cpy = arr.iter().map(|&c| c as u32).collect::<Vec<u32>>();
    u32_sorting_routine(&mut arr_cpy, len, max_ele);
    arr.clone_from_slice(&arr_cpy.into_iter().map(|c| char::from_u32(c).unwrap()).collect::<Vec<char>>());
}


