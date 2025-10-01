#[derive(Debug, Clone)]
struct Student {
    name: String,
    major: String,
}

impl Student {
    pub fn new(name: impl Into<String>, major: impl Into<String>) -> Self {
        Self { name: name.into(), major: major.into() }
    }

    pub fn set_major(&mut self, new_major: impl Into<String>) {
        self.major = new_major.into();
    }

    pub fn get_major(&self) -> &str {
        &self.major
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let mut s = Student::new("Guillermo", "Undeclared");
    println!("Student: {} | major = {}", s.get_name(), s.get_major());

    s.set_major("Computer Science");
    println!("Updated: {} | major = {}", s.get_name(), s.get_major());
}
