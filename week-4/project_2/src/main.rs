use std::io;

fn main() {
    
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let is_experienced = experience_input.trim().to_lowercase() == "yes";

  
    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Invalid input for age");

    
    let incentive: u32;
    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_480_000;
        }
    } else {
        incentive = 100_000;
    }

    // Output the incentive
    println!("The annual incentive for the employee is: N{}", incentive);
}
