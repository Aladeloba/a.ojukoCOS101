use std::io;

fn main() {
    // Get the customer's name
    println!("Welcome to the restaurant, please input your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // Initialize variables to track the cost of each menu item
    let mut p_cost = 0;
    let mut f_cost = 0;
    let mut a_cost = 0;
    let mut e_cost = 0;
    let mut w_cost = 0;

    // Main loop for taking orders
    loop {
        // Prompt the user to see the menu or cancel the order
        println!("Hello, {}Please press 1 to see the menu and press 2 to cancel your order", name);
        let mut input1_str = String::new();
        io::stdin().read_line(&mut input1_str).expect("Failed to read input");
        let input1:i32 = input1_str.trim().parse().expect("Failed to read input");

        if input1 == 1 {
            // Display the menu

            // Loop for handling the menu selection and portions
            loop {
                println!("Menu items: \n1 = Poundo Yam/Edinkaiko Soup                N3,200
                        \n2 = Fried Rice & chicken                     N3,000
                        \n3 = Amala & Ewedu Soup                       N2,500
                        \n4 = Eba & Egusi Soup                         N2,000
                        \n5 = White Rice & Stew                        N2,500
                        \nPlease select an option or press 6 to cancel your order");

                // Take user input for menu selection
                let mut input2_str = String::new();
                io::stdin().read_line(&mut input2_str).expect("Failed to read input");
                let input2:i32 = input2_str.trim().parse().expect("Failed to read input");

                if (1..=5).contains(&input2) {
                    // If the menu selection is valid, prompt for the number of portions

                    // Loop for handling the number of portions
                    loop {
                        println!("How many portions would you like to have?");
                        let mut input3_str = String::new();
                        io::stdin().read_line(&mut input3_str).expect("Failed to read input");
                        let input3: i32 = input3_str.trim().parse().expect("Failed to read input");

                        if input3 <= 10 {
                            // Calculate the cost based on the menu selection and portions

                            // Loop to calculate the cost based on the menu selection
                            loop {
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

                                // Calculate the total bill and apply a discount if applicable
                                let bill = p_cost + f_cost + a_cost + e_cost + w_cost;
                                let new_bill = if bill > 10000 {
                                    0.95 * bill as f32
                                } else {
                                    bill as f32
                                };

                                // Loop for asking the user if they want to get the bill or continue ordering
                                loop {
                                    println!("Would that be all? Press 1 to get your bill or 2 to continue order");

                                    let mut input5_str = String::new();
                                    io::stdin().read_line(&mut input5_str).expect("Failed to read input");
                                    let input5: i32 = input5_str.trim().parse().expect("Failed to read input");

                                    if input5 == 1 {
                                        // Display the total bill and exit the loop
                                        println!("Total Bill is {:.2}", new_bill);
                                        // Exit the current loop (loop for asking bill or continuing order)
                                        break;
                                    } else if input5 == 2 {
                                        // Exit the current loop (loop for calculating the bill based on portions)
                                        break;
                                    } else {
                                        // Handle invalid input
                                        println!("Invalid Input");
                                        // Exit the current loop (loop for asking bill or continuing order)
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Inform the user about the maximum order limit and prompt for valid portions
                            println!("Sorry, you can order a maximum of 10 portions");
                            println!("Please select portions between 1 and 10");
                        }
                    }
                } else if input2 == 6 {
                    // Cancel the order and exit the loop
                    println!("Thank you! Have a nice day");
                    // Exit the current loop (loop for handling the menu selection and portions)
                    return;
                } else {
                    // Handle invalid menu selection
                    println!("Invalid choice.");
                    // Exit the current loop (loop for handling the menu selection and portions)
                    break;
                }
            }
        } else if input1 == 2 {
            // Cancel the order and exit the loop
            println!("Thank you! Have a nice day");
            // Exit the main loop (loop for taking orders)
            break;
        } else {
            // Handle invalid input for the main menu
            println!("Invalid choice.");
            // Continue to the next iteration of the main loop
            continue;
        }
    }
}
