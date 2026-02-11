mod operators;

use operators::add;
use operators::div;
use operators::mul;
use operators::sub;
use std::io;

fn main() {
    while true {
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Pls type an actual number pls!");

        let x: i32 = x.trim().parse().expect("Please type a number!");

        let mut operator = String::new();

        io::stdin()
            .read_line(&mut operator)
            .expect("Please type an operator");

        let operator = operator.trim();

        let mut y = String::new();

        io::stdin()
            .read_line(&mut y)
            .expect("Pls type an actual number pls!");

        let y: i32 = y.trim().parse().expect("Please type a number!");

        if operator == "+" {
            let result: i32 = add::add(x, y);
            println!("Answer is {result}");
        } else if operator == "-" {
            let result: i32 = sub::sub(x, y);
            println!("Answer is {result}");
        } else if operator == "/" {
            let result: i32 = div::div(x, y);
            println!("Answer is {result}");
        } else if operator == "*" {
            let result: i32 = mul::mul(x, y);
            println!("Answer is {result}");
        } else {
            println!("Invalid operator!")
        }
    }
}
