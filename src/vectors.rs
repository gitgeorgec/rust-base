use std::mem;

pub fn run() {
	let mut numbers: Vec<i32> = vec![1,2,3,4,5];

	// re-assign value
	numbers[2] = 20;


	// add on to vector
	numbers.push(4);
	numbers.push(6);

	println!("{:?}", numbers);
	//
	numbers.pop();

	println!("{:?}", numbers);
	println!("single value: {}", numbers[0]);
	println!("Vector length: {}", numbers.len());

	println!("Vector occupoes {} bytes", mem::size_of_val(&numbers));

	// get slice

	let slice: &[i32] = &numbers[0..2];
	println!("slice: {:?}", slice);

	// loop through vector values
	for x in numbers.iter() {
		println!("Number: {}", x);
	}

	// loop and mutate values
	for x in numbers.iter_mut() {
		*x *=2;
	}

	println!("Numbers Vec: {:?}", numbers);
}
