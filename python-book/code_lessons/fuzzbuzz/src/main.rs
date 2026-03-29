// Challenge: Write FizzBuzz for 1..=30 
// using Rust’s expression-based match. 
// Each number should print “Fizz”, “Buzz”, “FizzBuzz”, or the number. 
// Use match (n % 3, n % 5) as the expression.

fn main() {
    for n in 1..=30 {
        let result = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => format!("{}", n),
        };
        println!("{} => {}", n, result);
    }

    
}
