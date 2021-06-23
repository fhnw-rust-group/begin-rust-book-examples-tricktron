struct Person<Name, Age> {
    name: Name,
    age: Age,
}

fn greet<T>(person: &Person<&str, T>) {
    println!("Hello, {}", person.name);
}

fn main() {
    let alice: Person<&str, u32> = Person {
        name: "Alice",
        age: 30_u32,
    };
    greet(&alice);
    let bob: Person<&str, u64> = Person {
        name: "Bob",
        age: 35_u64,
    };
    greet(&bob);
}
