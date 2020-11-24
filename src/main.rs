extern crate rayon;
extern crate rand;
extern crate mersenne_twister;

use std::mem::transmute_copy;
use rand::{SeedableRng, Rng};
use mersenne_twister::MersenneTwister;
use rayon::slice::ParallelSliceMut;
use std::time::Instant;

#[derive(Debug)]
struct Data {
    number: u64,
    name: String,
}

fn to_rand_char(seed: u8) -> char {
    static CHARS: [char; 64] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-'];
    
    return CHARS[(seed % 64) as usize];
}

fn to_rand_string(seed: u64) -> String
{
    let arr: [u8; 8] = unsafe { transmute_copy(&seed) };
    let len = (arr[0] % 4 + 4) as usize;
    let mut result = String::with_capacity(len);
    for i in 0..len {
        result.push(to_rand_char(arr[i+1]));
    }
    return result;
}

fn prepare(size: usize) -> Vec<Data>
{
    let mut rng1: MersenneTwister = SeedableRng::from_seed(0x12345);
    let mut rng2: MersenneTwister = SeedableRng::from_seed(0x54321);
    
    return rng1.gen_iter::<u64>().zip(rng2.gen_iter::<u64>())
               .take(size)
               .map(|(x, y)| Data { number:x, name:to_rand_string(y) } )
               .collect::<Vec<Data>>();
}

fn main() {
    let mut v = prepare(100_000_000);

    let ins = Instant::now();
    v.par_sort_by( |x, y| x.name.cmp(&y.name) );
    let d = ins.elapsed();
    println!("{}.{:09} sec used", d.as_secs(), d.subsec_nanos());

    for d in v.iter().take(10) {
        println!("{:?}", d);
    }
}
