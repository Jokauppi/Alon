use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut n: u64;
    n = input.trim().parse().unwrap();
    let mut output = String::new();

    while n != 1 {
        output.push_str(&format!("{} ", n));

        match n % 2 {
            0 => {
                n /= 2;
            }
            1 => {
                n = 3 * n + 1;
            }
            _ => break,
        }
    }

    output.push_str("1");
    println!("{}", output)
}
