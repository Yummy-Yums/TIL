use std::fmt;

// When you need to implement a foreign trait such as std::fmt::Display on a foreign type: Vec<T>,
// you cannot do the following
impl fmt::Display for Vec<String> {
    // Compile Error E1107
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// NewType Pattern: single-element struct tuple
struct CommaSeparated(Vec<String>);

// orphan rule satisfied
impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.join(","))
    }
}

fn main() {
    let list = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    println!("{:?}", list);
}
