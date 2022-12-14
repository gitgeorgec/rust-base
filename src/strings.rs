// Primitive str = Immutavle fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {
	// Primitive str
	let hello = "hello";
	println!("{}", hello);
	// String
	let mut world = String::from("world ");

	// get length
	println!("length: {}", world.len());

	world.push('W');
	println!("{}", world);

	world.push_str("wor");
	println!("{}", world);

	println!("is Empty: {}", world.is_empty());
	println!("contains 'world': {}", world.contains("world"));
	println!("replace: {}", world.replace("world", "hello"));
	println!("Capacity: {}", world.capacity());

	for word in world.split_whitespace() {
		println!("{}", word);
	}

	let mut s = String::with_capacity(10);

	s.push('a');
	s.push('b');

	// Assertion testing
	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());

	println!("{}", s);
}
