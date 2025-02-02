use std::io::Read;
use std::io;  

fn main() {  
    fn staff() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();  
    let mut contents = String::new();  
    file.read_to_string(&mut contents).unwrap();  
    print!("{}", contents);  
}  
    fn globacom() {
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();  
    let mut contents = String::new();  
    file.read_to_string(&mut contents).unwrap();  
    print!("{}", contents);  
}  
    fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();  
    let mut contents = String::new();  
    file.read_to_string(&mut contents).unwrap();  
    print!("{}", contents);  
}  
    fn dataplan() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();  
    let mut contents = String::new();  
    file.read_to_string(&mut contents).unwrap();  
    print!("{}", contents);  
}  
    fn project() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();  
    let mut contents = String::new();  
    file.read_to_string(&mut contents).unwrap();  
    print!("{}", contents);  
}  
    println!("Globacom Ltd. All rights reserved © 2025 ");
    println!("What is your role in the organization? ");
    println!("1.Administrator\n");
    println!("2.Employee\n");
    println!("3.Project Manager\n");
    println!("4.Customer\n");
    println!("5.Vendor");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let access:i32 = input1.trim().parse().expect("Not an integer");
    if access == 1 {
        println!("Since you are an administrator, this is the data you will be working with...");
        globacom();

    }
    if access == 2 {
        println!("Since you are an employee, this is the data you will be working with...");
        staff();

    }
    if access == 3 {
        println!("Since you are a project manager, this is the data you will be working with...");
        project();

    }
    if access == 4 {
        println!("Since you are a customer, this is the data you will be working with...");
        customer();

    }
    if access == 5 {
        println!("Since you are a vendor, this is the data you will be working with...");
        dataplan();

    }

    

}
