fn main() {
    let x = 5_i32;
    println!("x == {}", x.abs());
    println!("x == {}", (-5_i32).abs());

    print_fruit(increase_fruit(new_fruit()));
    Fruit::new().increase().print();
}

struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn new() -> Fruit {
        Fruit {
            apples: 10,
            bananas: 5,
        }
    }

    fn price(&self) -> i32 {
        self.apples * 8 + self.bananas * 12
    }

    fn increase(mut self) -> Fruit {
        self.apples *= 2;
        self.bananas *= 3;
        self
    }

    fn print(&self) {
        println!("You have {} apples and {} bananas", self.apples, self.bananas);
    }
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }
}

fn print_fruit(fruit: Fruit) {
    println!("You have {} apples and {} bananas", fruit.apples, fruit.bananas);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise1() {
        assert_eq!(concat_length("Hello", "World"), 10);
    }

    #[test]
    fn exercise2() {
        assert_eq!(5_i32.pow(2) + 3_i32.pow(3), 52);
    }
}

fn concat_length(s1: &str, s2: &str) -> usize {
    s1.len() + s2.len()
}