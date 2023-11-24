use std::io;

fn main() {
    println!("Welcome!
            \nInput number of siblings");
    let mut no_of_siblings = String::new();
    io::stdin().read_line(&mut no_of_siblings).expect("Failed to read input");
    let no_of_siblings:usize = no_of_siblings.trim().parse().expect("Failed to read input");

    let mut names = vec![];
    let mut ages: Vec<u32> = vec![];
    let mut status = vec!["n/a"; no_of_siblings];
    let mut job = vec!["n/a"; no_of_siblings];
    let mut uni = vec!["n/a".to_string(); no_of_siblings];
    let mut course = vec!["n/a".to_string(); no_of_siblings];
    let mut offspring: Vec<u32> = vec![];
    let mut city = vec!["n/a".to_string(); no_of_siblings];
    let mut waec = vec!["n/a"; no_of_siblings];
    let mut notwaec = vec!["n/a"; no_of_siblings];
    let mut school = vec!["n/a".to_string(); no_of_siblings];
    let mut class = vec!["n/a".to_string(); no_of_siblings];
    

    for number in 0..no_of_siblings {
        println!("\nPLEASE ENTER DETAILS FOR SIBLING {}
                \nPlease Input Name of Sibling {}", number + 1,number + 1);
        let mut sibling_name = String::new();
        io::stdin().read_line(&mut sibling_name).expect("Failed to read input");
        sibling_name = sibling_name.trim().to_string();
        let name = sibling_name.trim().to_string();
        names.push(name);

        println!("Please Input {}'s age", sibling_name);
        let mut sibling_age = String::new();
        io::stdin().read_line(&mut sibling_age).expect("Failed to read input");
        let sibling_age:u32 = sibling_age.trim().parse().expect("Failed to read input");
        ages.push(sibling_age);

        if sibling_age >= 18{
            {
                println!("Is {} married or single? (Input 1 for married or 2 for single)", sibling_name);
                let mut sibling_status = String::new();
                io::stdin().read_line(&mut sibling_status).expect("Failed to read input");
                let sibling_status:i32 = sibling_status.trim().parse().expect("Failed to read input");

                if sibling_status == 2 {
                    status[number] = "is single";
                    println!("Is {} a student or a worker? (Input 1 for student or 2 for worker)", sibling_name);
                    let mut sibling_job = String::new();
                    io::stdin().read_line(&mut sibling_job).expect("Failed to read input");
                    let sibling_job:i32 = sibling_job.trim().parse().expect("Failed to read input");
                    

                    if sibling_job == 1 {
                        job[number] = "student";
                        println!("What University does {} attend?", sibling_name);
                        let mut sibling_uni = String::new();
                        io::stdin().read_line(&mut sibling_uni).expect("Failed to read input");
                        let university = sibling_uni.trim().to_string();
                        uni[number] = university;
                        
                        println!("What is {}'s course of study?", sibling_name);
                        let mut sibling_course = String::new();
                        io::stdin().read_line(&mut sibling_course).expect("Failed to read input");
                        sibling_course = sibling_course.trim().to_string();
                        course[number]= sibling_course;
                        
                    
                    }else {
                        job[number] = "worker";
                    }


                }


                else if sibling_status == 1 {
                    status[number] = "is married";
                    println!("How many offspring's does {} have?", sibling_name);
                    let mut sibling_offspring = String::new();
                    io::stdin().read_line(&mut sibling_offspring).expect("Failed to read input");
                    let sibling_offspring:u32 = sibling_offspring.trim().parse().expect("Failed to read input");
                    offspring.push(sibling_offspring);

                    println!("What city does {}'s family live in?", sibling_name);
                    let mut sibling_city = String::new();
                    io::stdin().read_line(&mut sibling_city).expect("Failed to read input");
                    let sibling_city = sibling_city.trim().to_string();
                    city[number] = sibling_city;
                }
                else {
                    println!("Invalid Input, please select 1 or 2");
            continue;
                }
            }
        }

        else if sibling_age < 18 {
            loop{
                println!("What is {}'s WAEC status? (Input 1 for Yes and 2 for No)", sibling_name);
                let mut sibling_waec = String::new();
                io::stdin().read_line(&mut sibling_waec).expect("Failed to read input");
                let sibling_waec:i32 = sibling_waec.trim().parse().expect("Failed to read input");
                

                if sibling_waec == 1 {
                    println!("What Secondary School does {} attend?", sibling_name);
                    let mut sibling_school = String::new();
                    io::stdin().read_line(&mut sibling_school).expect("Failed to read input");
                    let sibling_school = sibling_school.trim().to_string();
                    school[number] = sibling_school;
                    waec[number] = "has completed WAEC";

                    break;
                   
                }

                else if sibling_waec == 2 {
                    println!("What class is {} in?", sibling_name);
                    let mut sibling_class = String::new();
                    io::stdin().read_line(&mut sibling_class).expect("Failed to read input");
                    let sibling_class = sibling_class.trim().to_string();
                    class[number] = sibling_class;
                    notwaec[number] = "has not completed WAEC";

                    break;
                }

                else {
                    println!("Invalid Input");
            continue;
                }
            }


        }

    } 

    for element in 0..no_of_siblings {
        if ages[element] >= 18 {
            if status[element] == "is married" {
                println!("{} is {} and {} and has {} children and lives in {}.", names[element],ages[element],status[element],offspring[element],city[element]);
            }
            else if status[element] == "is single" {
                if job[element] == "student" {
                println!("{} is {} and {} and is a {} of {}, studying {}.", names[element],ages[element],status[element],job[element],uni[element],course[element]);
                }
                else if job[element] == "worker"{
                println!("{} is {} and {} and is a {}.", names[element],ages[element],status[element],job[element]);    
                }
            }
        }
        else if ages[element] < 18 {
            if waec[element] == "has completed WAEC" {
                println!("{} is {} and {} and is in {}.", names[element],ages[element],waec[element],school[element]);
            }
            else if notwaec[element] == "has not completed WAEC" {
                println!("{} is {} and {} and is in {}.", names[element],ages[element],notwaec[element],class[element]);
            }
        }
    }
}
