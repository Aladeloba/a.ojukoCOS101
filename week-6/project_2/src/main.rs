use std::io;

fn main() {
  // CRITERIA FOR EXECUTION
  println!("Please input your Researcher Number");
  let mut eligible_researchers = String::new();
  io::stdin().read_line(&mut eligible_researchers).expect("Failed to read input");
  let eligible_researchers:i32 = eligible_researchers.trim().parse().expect("Failed to read input");

  if eligible_researchers <= 500 {

    // DECLARATION OF INCENTIVE VALUES
    let incentive1:f32 = 500000.00;
    let incentive2:f32 = 800000.00;
    let incentive3:f32 = 1000000.00;
    let incentive4:f32 = 100000.00;

    // INPUT FUNCTION TO REQUEST FOR NAME
    println!("Welcome to The Researchers Incentive System. \nPlease Enter Name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    // INPUT FUNCTION TO REQUEST FOR NUMBER OF PAPERS PUBLISHED
    println!("Please input number of papers published");
    let mut papers_published = String::new();
    io::stdin().read_line(&mut papers_published).expect("Failed to read input");
    let papers_published:i32 = papers_published.trim().parse().expect("Failed to read input");


    if (3..=5).contains(&papers_published) {
        println!("{}, your incentive is {}", name,incentive1);
    }

    else if (6..=9).contains(&papers_published) {
        println!("\n{}, your incentive is {}", name,incentive2);
    }

    else if papers_published >= 10 {
        println!("\n{}, your incentive is {}", name,incentive3);
    }

    else {
        println!("\n{}, your incentive is {}", name,incentive4);
    }


  }

  else {
    println!("Sorry, the slots have been filled up.");
  }


}
