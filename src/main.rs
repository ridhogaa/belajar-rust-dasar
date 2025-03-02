fn main() {
    println!("Hello, world!");
    let s = String::from("Hello");
    print_str(&s); // ✅ Meminjam data, tidak memindahkan ownership
    println!("{}", s); // ✅ Masih bisa digunakan
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn variable_test() {
    // Immutable variable
    let name = "Ridho Gymnastiar Al Rasyid";
    println!("Hello {}", name);

    // Mutable variable
    let mut name = "Ridho Gymnastiar Al Rasyid";
    println!("Name -> {}", name);
    name = "Gajadi Ridho";
    println!("Name -> {}", name);
}

#[test]
fn tuple_test() {
    let data = ("Ridho", "Gajadi", 22, "Jakarta", 20.0);

    println!("Name -> {}", data.0);
    println!("Last Name -> {}", data.1);
    println!("Age -> {}", data.2);
    println!("City -> {}", data.3);
    println!("Height -> {}", data.4);
    {
        let (name, last_name, age, city, height) = data;
        println!("Name -> {}", name);
        println!("Last Name -> {}", last_name);
        println!("Age -> {}", age);
        println!("City -> {}", city);
        println!("Height -> {}", height);
    }
}

#[test]
fn if_test() {
    let x = 10;
    if x > 5 {
        println!("Greater than 5");
    } else {
        println!("Less than or equal to 5");
    }
}

#[test]
fn loop_test() {
    for i in 0..5 {
        println!("{}", i);
    }

    let mut j = 0;
    while j < 5 {
        println!("{}", j);
        j += 1;
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct User {
    name: String,
    age: i32,
}

impl User {
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}

fn print_str(s: &String) {
    println!("{}", s);
}
