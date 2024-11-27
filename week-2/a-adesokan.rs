use std::io;
fn main(){
	let mut patient_name = String::new;
	let mut day_ofbirth = String::new;
	let mut month_ofbirth = String::new;
	let mut year_ofbirth = String::new;
	let mut email_address = String::new;
	let mut phone_number = String:: new;
	let mut number_siblings = String::new;
	let mut number_children = String::new;
	let mut medical_diagnosis = String::new;
	let mut village_of_residence = String::new;
	
	let count:u32 = 0;

	println!("\nWhat is your name?:");
	io::stdin().read_line(&mut patient_name).expect("Invalid input");

	println!("\nWhen were you born? ");
	println!("\nDay(dd): ");
	io::stdin().read_line(&mut day_ofbirth).expect("Invalid input");
	let day:i32 = day_ofbirth.trim().parse().expect("Invalid input")

	println!("\nMonth(mm): ");
	io::stdin().read_line(&mut month_ofbirth).expect("Invalid input");
	let month:i32 = month_ofbirth.trim().parse().expect("Invalid input")

	println!("\nYear(yyyy): ");
	io::stdin().read_line(&mut year_ofbirth).expect("Invalid input");
	let year:i32 = year_ofbirth.trim().parse().expect("Invalid input")

	let age:i32 = 2024 - year

	println!("\nWhat is your email address?:");
	io::stdin(&mut email_address).readline().expect("Invalid input");
	
    println!("\nWhat is your phone number");
    io::stdin().read_line(&mut phone_number).expect("Invalid input");
    let phone_number:i32 = phone_number.trim().parse().expect("Not a valid input");

    println!("\nHow many siblings do you have?:");
    io::stdin().read_line(&mut number_siblings).expect("Invalid input");
    let number_siblings:i32 = number_siblings.trim().parse().expect("Not a valid input");
    
    println!("\nHow many children do you have?:");
    io::stdin().read_line(&mut number_children).expect("Invalid input");
    let number_children:i32 = number_siblings.trim().parse().expect("Not a valid input");
    
    println!("\nWhat is your medical diagnosis?:");
    io::stdin().read_line(&mut medical_diagnosis).expect("Not a valid input");
 
    println!("\nWhich village do you reside in?:");
    io::stdin().read_line(&mut village_of_residence).expect("Invalid input");

    

    while count < 101 {
    	if medical_diagnosis == "Alzheimer" && age > 50 && number_children > 4 && village_of_residence == "Akpabom"
    	{
	    	 println!("Congratulations, you have received a 20% discount and your new amount is {}", 0.2 * 1_200_000);
	    	 println!("\nName:{}",patient_name);
	    	 println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth);
	    	 println!("\nEmail address:{}",email_address);
	    	 println!("\nPhone Number:{}",phone_number);
	    	 println!("\nNumber of Siblings:{}",number_siblings);
	    	 println!("\nNumber of Children:{}",number_children);
	    	 println!("\nMedical diagnosis:{}",medical_diagnosis);
	    	 println!("\nVillage of Residence:{}",village_of_residence);
    	 count+=1 ;
    	} 
    	else if medical_diagnosis == "Arrythmia" && age == 30 && number_siblings > 4 && village_of_residence =="Ngbauji"
    	{
    		println!("Congratulations, you have received a 5% discount,Your new amount is{}",0.05 * 550_000);
    		println!("\nName:{}",patient_name);
    	    println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth);
    	    println!("\nEmail address:{}",email_address);
	    	println!("\nPhone Number:{}",phone_number);
	    	println!("\nNumber of Siblings:{}",number_siblings);
	    	println!("\nNumber of Children:{}",number_children);
	    	println!("\nMedical diagnosis:{}",medical_diagnosis);
	    	println!("\nVillage of Residence:{}",village_of_residence);
    		count+=1
    	}
    	else if medical_diagnosis == "Chronic Kidney Disease" && age > 40 && number_children > 3 && number_siblings > 3 && village_of_residence == "Atabrikang"{
	        println!("Congratulations you have recived a 15% discount. Your new amount is {}", 0.15 * 1_500_000);
	        println!("\nName:{}",patient_name);
	        println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth);
	        println!("\nEmail address:{}",email_address);
	        println!("\nPhone Number:{}",phone_number);
	        println!("\nNumber of Siblings:{}",number_siblings);
	        println!("\nNumber of Children:{}",number_children);
	        println!("\nMedical diagnosis:{}",medical_diagnosis);
	        println!("\nVillage of Residence:{}",village_of_residence);
        count+=1;
    	}
    	else if medical_diagnosis == "Diabetes" && 45 > age > 28 && 4 > number_children > 2 &&  village_of_residence == "Okorobilom"{
	        println!("Congratulations you have recived a 10% discount. Your new amount is {}", 0.1 * 800_000);
	        println!("\nName:{}",patient_name);
	    	println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth);
	    	println!("\nEmail address:{}",email_address);
		    println!("\nPhone Number:{}",phone_number);
		    println!("\nNumber of Siblings:{}",number_siblings);
		    println!("\nNumber of Children:{}",number_children);
		    println!("\nMedical diagnosis:{}",medical_diagnosis);
		    println!("\nVillage of Residence:{}",village_of_residence);
        count+=1;
        }
        else if medical_diagnosis == "Arthtitis" && age > 40 && number_children > 5 && number_siblings > 5 && village_of_residence == "Emeremen"
        {
	        println!("Congratulations you have recived a 10% discount. Your new amount is {}", 0.1 * 450_000);
	        println!("\nName:{}",patient_name);
	    	println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth;
	    	println!("\nEmail address:{}",email_address);
		    println!("\nPhone Number:{}",phone_number);
		    println!("\nNumber of Siblings:{}",number_siblings);
		    println!("\nNumber of Children:{}",number_children);
		    println!("\nMedical diagnosis:{}",medical_diagnosis);
		    println!("\nVillage of Residence:{}",village_of_residence);
        count+=1;
        }
        else 
        {
        	println!("Normal Charges apply, you receive no discount.");
        	println!("\nName:{}",patient_name);
    	    println!("\nDate of Birth:{}/{}/{}",day_ofbirth,month_ofbirth,year_ofbirth);
    	    println!("\nEmail address:{}",email_address);
	    	println!("\nPhone Number:{}",phone_number);
	    	println!("\nNumber of Siblings:{}",number_siblings);
	    	println!("\nNumber of Children:{}",number_children);
	    	println!("\nMedical diagnosis:{}",medical_diagnosis);
	    	println!("\nVillage of Residence:{}",village_of_residence);
        	count+=1;
        }
        

        }   
 }