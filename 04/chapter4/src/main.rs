fn main() {
    println!("Chapter 4 Booleans");
}

fn can_drive(name: &str, age: u32) -> bool {
    (name != "Michael" && age >= 16) || (age >= 18)
}

fn can_see_movie(age: i32, parents_permit: bool) -> bool {
    (age >= 17) || (age >= 13 && parents_permit)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exercise1_micheal_can_only_drive_at_age_18_everyone_else_at_age_16() {
        assert!(can_drive("Michael", 18));
        assert!(!can_drive("Michael", 17));
        assert!(can_drive("Miriam", 16));
        assert!(!can_drive("Miriam", 15));
        println!("Success!");
    }

    #[test]
    fn exercise2_can_watch_movie_if_older_than_17_or_13_and_parents_permission() {
        assert!(can_see_movie(17, false));
        assert!(can_see_movie(21, false));
        assert!(!can_see_movie(13, false));
        assert!(can_see_movie(13, true));
        assert!(!can_see_movie(12, true));
    }
}
