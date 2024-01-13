use std::io;
use std::io::Write;

// Create a Struct to represent Organizatioin
struct Organization {
    company:String,
    date:i32,
    assets:f32,
    liabilities:f32,
    percent_lev:Option<f32>

}

// Implement method for Organization struct
impl Organization {
    fn percent_lev(&self) -> Option<f32> {
        self.percent_lev.or_else(|| {
        Some(((self.assets - self.liabilities) * 100.0) / self.assets)
    })        
    }
}

// Function to check if password is valid
fn is_valid_password(password: &str) -> bool {

    let characters = ['#', '$', '%', '@', '!', '^', '_', '-', '+', '>', '<', '/', '?'];

    !password.chars().any(|ch| characters.contains(&ch) || ch.is_ascii_uppercase())

}

fn main() {

    let vec_org1 = ("cadb".to_string(), "Cadbury Nigeria Plc".to_string(), 15_000_000.0, 5_500_000.0);
    let vec_org2 = ("cham".to_string(), "Champion Breweries Plc".to_string(), 25_000_000.0, 8_000_000.0);
    let vec_org3 = ("dang".to_string(), "Dangote Sugar Refinery Plc".to_string(), 18_000_000.0, 10_000_000.0);
    let vec_org4 = ("flou".to_string(), "Flour Mills Nigeria Plc".to_string(), 32_000_000.0, 4_000_000.0);
    let vec_org5 = ("nest".to_string(), "Nestle Nigeria Plc".to_string(), 8_000_000.0, 1_500_000.0);
    let vec_org6 = ("unil".to_string(), "Unilever Nigeria Plc".to_string(), 37_000_000.0, 11_000_000.0);
    let vec_org7 = ("hone".to_string(), "Honeywell Nigeria Plc".to_string(), 34_000_000.0, 9_000_000.0);
    let vec_org8 = ("nige".to_string(), "Nigerian Breweries Plc".to_string(), 30_000_000.0, 12_000_000.0);

    // Get username & password from user
    loop{ 
        let mut username = String::new();
        println!("Welcome, Please input valid username");
        io::stdin().read_line(&mut username).expect("Invalid Input");
        let username:String = username.trim().to_lowercase();

        // Check if the username is valid
        if username == vec_org1.0 || username == vec_org2.0 || username == vec_org3.0 || username == vec_org4.0 || username == vec_org5.0 || username == vec_org6.0 || username == vec_org7.0 || username == vec_org8.0 {
        
            // Validate the password
            loop{ 
                let mut password = String::new();
                println!("Please, enter the password");
                io::stdin().read_line(&mut password).expect("Invalid Input");

                let password = password.trim();

                // Check if the password is valid
                if password.len() >= 3  && password.len() <= 8 && is_valid_password(&password) {
                    // Display company information based on the username
                    if username == vec_org1.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org1.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    else if username == vec_org2.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org2.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    if username == vec_org3.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org3.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    if username == vec_org4.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org4.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    if username == vec_org5.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org5.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    else if username == vec_org6.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org6.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    else if username == vec_org7.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org7.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                    else if username == vec_org8.0 {
                        println!("\nCompany's name: {} \nCompany's Shares: {} \nCompany's Liabilities is: {}", vec_org8.1,vec_org1.2,vec_org1.3);
                        break;
                    }
                }

                else {
                    println!("Invalid Password, Please try again");
                    continue;
                }
            }   

            break;
        }

        else {
            continue;
        }
    }

    // Creating a vector for the struct
    let mut all_companies: Vec<Organization> = Vec::new();

    // Initializing the struct 
    let mut company_1 = Organization {
        company:String::from("Cadbury Nigeria Plc"),
        date:1965,
        assets:15_000_000.0,
        liabilities:5_500_000.0,
        percent_lev: None
    };
    company_1.percent_lev = company_1.percent_lev();
    all_companies.push(company_1);

    let mut company_2 = Organization {
        company:String::from("Champion Breweries Plc"),
        date:1974,
        assets:25_000_000.0,
        liabilities:8_000_000.0,
        percent_lev:None
    };
    company_2.percent_lev = company_2.percent_lev();
    all_companies.push(company_2);

    let mut company_3 = Organization {
        company:String::from("Dangote Sugar Refinery"),
        date:1970,
        assets:18_000_000.0,
        liabilities:10_000_000.0,
        percent_lev:None
    };
    company_3.percent_lev = company_3.percent_lev();
    all_companies.push(company_3);

    let mut company_4 = Organization {
        company:String::from("Flour Mills Nigeria Plc"),
        date:1960,
        assets:32_000_000.0,
        liabilities:4_000_000.0,
        percent_lev:None
    };
    company_4.percent_lev = company_4.percent_lev();
    all_companies.push(company_4);

    let mut company_5 = Organization {
        company:String::from("Nestle Nigeria Plc"),
        date:1961,
        assets:8_000_000.0 ,
        liabilities:1_500_000.0,
        percent_lev:None
    };
    company_5.percent_lev = company_5.percent_lev();
    all_companies.push(company_5);

    let mut company_6 = Organization {
        company:String::from("Unilever Nigeria Plc"),
        date:1923,
        assets:37_000_000.0,
        liabilities:11_000_000.0,
        percent_lev:None
    };
    company_6.percent_lev = company_6.percent_lev();
    all_companies.push(company_6);

    let mut company_7 = Organization {
        company:String::from("Honeywell Nigeria Plc"),
        date:1906,
        assets:34_000_000.0,
        liabilities:9_000_000.0,
        percent_lev:None
    };
    company_7.percent_lev = company_7.percent_lev();
    all_companies.push(company_7);

    let mut company_8 = Organization {
        company:String::from("Nigerian Breweries Plc"),
        date:1946,
        assets:30_000_000.0,
        liabilities:12_000_000.0,
        percent_lev:None
    };
    company_8.percent_lev = company_8.percent_lev();
    all_companies.push(company_8);

    // Create a file to write company information
    let mut file1 = std::fs::File::create("Companies Information.txt").expect("Failed to create file");

    // Header information for the file
    let header = vec!["Company", "Leverages(%)", "Date", "Assets", "Liabilities"];

    // Write header information to file
    for item in &header {
        file1.write_all(item.as_bytes()).expect("Failed to write to file");
        file1.write_all(b"\t\t").expect("Failed to write to file")
    }
        file1.write_all(b"\n\n").expect("Failed to write to file");

    // Write company details to the file
    for comp in &all_companies {
        file1.write_all(comp.company.as_bytes()).expect("Failed to write to file");
        file1.write_all(b"\t\t").expect("Failed to write to file");

        if let Some(percent_lev) = comp.percent_lev() {
            let format_percent_lev = format!("{:.1}", percent_lev);
            file1.write_all(format_percent_lev.to_string().as_bytes()).expect("Failed to write to file");
        }
        else {
        
            file1.write_all("N/A".as_bytes()).expect("Failed to write to file");
        }

        file1.write_all(b"\t\t").expect("Failed to write to file");

        file1.write_all(comp.date.to_string().as_bytes()).expect("Failed to write to file");
        file1.write_all(b"\t\t").expect("Failed to write to file");

        file1.write_all(comp.assets.to_string().as_bytes()).expect("Failed to write to file");
        file1.write_all(b"\t\t").expect("Failed to write to file");

        file1.write_all(comp.liabilities.to_string().as_bytes()).expect("Failed to write to file");
        
        file1.write_all(b"\n").expect("Failed to write to file");
    }
        // Check and create file for companies with assets greater 20M
        if vec_org1.2 > 20_000_000.0 || vec_org2.2 > 20_000_000.0 || vec_org3.2 > 20_000_000.0 || vec_org4.2 > 20_000_000.0|| vec_org5.2 > 20_000_000.0 || vec_org6.2 > 20_000_000.0 || vec_org7.2 > 20_000_000.0 || vec_org8.2 > 20_000_000.0 { 
            
            let mut file2 = std::fs::File::create("Company's Share Info.txt").expect("Failed to create file");
            
            // Write information for companies with assets greater than 20M
            for comp in &all_companies {
                if comp.assets > 20_000_000.0 {
                    file2.write_all("Company Name: ".as_bytes()).expect("Failed to write to file");
                    file2.write_all(comp.company.as_bytes()).expect("Failed to write to file");
                    file2.write_all(b"\n").expect("Failed to write to file");

                    file2.write_all("Share Price: ".as_bytes()).expect("Failed to write to file");
                    file2.write_all(comp.assets.to_string().as_bytes()).expect("Failed to write to file");
                    file2.write_all(b"\n").expect("Failed to write to file");

                    file2.write_all("Percentage Leverage: ".as_bytes()).expect("Failed to write to file");
                    file2.write_all(comp.percent_lev().map(|p| format! ("{:.1}%", p)).unwrap_or("N/A".to_string()) .as_bytes()).expect("Failed to write to file");
                    file2.write_all(b"\n").expect("Failed to write to file");

                    file2.write_all(b"\n").expect("Failed to write to file");
                }
            }
        }

        // Check and create file for companies with liabilities less than 10M
        if vec_org1.3 < 10_000_000.0 || vec_org2.3 < 10_000_000.0 || vec_org3.3 < 10_000_000.0 || vec_org4.3 < 10_000_000.0|| vec_org5.3 < 10_000_000.0 || vec_org6.3 < 10_000_000.0 || vec_org7.3 < 10_000_000.0 || vec_org8.3 < 10_000_000.0 { 
            
            let mut file3 = std::fs::File::create("Company's Liability Info.txt").expect("Failed to create file");
            
            // Write information for companies with liabilities less than 10M    
            for comp in &all_companies {
                if comp.liabilities < 10_000_000.0 {
                    file3.write_all("Company Name: ".as_bytes()).expect("Failed to write to file");
                    file3.write_all(comp.company.as_bytes()).expect("Failed to write to file");
                    file3.write_all(b"\n").expect("Failed to write to file");

                    file3.write_all("Company Liabilities: ".as_bytes()).expect("Failed to write to file");
                    file3.write_all(comp.liabilities.to_string().as_bytes()).expect("Failed to write to file");
                    file3.write_all(b"\n").expect("Failed to write to file");

                    // Calculate 5% of Percentage Leverage
                    let five_percent_leverage = comp.percent_lev().map(|p| p * 0.05);

                    file3.write_all("5% of Percentage Leverage: ".as_bytes()).expect("Failed to write to file");
                    file3.write_all(five_percent_leverage.map(|p| format! ("{:.1}%", p)).unwrap_or("N/A".to_string()) .as_bytes()).expect("Failed to write to file");
                    file3.write_all(b"\n").expect("Failed to write to file");

                    file3.write_all(b"\n").expect("Failed to write to file");
                }
            }
        }                    
}

