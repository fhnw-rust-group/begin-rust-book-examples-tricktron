use std::fmt::Display;

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl Double for i64 {
    fn double(&self) -> Self {
        self * 2
    }
}

trait DoubleString {
    fn double_string(&self) -> String;
}

impl DoubleString for i64 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}

impl DoubleString for i32 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}

fn print_double<T: DoubleString>(x: T) {
    println!("The doubled value is {}", x.double_string());
}

fn quadruple<T: Double>(x: T) -> T {
    x.double().double()
}

#[derive(Clone)]
struct Fruit<T> {
    apples: T,
    bananas: T
}

impl<T: Double> Double for Fruit<T> {
    fn double(&self) -> Self {
        Fruit {
            apples: self.apples.double(),
            bananas: self.bananas.double()
        }
    }
}

fn stringy<T:Display>(x: T) -> String {
    format!("{}", x)
}


fn main() {
    println!("double 5_i32 == {}", 5_i32.double());
    println!("double 5_i64 == {}", 5_i64.double());
    print_double(5_i32);
    print_double(20_i64);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generalise_stringy_to_any_formattable_type() {
        assert_eq!(stringy(42), "42".to_owned());
        assert_eq!(stringy(true), "true".to_owned());
        assert_eq!(stringy("hello"), "hello".to_owned());
    }
}
