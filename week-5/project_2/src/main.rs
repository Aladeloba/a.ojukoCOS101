use std::io;

fn main() {
    loop {
    println!("Input your Experience Level (Experienced/Inexperienced): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();


 if experience == "experienced" || experience == "inexperienced" {
    println!("Input your age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Failed to read input"); 

    if experience == "experienced" {
    if age >= 40{
    println!("Your incentive is N1,560,000");
}

else if age >= 30 && age < 40 {
    println!("Your incentive is N1,480,000");
 }

else if age < 30 {
    println!("Your incentive is N1,300,000");

}
}

else {
    println!("Your incentive is N100,000");
}
break;
}

else {
    println!("ERROR!");
    }
}

}
