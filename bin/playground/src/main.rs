use std::ops::Deref;

struct Original {
    value: i32,
}

impl Original {
    fn new(value: i32) -> Self {
        Original { value }
    }

    fn display(&self) {
        println!("Value: {}", self.value);
    }
}

struct Extended {
    original: Original,
    new_member: String,
}

impl Extended {
    fn new(original: Original, new_member: String) -> Self {
        Extended {
            original,
            new_member,
        }
    }

    fn display_new_member(&self) {
        println!("New Member: {}", self.new_member);
    }
}

// Implement Deref for Extended
impl Deref for Extended {
    type Target = Original;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

fn main() {
    let original = Original::new(10);
    let extended = Extended::new(original, String::from("Hello"));

    extended.display(); // Now you can call display() directly
    extended.display_new_member(); // Accessing method from Extended
}
