use std::fmt;

struct Fruit {
    apples: i32,
    bananas: i32,
}

impl fmt::Display for Fruit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You have {} apples and {} bananas", self.apples, self.bananas)
    }
}

fn increase_fruit0(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

struct FruitAndPrice {
    fruit: Fruit,
    price: i32,
}

fn count_fruit(fruit: Fruit) -> Fruit {
    println!(
        "I've got {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
    fruit
}
fn price_fruit(fruit: Fruit) -> FruitAndPrice {
    let price = fruit.apples * 8 + fruit.bananas * 12;
    FruitAndPrice {
        fruit: fruit,
        price: price,
    }
}
fn increase_fruit(mut fruit: Fruit) -> Fruit {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    fruit
}

struct Point {
    x: i32,
    y: i32,
}
fn print_point(point: Point) {
    println!("x == {}, y == {}", point.x, point.y);
}

fn exercise4() {
    print_point(Point { x: 3, y: -6 });
}

fn main() {
    exercise4();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise1_no_let_statements() {
        let fruit = increase_fruit0(new_fruit());
        assert_eq!(format!("{}", fruit), "You have 20 apples and 15 bananas");
    }

    #[test]
    fn exercise2_print_price_for_original_and_increased_fruit() {
        let fruit = Fruit {
            apples: 10,
            bananas: 5,
        };
        let line1 = format!("I've got {} apples and {} bananas", fruit.apples, fruit.bananas);
        let fruit_and_price = price_fruit(fruit);
        let line2 = format!("Original price: {}", fruit_and_price.price);
        let fruit = increase_fruit(fruit_and_price.fruit);
        let fruit_and_price = price_fruit(fruit);
        let line3 = format!("I can make {} cents for more fruit", fruit_and_price.price);
        assert_eq!(line1, "I've got 10 apples and 5 bananas");
        assert_eq!(line2, "Original price: 140");
        assert_eq!(line3, "I can make 340 cents for more fruit");
    }

    #[test]
    fn exercis3_print_price_for_twice_increased_fruit() {
        let fruit = Fruit {
            apples: 10,
            bananas: 5,
        };
        let fruit_and_price = price_fruit(fruit);
        let fruit = increase_fruit(fruit_and_price.fruit);
        let fruit = increase_fruit(fruit);
        let fruit_and_price = price_fruit(fruit);
        let twice_increased = format!("I can make {} cents for even more fruit", fruit_and_price.price);
        assert_eq!(twice_increased, "I can make 860 cents for even more fruit");
    }
}
