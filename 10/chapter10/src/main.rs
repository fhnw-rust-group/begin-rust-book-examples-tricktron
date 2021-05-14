use std::fmt::format;

fn main() {
    let name: &'static str = "Michael";
    println!("Name is {}", name);
    let person = Person {
        name: "John",
        age: 42
    };
    println!("{} is {} years old", person.name, person.age);
    let first_name = "Jon";
    let last_name = " Snow";
    let full_name: std::string::String = first_name.to_owned() + last_name;
    println!("Full name is {}", full_name);
    greet(&full_name);
    greet(&full_name);
    greet(first_name);

    let p2 = Person {
        name: &first_name,
        age: 64
    };

    greet_person(&person);
    greet_person(&person);
    greet_person(&p2);
    greet_person(&p2);

    let alice = BetterPerson {
        name: "Alice Smith".to_owned(),
        age: 30
    };

    let bob = BetterPerson {
        name: "Bob Johnson".to_owned(),
        age: 25
    };
    alice.say_info();
    bob.say_info();

    let mut hello = "Hello, ".to_owned();
    hello += "world!";
    println!("{}", hello);

    let test_name = "Michael".to_owned();
    say_hi(&test_name);
    say_hi(&test_name);
    say_hi(first_name);

    let michael_ex4: String = add_snoyman("Michael");
    let miriam_ex4: String = add_snoyman("Miriam");
    let john_ex4: &'static str = "John Doe";

    say_hi(&michael_ex4);
    say_hi(&miriam_ex4);
    say_hi(&john_ex4);
}

fn add_snoyman(first: &str) -> String {
    format!("{} Snoyman", first)
}

fn say_hi(name: &str) {
    println!("Hi {}", name);
}

fn greet(name: &str) {
    println!("Hello {}", name);
}

fn greet_person(person: &Person) {
    println!("Hello {}", person.name);
}

fn concat(s1: &str, s2: &str) -> String {
    s1.to_owned() + s2
}

struct Person {
    name: &'static str,
    age: i32
}

struct BetterPerson {
    name: String,
    age: i32
}

impl BetterPerson {
    fn say_info(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}

fn make_full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_macro_test() {
        assert_eq!(&make_full_name("Michael", "Snoymann"), "Michael Snoymann");
        assert_eq!(&make_full_name("Alice", "Smith"), "Alice Smith");
    }
}