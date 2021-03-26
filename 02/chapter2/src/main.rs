fn main() {
    // this is a comment
    let apples: i32 = {
        println!("I'm about to figure out how many apples there are");
        let x = 10 + 5;
        println!("Now I know how many apples there are");
        x
    };
    println!("I will be evaluated after the fist statement with its side-effects");
    println!("I have {} apples", apples);
    println!("I will be evaluated last");
    // exercise1();
    exercise5();
}

fn exercise1() {
    let name = { "Michael" };
    println!("My name is {}", name);
}

fn exercise2() {
    println!("4 + 5  == {}", {
        {
            4 + 5
        }
    });
}

fn exercise5() {
    println!("Far over the Misty Mountains cold");
    "Dragon!!!";
    let dwarves: i32 = 13;
    let hobbit: i32 = {
        "Not a wizard";
        5 - 4
    };
    dwarves - 3;
    println!(
        "The Company of Thorin has {} dwarves and {} hobbit, {}
        members in all.",
        dwarves,
        hobbit,
        dwarves + hobbit
    );
    println!(
        "After the Battle of the Five Armies, there were only {} dwarves remaining.",
        dwarves - 3
    );
}
