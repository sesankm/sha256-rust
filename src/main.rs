use std::env;
mod hash;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 as usize {
		panic!("ERROR: Expected an argument with length > 0 in quotes.\nExample: `cargo run \"this is an example\"`");
	}
	let input = &args[1];
	println!("{}", hash::hash(input.to_string()));
}
