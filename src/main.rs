use clap::Parser;

use trust::add;
use trust::div;
use trust::mul;
use trust::sub;


fn main() {
    println!("Hello, world!");
    println!("add(2, 3) = {}", add(2, 3));
    println!("sub(5, 3) = {}", sub(5, 3));
    println!("mul(2, 3) = {}", mul(2, 3));
    println!("div(6, 3) = {}", div(6, 3));
}
