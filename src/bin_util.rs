pub fn str_to_bin(input: String) -> String {
	let mut output = "".to_string();
	for c in input.clone().into_bytes() {
		let input = format!("0{:b}", c);
		output += &format!("{:0>8}", input);
	}
	output + "1"
}

pub fn bin_rot(inp: &[char], bits_to_rotate: usize) -> Vec<char> {
	let mut res = inp.to_vec();
	res.rotate_right(bits_to_rotate);
	res
}

pub fn bin_xor(inp1: &[char], inp2: &[char]) -> Vec<char> {
	inp1.iter()
		.zip(inp2.iter())
		.map(|(i_1, i_2)| ((i_1 != i_2) as i32).to_string().pop().unwrap())
		.collect()
}

pub fn bin_and(inp1: &[char], inp2: &[char]) -> Vec<char> {
	inp1.iter()
		.zip(inp2.iter())
		.map(|(i_1, i_2)| ((*i_1 == '1' && i_1 == i_2) as i32).to_string().pop().unwrap())
		.collect()
}


pub fn bin_add(inp1: &[char], inp2: &[char]) -> Vec<char> {
	let inp1_int = bin_to_decimal(inp1);
	let inp2_int = bin_to_decimal(inp2);

	let mut sum_bin = format!("{:b}", inp1_int + inp2_int);
	while sum_bin.len() > 32 {
		sum_bin.remove(0);
	}
	sum_bin = format!("{:0>32}", sum_bin);
	sum_bin.chars().collect()
}

pub fn bin_not(inp: &[char]) -> Vec<char> {
	inp.into_iter()
		.map(|i| (!(i.to_string().parse::<i32>().unwrap() == 1) as i32).to_string().pop().unwrap())
		.collect()
}

pub fn bin_to_decimal(inp: &[char]) -> i128 {
	let inp:String = inp.into_iter().collect();
	let inp_as_str = inp.trim();
	if inp_as_str.is_empty() {
		return 0;
	}
	isize::from_str_radix(&inp_as_str.trim(), 2).unwrap() as i128
}


// DEBUG
#[allow(dead_code)]
pub fn dump_vec(inp: &[char]) {
	println!("{}", inp.into_iter().collect::<String>());
}
