use std::io;

fn main() {
    let mut people_info: Vec<(String, u32, String, String, i32)> = Vec::new();

    let mut highest_experience: Option<usize> = None;

    println!("Hello Admin, how many people are we interviewing today?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:i32 = input1.trim().parse().expect("Invalid input");

    for number in 1..=input1{

    println!("Send person number {} in", number);

    // Ask for name
    println!("\nWelcome, Please input your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string();

    // Ask for age
    println!("Hello {}, please input age", name);
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:u32 = age.trim().parse().expect("Invalid Input");

    // Ask for gender
    println!("Please input Gender");
    let mut gender = String::new();
    io::stdin().read_line(&mut gender).expect("Failed to read input");
    let gender = gender.trim().to_string();

    // Ask for degree class
    println!("Please input your degree class");
    let mut degree_class = String::new();
    io::stdin().read_line(&mut degree_class).expect("Failed to read input");
    let degree_class = degree_class.trim().to_string();

    // Ask for experience years
    println!("Please input programming experience years");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience:i32 = experience.trim().parse().expect("Invalid Input");

    println!("Thank you");

    let person_info = (name,age,gender,degree_class,experience);
    people_info.push(person_info);

    //For the frist person interviewed, it saves the experience years then for every other person, then checks to see if their experience years is greater than the original person
    
    if let Some(index) = highest_experience {
        if experience > people_info[index].4 {
            highest_experience = Some(people_info.len() - 1);
            }
        }
        else{
            highest_experience =  Some(people_info.len() - 1);
        }
    

    }

    // If i wanted it to print out each person's information
    /*    
    for element in &people_info{
        println!("{:?}", element);
     } 
    */

    if let Some(index) = highest_experience {
        let highest_experience = &people_info[index];
        print!("\nPerson With Highest Experience: \nName: {} \nAge: {} \nGender: {} \nDegree Class: {} \nExperience Years: {}", highest_experience.0,highest_experience.1,highest_experience.2,highest_experience.3,highest_experience.4);
    }    
}
