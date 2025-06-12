mod k_constants;
mod bin_util;

fn init_message_block(input_bin: String) -> Vec<Vec<char>> {
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

fn fill_message_block(message_block: &mut Vec<Vec<char>>) {
	for i in 16..64 {
		let result = calc_next_word(i, message_block.to_vec());
		message_block.push(result);
	}
}

fn calc_next_word(curr_row: usize, message_block: Vec<Vec<char>>) -> Vec<char> {
	let x = message_block[curr_row - 16].clone();
	let y = message_block[curr_row - 7].clone();
	let sig_zero = calc_sig_zero(message_block[curr_row - 15].clone());
	let sig_one = calc_sig_one(message_block[curr_row - 2].clone());

	let mut s1 = bin_util::bin_add(x, y);
	s1 = bin_util::bin_add(s1, sig_zero);
	bin_util::bin_add(s1, sig_one)
}

fn calc_sig_zero(curr_row: Vec<char>) -> Vec<char> {
	let v1 = bin_util::bin_rot(curr_row.clone(), 7);
	let v2 = bin_util::bin_rot(curr_row.clone(), 18);
	let v3 = shift(curr_row, 3);

	let xor_res = bin_util::bin_xor(v1, v2);
	bin_util::bin_xor(xor_res, v3)
}

fn calc_sig_one(curr_row: Vec<char>) -> Vec<char> {
	let v1 = bin_util::bin_rot(curr_row.clone(), 17);
	let v2 = bin_util::bin_rot(curr_row.clone(), 19);
	let v3 = shift(curr_row, 10);

	let xor_res = bin_util::bin_xor(v1, v2);
	bin_util::bin_xor(xor_res, v3)
}

fn calc_final_hash(message_block: Vec<Vec<char>>) {
	let initial_hashes: Vec<Vec<char>> = calculate_initial_hashes();
	let mut a = initial_hashes[0].clone();
	let mut b = initial_hashes[1].clone();
	let mut c = initial_hashes[2].clone();
	let mut d = initial_hashes[3].clone();
	let mut e = initial_hashes[4].clone();
	let mut f = initial_hashes[5].clone();
	let mut g = initial_hashes[6].clone();
	let mut h = initial_hashes[7].clone();

	for curr_ind in 0..64 {
		let curr_row = message_block[curr_ind].clone();
		let sum_one = calc_sum_one(e.to_vec());
		let choice = calc_choice(e.clone(), f.clone(), g.clone());
		let k_vec = k_constants::K_CONSTANTS[curr_ind].chars().collect();

		let mut s1 = bin_util::bin_add(h, sum_one);
		s1 = bin_util::bin_add(s1.clone(), choice);
		s1 = bin_util::bin_add(s1.clone(), k_vec);
		let temp_one = bin_util::bin_add(s1.clone(), curr_row);

		let sum_zero = calc_sum_zero(a.clone());
		let majority = calc_majority(a.clone(), b.clone(), c.clone());
		let temp_two = bin_util::bin_add(sum_zero, majority);

		h = g;
		g = f;
		f = e;
		e = bin_util::bin_add(d, temp_one.clone());
		d = c;
		c = b;
		b = a;
		a = bin_util::bin_add(temp_one, temp_two);
	}

	let result_word_one   = bin_util::bin_add(a, initial_hashes[0].clone());
	let result_word_two   = bin_util::bin_add(b, initial_hashes[1].clone());
	let result_word_three = bin_util::bin_add(c, initial_hashes[2].clone());
	let result_word_four  = bin_util::bin_add(d, initial_hashes[3].clone());
	let result_word_five  = bin_util::bin_add(e, initial_hashes[4].clone());
	let result_word_six   = bin_util::bin_add(f, initial_hashes[5].clone());
	let result_word_seven = bin_util::bin_add(g, initial_hashes[6].clone());
	let result_word_eight = bin_util::bin_add(h, initial_hashes[7].clone());

	let seg_1 = to_hex(result_word_one.iter().collect::<String>(), 4);
	let seg_2 = to_hex(result_word_two.iter().collect::<String>(), 4);
	let seg_3 = to_hex(result_word_three.iter().collect::<String>(), 4);
	let seg_4 = to_hex(result_word_four.iter().collect::<String>(), 4);
	let seg_5 = to_hex(result_word_five.iter().collect::<String>(), 4);
	let seg_6 = to_hex(result_word_six.iter().collect::<String>(), 4);
	let seg_7 = to_hex(result_word_seven.iter().collect::<String>(), 4);
	let seg_8 = to_hex(result_word_eight.iter().collect::<String>(), 4);

	print!("{}{}{}{}{}{}{}{}", seg_1, seg_2, seg_3, seg_4, seg_5, seg_6, seg_7, seg_8);
}

fn to_hex(val: String, len: usize) -> String {
    let n: u32 = u32::from_str_radix(&val, 2).unwrap();
    format!("{:01$x}", n, len * 2)
}

fn calc_choice(e: Vec<char>, f: Vec<char>, g: Vec<char>) -> Vec<char> {
 	let not_e = bin_util::bin_not(e.to_vec());
 	let e_and_f = bin_util::bin_and(e.to_vec(), f);
 	let not_e_and_g = bin_util::bin_and(not_e, g);
 	bin_util::bin_xor(e_and_f, not_e_and_g)
}

fn calc_sum_one(input: Vec<char>) -> Vec<char> {
	let x = bin_util::bin_rot(input.clone(), 6);
	let y = bin_util::bin_rot(input.clone(), 11);
	let z = bin_util::bin_rot(input.clone(), 25);
	let result = bin_util::bin_xor(x, y);
	bin_util::bin_xor(result, z)
}


fn calc_sum_zero(input: Vec<char>) -> Vec<char> {
	let x = bin_util::bin_rot(input.clone(), 2);
	let y = bin_util::bin_rot(input.clone(), 13);
	let z = bin_util::bin_rot(input.clone(), 22);
	let result = bin_util::bin_xor(x, y);
	bin_util::bin_xor(result, z)
}

fn calc_majority(a: Vec<char>, b: Vec<char>, c: Vec<char>) -> Vec<char> {
	let x = bin_util::bin_and(a.clone(), b.clone());
	let y = bin_util::bin_and(a.clone(), c.clone());
	let z = bin_util::bin_and(b.clone(), c.clone());
	let result = bin_util::bin_xor(x, y);
	bin_util::bin_xor(result, z)
}

fn calculate_initial_hashes() -> Vec<Vec<char>> {
	let mut initial_hashes = Vec::new();
	let mut num_primes = 0;
	let mut i = 2;
	loop {
		if is_prime(i) {
			num_primes += 1;
			let sqrt = (i as f64).sqrt();
			let mut sqrt_string = sqrt.to_string();
			sqrt_string.remove(0);

			sqrt_string = sqrt_string.chars().collect();
			let dec_part: f64 = sqrt_string.parse().unwrap();

			let result_dec = ((dec_part * 4294967296.0 * 10000000000.0).round() / 10000000000.0) as i128;
			let mut result_bin = format!("{:b}", result_dec);
			result_bin = format!("{:0>32}", result_bin);
			result_bin = result_bin.chars().take(32).collect();
			let hash_vec:Vec<char> = result_bin.chars().collect();

			initial_hashes.push(hash_vec);
		}
		if num_primes >= 8 {
			break;
		}
		i += 1;
	}
	initial_hashes
}

fn is_prime(num: i32) -> bool {
	for i in 0..num {
		if num as f32 % i as f32 == 0.0 && i != 1 && i != num {
			return false;
		}
	}
	return true;
}

fn main() {
	let input = "abc123".to_string();
	let input_bin = bin_util::str_to_bin(input.clone());
	let mut message_block = init_message_block(input_bin);
	fill_message_block(&mut message_block);
	calculate_initial_hashes();
	calc_final_hash(message_block);
}
