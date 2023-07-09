use random_number::random;

pub fn get_par_random(arr: [u32; 8]) -> i32 {
    let mut res: i32 = -1;
    let i: usize = random!(0, 7);
    if arr[i] % 2 == 0 {
        res = arr[i] as i32;
    }
    res
}
