use std::io;

fn main() {
    println!("Welcome to the restaurant, please input your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let mut p_cost = 0;
    let mut f_cost = 0;
    let mut a_cost = 0;
    let mut e_cost = 0;
    let mut w_cost = 0;

    loop {
        println!("Hello, {}Please press 1 to see the menu and press 2 to cancel your order", name);
        let mut input1_str = String::new();
        io::stdin().read_line(&mut input1_str).expect("Failed to read input");
        let input1: i32 = input1_str.trim().parse().expect("Failed to read input");

        if input1 == 1 {
            println!("Menu items: \n1 = Poundo Yam/Edinkaiko Soup                N3,200
                    \n2 = Fried Rice & chicken                     N3,000
                    \n3 = Amala & Ewedu Soup                       N2,500
                    \n4 = Eba & Egusi Soup                         N2,000
                    \n5 = White Rice & Stew                        N2,500
                    \nPlease select an option or press 6 to cancel your order");

            let mut input2_str = String::new();
            io::stdin().read_line(&mut input2_str).expect("Failed to read input");
            let input2: i32 = input2_str.trim().parse().expect("Failed to read input");

            if (1..=5).contains(&input2) {
                println!("How many portions would you like to have?");
                let mut input3_str = String::new();
                io::stdin().read_line(&mut input3_str).expect("Failed to read input");
                let input3: i32 = input3_str.trim().parse().expect("Failed to read input");

                if input3 <= 10 {
                    let p: i32 = 3200;
                    let f: i32 = 3000;
                    let a: i32 = 2500;
                    let e: i32 = 2000;
                    let w: i32 = 2500;

                    if input2 == 1 {
                        p_cost = input3 * p;
                    } else if input2 == 2 {
                        f_cost = input3 * f;
                    } else if input2 == 3 {
                        a_cost = input3 * a;
                    } else if input2 == 4 {
                        e_cost = input3 * e;
                    } else if input2 == 5 {
                        w_cost = input3 * w;
                    }

                    let bill = p_cost + f_cost + a_cost + e_cost + w_cost;
                    let new_bill = if bill > 10000 {
                        0.95 * bill as f32
                    } else {
                        bill as f32
                    };

                    println!("Would that be all? Press 1 to get your bill or 2 to continue order");

                    let mut input5_str = String::new();
                    io::stdin().read_line(&mut input5_str).expect("Failed to read input");
                    let input5: i32 = input5_str.trim().parse().expect("Failed to read input");

                    if input5 == 1 {
                        println!("Total Bill is {:.2}", new_bill);
                        break;
                    } else if input5 == 2 {
                        continue;
                    } else {
                        println!("Invalid Input");
                        break;
                    }
                } else {
                    println!("Sorry, you can order a maximum of 10 portions");
                    println!("Please select portions between 1 and 10");
                }
            } else if input2 == 6 {
                println!("Thank you! Have a nice day");
                break;
            } else {
                println!("Invalid choice.");
                continue;
            }
        } else if input1 == 2 {
            println!("Thank you! Have a nice day");
            break;
        } else {
            println!("Invalid choice.");
            continue;
        }
    }
}
