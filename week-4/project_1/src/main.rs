use std::io;
fn main() {
    let mut dist = 80.0;
   let mut time = 2.0;
   let mut dist_km = dist / 0.621;

   let mut speed = dist_km * time;
   println!("The speed of the car is {}", speed);

   dist = 120.0;
   time = 4.0;
   dist_km = dist / 0.621;

   speed = dist_km * time;
   println!("The new speed of the car is {}", speed);

   let mut dist = String::new();
   let mut time = String::new();

   println!("Enter the distance covered by the car");
   io::stdin().read_line(&mut dist).expect("Not a valid string");
   let a:f32 = dist.trim().parse().expect("Not a valid number");

   println!("Enter the time taken by the car");
   io::stdin().read_line(&mut time).expect("Not a valid string");
   let time:f32 = time.trim().parse().expect("Not a valid number");

   let dist_km:f32 = a / 0.621;
   let speed:f32 = dist_km * time;

   println!("The speed of the car is {}", speed);
}

