fn main() {
    println!("Which temperature in Celsius would you like to convert to Fahrenheit?");
    
    // Get input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
  
    // Convert input string to a float
    let t_celcius_f: f32 = input.trim().parse().expect("Please enter a valid number");

    // Perform the conversion
    let t_fahrenheit: f32 = t_celcius_f * (9.0 / 5.0) + 32.0;

    // Output the result
    println!("{t_celcius_f}°C is equivalent to {t_fahrenheit}°F!");
}
