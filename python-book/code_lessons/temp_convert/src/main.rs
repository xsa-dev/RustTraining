fn classify(temp_f: f64) -> &'static str {
    if temp_f > 80.0 {
        "hot"
    } else if temp_f > 50.0 {
        "mild"
    } else {
        "cold"
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 20.0
}

fn main() {
    let celsius_to_fahrenheit_test_temp_1: f64 = 0.0;
    let celsius_to_fahrenheit_test_temp_2: f64 = 20.0;
    let celsius_to_fahrenheit_test_temp_3: f64 = 35.0;

    let temp_celsius_1 = celsius_to_fahrenheit(celsius_to_fahrenheit_test_temp_1);
    let temp_celsius_2 = celsius_to_fahrenheit(celsius_to_fahrenheit_test_temp_2);
    let temp_celsius_3 = celsius_to_fahrenheit(celsius_to_fahrenheit_test_temp_3);

    let classify_temp_celsius_1 = classify(temp_celsius_1);
    let classify_temp_celsius_2 = classify(temp_celsius_2);
    let classify_temp_celsius_3 = classify(temp_celsius_3);

    println!("{temp_celsius_1} is {classify_temp_celsius_1}");
    println!("{temp_celsius_2} is {classify_temp_celsius_2}");
    println!("{temp_celsius_3} is {classify_temp_celsius_3}");
}
