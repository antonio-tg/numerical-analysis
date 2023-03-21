// This program give an approach to pi using the solution of Euler to the famous series 1/x²

fn main(){
    let mut sum: f64 = 0.0;
    let epsilon: f64 = 1E-16;
    let pi: f64;
    let mut term: f64;
    let mut i:f64 = 0.0;
    loop {
        i += 1.0;
        term = 1.0/(i*i);
        sum += term;
        if term < epsilon {
            break;
        }
        //println!("{} {} {}", term, epsilon,i);
    }
    pi = sum*6.0;
    println!("The approach of pi using an epsilon of {} is {}", epsilon, pi.sqrt());
}
