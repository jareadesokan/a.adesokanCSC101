fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:i32 = 5;

	let a = p * (1.0 + (r / 100.0)).powi(n);
	
	let ci = a - p;
    println!("Compound Interest is{}",ci);
}