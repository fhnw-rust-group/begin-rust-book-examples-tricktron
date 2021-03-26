fn main() {
    println!("Chapter 5 Conditionals");
    exercise2();
    exercise3();
}

fn _main() {
    let is_hot = false;
    if is_hot {
        println!("It's hot!");
    } else {
        println!("It's not hot!");
    }
}

fn _tell_temperature(temp: i32) {
    println!(
        "{}",
        if temp <= 10 {
            "It's cold!"
        } else if temp <= 25 {
            "It's nice"
        } else if temp <= 30 {
            "It's warm"
        } else {
            "It's hot!"
        } // no ; allowed here because we need need an expression = evaluated
    );
}

fn want_apples() -> bool {
    true
}
fn want_five() -> bool {
    false
}
fn talk_about_fruit() -> bool {
    true
}
fn talk_about_numbers() -> bool {
    false
}
fn exercise2() {
    let fruit = if want_apples() { "apple" } else { "banana" };
    let number = if want_five() { 5 } else { 6 };
    let _x = if talk_about_fruit() {
        println!("The fruit is {}", fruit);
    };
    let _y = if talk_about_numbers() {
        println!("The number is {}", number);
    };
}

fn comment(apples: i32) {
    println!(
        "You have {} apples",
        if apples > 10 { "lots of" } else { "very few" }
    );
}
fn exercise3() {
    comment(5);
    comment(100);
}

fn is_cold() -> bool {
    true // feel free to change this to false
}

fn is_hot() -> bool {
    !is_cold()
}

fn can_go_outside(is_raining: bool, temp: i32) -> bool {
    unimplemented!()
}

fn tell_kids(is_raining: bool, temp: i32) -> &'static str {
    if !is_raining || temp >= 10 {
        "You can go outside"
    } else {
        "Sorry, it's too cold and it's raining"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise4_invert_function() {
        assert_ne!(is_cold(), is_hot());
        println!("Success!");
    }

    #[test]
    fn exercise5_tell_kids() {
        assert_eq!(tell_kids(true, 9), "Sorry, it's too cold and it's raining");
        assert_eq!(tell_kids(true, 10), "You can go outside");
        assert_eq!(tell_kids(false, 9), "You can go outside");
        assert_eq!(tell_kids(false, 10), "You can go outside");
    }
}
