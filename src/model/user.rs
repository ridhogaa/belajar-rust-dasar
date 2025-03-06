pub struct User {
    name: String,
    age: i32,
}

impl User {

    pub fn new(name: String, age: i32) -> Self {
        User { name, age }
    }

    pub fn greet(&self, name: String) {
        println!("Hello, {}, from {}", name, self.name);
        println!("Age, {}", self.age);
    }
}
