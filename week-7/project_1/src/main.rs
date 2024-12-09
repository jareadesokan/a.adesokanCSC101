use std::io;
fn trapezium() ->f32{
    let mut a = String::new();
    println!("Enter the value of the first parallel side: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    
    let mut b = String::new();
    println!("\nEnter the value of the second parallel side: ");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid integer");
    
    let mut c = String::new();
    println!("\nEnter the value of the height: ");
    io::stdin().read_line(&mut c).expect("Not a valid input");
    let c:f32 = c.trim().parse().expect("Not a valid integer");

    let area = 0.5 * (a + b) * c;
    println!("Your area is {}", area);
    return area;
}

fn rhombus()->f32{
    let mut a = String::new();
    println!("Enter the value of the first diagonal side: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    
    let mut b = String::new();
    println!("\nEnter the value of the second diagonal side: ");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid integer");
    
    

    let area = 0.5 * a  * b;
    println!("Your area is {}", area);

    return area;
    
}
fn parallelogram()->f32{
    let mut a = String::new();
    println!("Enter the value of the base side: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    
    let mut b = String::new();
    println!("\nEnter the value of the altitude side: ");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid integer");
    
    

    let area = a *b;
    println!("Your area is {}", area);

    return area;
    
}
fn cube()->f32{
    let mut a = String::new();
    println!("Enter the value of the  side: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    let area = a.powf(2.0) * 6.0 ;
    println!("Your area is {}", area);
    return area;
   
}
fn cylinder() -> f32{
    let mut a = String::new();
    println!("Enter the value of the radius: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    
    let mut b = String::new();
    println!("\nEnter the value of the height : ");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid integer");
    
    let pi = 3.14;

    let area = a * a * b * pi;
    println!("Your area is {}",area);
    return area;
    
}
fn main() {
    println!("Welcome to Mr.Desmond's Equation Calculator!");
    println!("\nThese are the equations and their codes:");
    println!("\nType 1 for trapezium");
    println!("\nType 2 for rhombus");
    println!("\nType 3 for parallelogram");
    println!("\nType 4 for cube");
    println!("\nType 5 for cylinder");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid input");
    let choice:i32 = choice.trim().parse().expect("Not a valid integer");

    
        if choice == 1{
             trapezium();
             
        }
        else if choice == 2{
             rhombus();
             
        }
        else if choice == 3{
             parallelogram(); 
             
        }
        else if choice == 4{
             cube();
              
        }
        else if choice == 5{
            cylinder();
            
        }
        else {
            println!("Not a valid input");
        }
}