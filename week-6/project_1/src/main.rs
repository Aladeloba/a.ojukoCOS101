use std::io;

fn main() {
    // CRITERIA FOR EXECUTION
    println!("Input the number you were given by the School");
    let mut eligible_candidates = String::new();
    io::stdin().read_line(&mut eligible_candidates).expect("Failed to read Input");
    let eligible_candidates:i32 = eligible_candidates.trim().parse().expect("Failed to read input");
    
    if eligible_candidates <= 150 {


        // INPUT FUNCTION TO REQUEST FOR NAME
        println!("Welcome to the Student Council Voter System! \nPlease enter Name");
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).expect("Failed to read Input");

        // INPUT FUNCTION TO REQUEST FOR EMAIL
        println!("Please enter your official email address");
        let mut email_address = String::new();
        io::stdin().read_line(&mut email_address).expect("Failed to read input");

        // INPUT FUNCTION TO REQUEST FOR DEPARTMENT
        println!("Please enter your department");
        let mut dept = String::new();
        io::stdin().read_line(&mut dept).expect("Failed to read input");

        // INPUT FUNCTION TO REQUEST FOR STATE OF ORIGIN
        println!("Please enter your state of origin");
        let mut state = String::new();
        io::stdin().read_line(&mut state).expect("Failed to read input");

        // INPUT FUNCTION TO VALIDATE CLASS REP
        println!("Are you a current Class Rep? (Input 1 for YES and 2 for NO)");
        let mut class_rep = String::new();
        io::stdin().read_line(&mut class_rep).expect("Failed to read Input");
        let class_rep:i32 = class_rep.trim().parse().expect("Failed to read input");

        // INPUT FUNCTION TO VALIDATE LEVEL
        println!("Are you in 100level? (Input 1 for YES and 2 for NO)");
        let mut level = String::new();
        io::stdin().read_line(&mut level).expect("Failed to read Input");
        let level:i32 = level.trim().parse().expect("Failed to read input");

        // INPUT FUNCTION TO VALIDATE CGPA
        println!("What is your CGPA?");
        let mut cgpa = String::new();
        io::stdin().read_line(&mut cgpa).expect("Failed to read Input");
        let cgpa:f32 = cgpa.trim().parse().expect("Failed to read input");


            if class_rep == 1 && level == 2 && cgpa > 4.0 {
                println!("\n{} \n{} \n{} \n{} \nYou can vote", input_name,email_address,dept,state);
            }

            else {
                println!("Sorry, you are not eligible to vote");
            }


    }
    else {
        println!("Sorry, the spaces have been filled up. Try again next year!");
    }
}
