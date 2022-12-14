pub fn run() {
	// Print to console
	println!("hello from print.rs file");

	println!("Number: {}", 1);

	println!("{} is from : {}", "jack", "US");

	println!("{0} is from : {1} and {0} likes to {2}", "jack", "US", "code");

	println!("{name} likes to {activity}", name = "jack", activity = "code");

	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	println!("{:?}", (12, true, "hello"));
}
