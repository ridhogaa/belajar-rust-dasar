mod model {
    pub mod user;
}
use model::user::User;

fn main() {
    println!("Hello, world!");
    let s = String::from("Hello");
    print_str(&s); // ✅ Meminjam data, tidak memindahkan ownership
    println!("{}", s); // ✅ Masih bisa digunakan
    let user = User::new(String::from("Ridho"), 22);
    user.greet(String::from("Budi"));
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

// struct User {
//     name: String,
//     age: i32,
// }

// impl User {
//     fn greet(&self) {
//         println!("Hello, {}", self.name);
//         println!("Age, {}", self.age);
//     }
// }

fn print_str(s: &String) {
    println!("{}", s);
}

#[test]
fn struct_test() {
    let result = add(1, 2);
    println!("{}", result);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Ridho");
    let c = "Ridho";
    println!("a -> {}", a);
    println!("b -> {}", b);
    println!("c -> {}", c);
}

fn function_b() {
    let a = 10;
    let b = String::from("Ridho Gymnastiar          ");

    println!("a -> {}", a);
    println!("b -> {}", b.trim());
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;
    println!("a -> {}", a);
    println!("b -> {}", b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Ridho");
    let name2 = name1;
    println!("name2 -> {}", name2);

    let name1 = String::from("Ridho");
    let name2 = &name1;
    println!("{}", name1);
    println!("name2 -> {}", name2);
}

#[test]
fn say_hello() {
    let result = factorial_loop(5);
    println!("{}", result);
    let result = factorial_recursion(5);
    println!("{}", result);
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn factorial_recursion(n: i32) -> i32 {
    if n < 1 {
        0;
    }
    n * factorial_recursion(n - 1)
}
