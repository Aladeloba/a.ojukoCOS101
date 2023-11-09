use std::io;

fn main() {
        println!("Input the value of a");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f32 = input1.trim().parse().expect("Failed to read input");

        println!("Input the value of b");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let b:f32 = input2.trim().parse().expect("Failed to read input");

        println!("Input the value of c");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let c:f32 = input3.trim().parse().expect("Failed to read input");

        let a_1:f32 = b.powf(2.0);
        let a_2:f32 = a_1 - (4.0 * a * c);
        let a_3:f32 = a_2.sqrt();

        let x_1:f32 = 2.0 * a;
        let x:f32 = (-b + a_3) / x_1;

        let y:f32 = (-b - a_3) / x_1;

        println!("roots = {},{}", x,y);

        let dis = a_1 - (4.0 * a * c);
        println!("Discriminant = {}", dis);

        if dis > 0.0 {
            println!("Since discriminant = {}, there are two distinct roots", dis);
        }

        else if dis == 0.0 {
            println!("Since determinant = {}, there is exactly one root", dis);
        }

        else if dis < 0.0 {
            println!("Since discriminant = {}, there are no real roots", dis);
        }
        else {
            println!("ERROR!");
        }
}
