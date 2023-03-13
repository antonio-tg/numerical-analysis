// This program solves a quadratic equation of the second degree
//use std::io::stdin;

use std::io;

fn main(){
    println!("This program solves a quadratic equation of the second degree like ax²+bx+c=0, enter the coefficients:");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("Failed to read a");
    let a: f64 = a.trim().parse().expect("Invalid input for a");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect("Failed to read b");
    let b: f64 = b.trim().parse().expect("Invalid input for b");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut c).expect("Failed to read c");
    let c: f64 = c.trim().parse().expect("Invalid input for c");

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots are real and distinct:");
        println!("x1 = {}", root1);
        println!("x2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);

        println!("The roots are real and equal:");
        println!("x = {}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);

        println!("The roots are complex conjugates:");
        println!("x1 = {} + {}i", real_part, imaginary_part);
        println!("x2 = {} - {}i", real_part, imaginary_part);
    }
}

