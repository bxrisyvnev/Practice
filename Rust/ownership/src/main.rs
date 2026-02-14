fn main() {
    let mut x: i32 = 1;
    let mut y: i32 = x;
    {
        x = 2;
    }
    println!("{x}");
    println!("{y}");
    y = x;
    println!("{y}");

    let mut a = String::from("hello");
    a = "helloo".to_string();
    println!("{a}");
}
