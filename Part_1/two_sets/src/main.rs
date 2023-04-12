use std::io::stdin;

fn print_set(set: Vec<i64>) {
    println!("{}", set.len());
    for num in set {
        print!("{} ", num);
    }
    println!();
}

fn main() {
    let mut n_str = String::new();
    stdin().read_line(&mut n_str).unwrap();
    // n
    let n: i64 = n_str.trim().parse().unwrap();

    // The total of both sets
    let set_total: i64 = (n * (n + 1)) / 2;

    // Total must be divisible by 2 to divide the set equally
    if set_total % 2 != 0 {
        println!("NO");
    } else {
        println!("YES");

        let set_sum: i64 = set_total / 2;

        // The other set is first filled with the largest consecutive numbers from he total set
        // The smallest of these is determined by solving the equation set_total - (n(n+1))/2 = set_sum
        let consecutive_start =
            (-0.5 + (0.25 - (2 * (set_sum - set_total)) as f64).sqrt()) as i64 + 2;
        let consecutive_sum = ((n - consecutive_start + 1) * (consecutive_start + n)) / 2;

        // If the set is already filled by the consecutive numbers
        if consecutive_sum == set_sum {
            print_set((1..consecutive_start).collect());
            print_set((consecutive_start..=n).collect());
        }
        // If the set is not yet filled by the consecutive numbers an additional number is used to fill it
        else {
            let set_filler = set_sum - consecutive_sum;
            let mut consecutive_set: Vec<i64> = (consecutive_start..=n).collect();

            let mut set_1 = Vec::from([set_filler]);
            set_1.append(&mut consecutive_set);
            print_set(set_1);

            let mut set_2_part_1: Vec<i64> = (1..set_filler).collect();
            let mut set_2_part_2: Vec<i64> = ((set_filler + 1)..consecutive_start).collect();
            let mut set_2: Vec<i64> = Vec::new();
            set_2.append(&mut set_2_part_1);
            set_2.append(&mut set_2_part_2);
            print_set(set_2);
        }
    }
}
