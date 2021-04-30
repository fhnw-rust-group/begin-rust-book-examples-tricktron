fn main() {
    let mut x = 5;
    x = 10;
    let y = &x;
    println!("x == {}, y == {}", x, y);
    println!("x == {}, y == {}", x, y);
}

fn not_valid() {
    let mut x = 5;
    let y = &mut x;
    //let z = &x; cannot borrow as mutable and immutable
    *y *= 2;
}

struct RunningTotal {
    total: i32,
    count: i32
}

fn new_running_total() -> RunningTotal {
    RunningTotal {
        total: 0,
        count: 0
    }
}

fn add_value(rt: &mut RunningTotal, value: i32) {
    rt.total += value;
    rt.count += 1;
}

fn average(rt: &RunningTotal) -> String {
    // FIXME by returning an exception here
    if rt.count == 0 {
        "I don't have any values yet!".to_owned()
    } else {
        (rt.total / rt.count).to_string()
    }
}

fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;
    }
}

fn sum_no_loop(total: &mut i32, low: i32, high: i32) {
    let n = high - low + 1;
    *total = (n * (n + 1)) / 2;
}

fn triple(x: &mut i32) {
    *x = *x * 3;
}

fn is_five(x: &i32) -> bool {
    *x == 5
}

fn is_five_strange(x: &i32) -> bool {
    x == &5
}

fn add(x: i32, y:i32) -> i32 {
    x + y
}

fn add_with_references(x: &i32, y: &i32) -> i32 {
    x + y
}

fn double(x: i32) -> i32 {
    add(x, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(double(5), 10);
    }

    #[test]
    fn exercise1() {
        assert!(is_five(&5));
        assert!(!is_five(&6));
        assert!(is_five_strange(&5));
        assert!(!is_five_strange(&6));
    }

    #[test]
    fn exercise2() {
        let result = add_with_references(&5, &6);
        assert_eq!(result, 11);
    }

    #[test]
    fn exercise3() {
        let mut x = 5;
        triple(&mut x);
        assert_eq!(x, 15);
    }

    #[test]
    fn exercise4() {
        let mut x = 5;
        let y: &mut i32 = &mut x;
        *y += 1;
        assert_eq!(x, 6);
    }

    #[test]
    fn exercise5() {
        let mut total = 0;
        let mut total1 = 0;
        sum(&mut total, 1, 10);
        sum_no_loop(&mut total1, 1, 10);
        assert_eq!(total, 55);
        assert_eq!(total1, 55);
        let mut total = 10;
        sum(&mut total, 1, 10);
        assert_eq!(total, 65);
    }

    #[test]
    fn exercise6() {
        let mut x: i32 = 5;
        let y: &mut i32 = &mut x;
        *y += 1;
        assert_eq!(y, &6);
        assert_eq!(x, 6);
    }

    #[test]
    fn exercise7() {
        let mut rt = new_running_total();
        // FIXME by asserting exception here
        assert_eq!(average(&rt), "I don't have any values yet!");
        add_value(&mut rt, 5);
        add_value(&mut rt, 3);
        assert_eq!(average(&rt), 4.to_string());
    }
}