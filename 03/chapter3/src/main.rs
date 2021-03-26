fn main() {
    42; // needs ";" because main cannot return i32
    println!("{}", 42) // works because println -> ()
                       //say_apples(42);
                       //exercise3();
}

fn say_apples(apples: i32) {
    println!("I nave {}", apples);
}

fn exercise3() {
    let x = 5;
    {
        let x = 6;
        println!("First time: {}", x);
    }
    println!("Second time: {}", x);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

/* currying does not work as easily
fn add_3_curry(x: i32) -> fn(y: i32) {
    add(3)
}*/

fn add_3(x: i32) -> i32 {
    add(3, x)
}

fn add_4(x: i32) -> i32 {
    add(4, x)
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    double(double(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise4_some_math() {
        let x = (5 + 3) * (6 + 4);
        let y = times(add_3(5), add_4(6));
        assert_eq!(x, y);
        println!("Good job!");
    }

    #[test]
    fn exercise7_quadruple() {
        assert_eq!(4, quadruple(1));
        assert_eq!(8, quadruple(2));
        assert_eq!(12, quadruple(3));
        assert_eq!(48, quadruple(quadruple(3)));
    }
}
