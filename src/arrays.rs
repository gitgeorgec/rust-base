use std::mem;

pub fn run() {
	let mut numbers: [i32; 5] = [1,2,3,4,5];

	// re-assign value
	numbers[2] = 20;

	println!("{:?}", numbers);
	println!("single value: {}", numbers[0]);
	println!("array length: {}", numbers.len());

	println!("array occupoes {} bytes", mem::size_of_val(&numbers));

	// get slice

	let slice: &[i32] = &numbers[0..2];
	println!("slice: {:?}", slice);
}
