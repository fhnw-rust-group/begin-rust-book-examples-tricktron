use Job::*;
use Science::*;

#[derive(PartialEq, Eq)]
enum Job {
    Teacher,
    Scientist(Science),
    Chef,
}

#[derive(PartialEq, Eq)]
enum Science {
    Physics,
    Biology,
}
struct Person {
    name: String,
    age: u32,
    job: Job,
}

impl Person {
    fn same_job(&self, other: &Person) -> bool {
        self.job == other.job
    }

    fn greeting(&self) -> String {
        match self.job {
            Teacher => format!("Hello, you're a teacher named {}", self.name),
            Scientist(Physics) => format!("Hello, you're a physicist named {}", self.name),
            Scientist(Biology) => format!("Hello, you're a biologist named {}", self.name),
            Chef => format!("Hello, you're a Chef named {}", self.name),
        }
    }

    fn is_teacher(&self) -> bool {
        match self.job {
            Teacher => true,
            Scientist(_) => false,
            Chef => false
        }
    }
}

fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
        job: Chef,
    };
    println!("{}", alice.greeting());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_same_jobs() {
        let alice = Person {
            name: "Alice".to_owned(),
            age: 30,
            job: Scientist(Physics),
        };
        let bob = Person {
            name: "Bob".to_owned(),
            age: 35,
            job: Scientist(Physics),
        };
        let charlie = Person {
            name: "Charlie".to_owned(),
            age: 40,
            job: Teacher,
        };
        assert!(alice.same_job(&bob));
        assert!(!alice.same_job(&charlie));
    }

    #[test]
    fn test_combining_enums() {
        let rick = Person {
            name: "Rick".to_owned(),
            age: 60,
            job: Scientist(Physics),
        };
        assert!(!rick.is_teacher());
        assert_eq!("Hello, you're a physicist named Rick".to_owned(), rick.greeting());
    }
}
