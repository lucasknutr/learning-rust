fn main() {
    let number = sum(23, 12);
    println!("{}", number);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// Learning how to use different types of functions in Rust

fn stringToInteger(str: &str) -> i32 {
    let x = str.parse::<i32>().unwrap();
    x
}

fn integerToString() {
    let x = 123.to_string();
    println!("{}", x);
}

// Is a value without semicolon a return value?
// Yes, it is. The last expression in a function without a semicolon is a return value.
// So, there is not an explicit 'return' keyword in Rust?

fn fileIO() {
    let mut file = File::open("data.yaml").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read the file");
    println!("File contents: {}", contents);
}

fn esp32Button() {
    let mut button = Button::new(0);
    loop {
        if button.is_pressed() {
            println!("Apertou o botão");
        } else {
            println!("Não foi pres");
        }
        sleep(Duration::from_millis(100));
    }
}
