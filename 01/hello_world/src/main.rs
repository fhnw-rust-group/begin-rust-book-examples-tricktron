
fn main() {
    let apples = 10;
    let answer = (6 / 2 + 4) * 3;
    println!("I have {} apples and the answer is {}", apples, answer);
}

fn exercise4() -> String {
    let first_line = "Hello, world!";
    let second_line = "I have 10 apples.";
    let third_line = "Goodbye!";
    let lines = [first_line, second_line, third_line];
    return lines.join("\n");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise4_print_output() {
        assert_eq!("Hello, world!\nI have 10 apples.\nGoodbye!", exercise4());
    }
}
