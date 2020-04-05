fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
    .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let num: i32 = read();
    let ans: f32 = num as f32 * 10 as f32;
    println!("{}", ans);
}

// let num: i32 = read();
// let ans: f32 = 1 as f32 / num as f32;
// println!("{}", ans);
