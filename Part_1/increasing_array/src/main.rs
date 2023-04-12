use std::io::stdin;

fn main() {
    let mut len_str = String::new();
    stdin().read_line(&mut len_str).unwrap();
    let _length: u32 = len_str.trim().parse().unwrap();

    let mut num_str = String::new();
    stdin().read_line(&mut num_str).unwrap();
    let numbers: Vec<u64> = num_str
        .split_whitespace()
        .map(|n| n.trim())
        .map(|n| n.parse().unwrap())
        .collect();

    let mut steps = 0;
    let mut last = numbers.first().unwrap();

    for n in &numbers[1..] {
        if n > last {
            last = n;
        } else {
            steps += last - n;
        }
    }

    println!("{}", steps)
}
