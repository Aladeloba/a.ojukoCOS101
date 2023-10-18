fn main() {
	let T = 450_000.00;
	let M = 1_500_000.00;
	let H = 750_000.00;
	let D = 2_850_000.00;
	let A = 250_000.00;
	let nT = 2.0;
	let nM = 1.0;
	let nH = 3.0;
	let nD = 3.0;
	let nA = 1.0;

	//sum
	let s = (T * nT) + (M * nM) + (H * nH) + (D * nD) + (A * nA);
	println!("Sum is {}", s);

	//Average
	let A = s/(nT + nM + nH + nD + nA);
	println!("Average is {}", A);

}