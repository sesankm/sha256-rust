fn dump_message_block(message_block: Vec<Vec<char>>) {
	for row in message_block {
		for c in row {
			print!("{}", c);
		}
		println!();
	}
}

fn convert_input_to_binary(input: String) -> String {
	let mut output = "".to_string();
	for c in input.clone().into_bytes() {
		let input = format!("0{:b}", c);
		output += &format!("{:0>8}", input);
	}
	output + "1"
}

fn step_one(input_bin: String) -> Vec<Vec<char>> {
	let mut message_block = vec!['0'; 512];

	let input_len = format!("{:b}", input_bin.len() - 1);
	for (ind, val) in input_bin.chars().enumerate() {
		message_block[ind] = val;
	}

	let input_len_bin_len = input_len.len();
	for (ind, val) in input_len.chars().enumerate() {
		message_block[512 - input_len_bin_len - ind + 1] = val;
	}

	let vec_of_slices: Vec<&[char]> = message_block.chunks(32).collect();
	vec_of_slices.into_iter().map(|x| x.to_vec()).collect()
}

fn shift(curr_row: Vec<char>, positions_to_shift: usize) -> Vec<char> {
	let mut output = curr_row.to_vec();
	output.rotate_right(positions_to_shift);
	for i in 0..positions_to_shift {
		output[i] = '0';
	}
	output
}

fn sig_zero(curr_row: Vec<char>) -> Vec<char> {
	let mut v1 = curr_row.to_vec();
	v1.rotate_right(7);
	v1.clone().into_iter().map(|x| x as u32 - '0' as u32);

	let mut v2 = curr_row.to_vec();
	v2.rotate_right(18);
	v2.clone().into_iter().map(|x| x as u32 - '0' as u32);

	let v3 = shift(curr_row, 3);
	let mut result = Vec::new();

	for (ind, val) in v1.iter().enumerate() {
		let r1 = ((*val != v2[ind]) as i32).to_string().pop().unwrap();
		let r2 = ((r1 != v3[ind]) as i32).to_string().pop().unwrap();
		result.push(r2);
	}
	result
}

fn sig_one(curr_row: Vec<char>) -> Vec<char> {
	let mut v1 = curr_row.to_vec();
	v1.rotate_right(17);
	v1.clone().into_iter().map(|x| x as u32 - '0' as u32);

	let mut v2 = curr_row.to_vec();
	v2.rotate_right(19);
	v2.clone().into_iter().map(|x| x as u32 - '0' as u32);

	let v3 = shift(curr_row, 10);
	let mut result = Vec::new();

	for (ind, val) in v1.iter().enumerate() {
		let r1 = ((*val != v2[ind]) as i32).to_string().pop().unwrap();
		let r2 = ((r1 != v3[ind]) as i32).to_string().pop().unwrap();
		result.push(r2);
	}
	result
}

fn calc_next_word(curr_row: usize, message_block: Vec<Vec<char>>) -> Vec<char> {
	let x = message_block[curr_row - 16].clone();
	let y = message_block[curr_row - 7].clone();
	let sig_zero = sig_zero(message_block[curr_row - 15].clone());
	let sig_one = sig_one(message_block[curr_row - 2].clone());

	let x_str: String = x.into_iter().collect();
	let y_str: String = y.into_iter().collect();
	let sig_zero_str: String = sig_zero.into_iter().collect();
	let sig_one_str: String = sig_one.into_iter().collect();

	let x_int = i128::from_str_radix(&x_str, 2).unwrap();
	let y_int = i128::from_str_radix(&y_str, 2).unwrap();
	let sig_zero_int = i128::from_str_radix(&sig_zero_str, 2).unwrap();
	let sig_one_int = i128::from_str_radix(&sig_one_str, 2).unwrap();

	let result_dec = x_int + y_int + sig_zero_int + sig_one_int;
	let mut result_bin_str = format!("{:b}", result_dec);

	result_bin_str = format!("{:0>32}", result_bin_str);
	let mut result_bin_vec: Vec<char> = result_bin_str.chars().collect();

	while result_bin_vec.len() > 32 {
		result_bin_vec.remove(0);
	}

	result_bin_vec
}

fn main() {
	let mut input = "abc123".to_string();
	let input_bin = convert_input_to_binary(input.clone());
	let mut message_block = step_one(input_bin);
	let mut output = "".to_string();

	for i in 16..64 {
		let result = calc_next_word(i, message_block.clone());
		message_block.push(result);
	}

	//dump_message_block(message_block);
}
