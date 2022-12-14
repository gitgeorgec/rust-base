pub fn run() {
	let name = "jack";
	let mut age = 30;
	println!("My name is {} and I am {}", name, age);
	age = 31;
	println!("My name is {} and I am {}", name, age);

	const ID: i32 = 001;
	println!("ID: {}", ID);

	let ( my_name, my_age) = ("jack", 32);
	println!("{} is {}", my_name, my_age);

}
