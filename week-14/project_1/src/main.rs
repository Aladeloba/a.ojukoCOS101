use std::io;
use std::io::Read;

fn main() {
    loop{
        println!("Welcome to the Globacom database. Please Input position.");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let input1 = input1.trim().to_lowercase();

            if input1 == "administrator" {
                administrator();
                break;
            }
            else if input1 == "project manager" {
                project_manager();
                break;
            }
            else if input1 == "employee" {
                employee();
                break;
            }
            else if input1 == "customer" {
                customer();
                break;
            }
            else if input1 == "vendor" {
                vendor();
                break;
            }
            else {
                println!("Invalid Input");
                continue;
            }
        }
    }

fn administrator() {
    let mut file1 = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut admin_content= String::new();
    file1.read_to_string(&mut admin_content).unwrap();
    print!("{}", admin_content);
}

fn project_manager() {
    let mut file1 = std::fs::File::open("project.sql").unwrap();
    let mut pm_content = String::new();
    file1.read_to_string(&mut pm_content).unwrap();
    print!("{}", pm_content);
}

fn employee() {
    let mut file3 = std::fs::File::open("staff.sql").unwrap();
    let mut emp_content = String::new();
    file3.read_to_string(&mut emp_content).unwrap();
    print!("{}", emp_content);
}

fn customer() {
    let mut file4 = std::fs::File::open("customer.sql").unwrap();
    let mut customer_content = String::new();
    file4.read_to_string(&mut customer_content).unwrap();
    print!("{}", customer_content);
}

fn vendor() {
    let mut file5 = std::fs::File::open("dataplan.sql").unwrap();
    let mut vend_content = String::new();
    file5.read_to_string(&mut vend_content).unwrap();
    print!("{}", vend_content);
}

