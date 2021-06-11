use std::fmt::Display;

fn print_them_all<I>(iter: I)
where
    I: Iterator,
    I::Item: Display,
{
    for x in iter {
        println!("{}", x);
    }
}

fn sum<I: Iterator<Item = u32>>(iter: I) -> u32 {
    let mut total = 0;
    for val in iter {
        total += val;
    }
    total
}

fn main() {
    let mut total = 0;

    for i in 1..11 {
        total += i;
    }
    assert_eq!(total, 55);

    let fibs: Vec<u32> = vec![1, 1, 2, 3, 5, 8, 13];
    for x in fibs {
        println!("{}", x);
    }

    let fibs_array = [1, 1, 2, 3, 5, 8, 13];
    for x in &fibs_array {
        println!("{}", x);
    }

    let name = "Pickle Rick";
    for x in name.chars() {
        println!("{}", x);
    }
    for x in name.bytes() {
        println!("{}", x);
    }

    print_them_all(1..11);
    println!("{}", sum(1..11));
}
