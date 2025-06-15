use std::env;

// after pushing input as 8 bit ints to message block, convert message block to vec of 32 bit words
fn convert_message_block_to_u32(message_block: &[u8]) -> Vec<u32> {
	let convert_to_u32 = message_block.chunks(4)
		.map(|chunk| chunk.to_vec().into_iter().map(|x| format!("{x:0>8b}")).collect::<Vec<String>>().join(""))
		.collect::<Vec<String>>();
	convert_to_u32.into_iter().map(|x| u32::from_str_radix(&x, 2).unwrap()).collect::<Vec<u32>>()
}

// initialize vector of unsigned 8 bit ints with 64 elements
fn create_message_block(input: String) -> Vec<u32> {
	let mut message_block: Vec<u8> = Vec::new();
	for c in input.to_string().chars() {
		message_block.push(c as u8);
	}
	message_block.push(1 << 7);
	message_block.extend(vec![0b0000 as u8; 64 - input.len() - 5]); // reserve last 32 bits for length of input

	let original_message_bin_len = message_block[0..input.len()]
		.into_iter()
		.map(|x| format!("{x:0>8b}"))
		.collect::<Vec<String>>()
		.join("")
		.len();

	let mut message_block = convert_message_block_to_u32(&message_block);
	message_block.push(original_message_bin_len as u32);
	message_block.extend(vec![0; 48]); // 64 elements of 32 bit ints
	message_block
}

fn is_prime(num: i32) -> bool {
	for i in 1..num {
		if num % i == 0 && i != 1 && i != num {
			return false;
		}
	}
	return true;
}

fn fill_message_block(message_block: &mut [u32]) {
	for ind in 16..message_block.len() {
		let w0 = message_block[ind - 16];
		let w1 = message_block[ind - 15];
		let w9 = message_block[ind - 7];
		let w14 = message_block[ind - 2];

		let s0 = w1.clone().rotate_right(7) ^ w1.clone().rotate_right(18) ^ w1.clone() >> 3;
		let s1 = w14.clone().rotate_right(17) ^ w14.clone().rotate_right(19) ^ w14.clone() >> 10;
		let new_word = w0.wrapping_add(s0).wrapping_add(w9).wrapping_add(s1);
		message_block[ind] = new_word;
	}
}

fn calc_initial_hashes() -> Vec<u32> {
	let mut initial_hashes = Vec::new();
	for num in 2..=19 {
		if is_prime(num) {
			let sqrt = (num as f64).sqrt();
			let sqrt_fract = sqrt.fract();
			let result_dec = ((sqrt_fract * 2_f64.powf(32.0) * 10_f64.powf(9.0)).round() / 10_f64.powf(9.0)) as u32;
			initial_hashes.push(result_dec);
		}
	}
	initial_hashes
}

fn calc_k_constants() -> Vec<u32> {
	let mut initial_hashes = Vec::new();
	for num in 2..=311 {
		if is_prime(num) {
			let rt = (num as f64).cbrt();
			let rt_fract = rt.fract();
			let result_dec = ((rt_fract * 2_f64.powf(32.0) * 10000000000.0).round() / 10000000000.0) as u32;
			initial_hashes.push(result_dec);
		}
	}
	initial_hashes
}

fn calc_hash(message_block: &[u32]) -> String {
	let hashes = calc_initial_hashes();
	let k_constants = calc_k_constants();

	let mut a = hashes[0];
	let mut b = hashes[1];
	let mut c = hashes[2];
	let mut d = hashes[3];
	let mut e = hashes[4];
	let mut f = hashes[5];
	let mut g = hashes[6];
	let mut h = hashes[7];

	for row in 0..message_block.len() {
		let s0 = a.clone().rotate_right(2) ^ a.clone().rotate_right(13) ^ a.clone().rotate_right(22);
		let s1 = e.clone().rotate_right(6) ^ e.clone().rotate_right(11) ^ e.clone().rotate_right(25);
		let choice = (e & f) ^ (!e & g);
		let majority = (a & b) ^ (a & c) ^ (b & c);
		let temp1 = h.wrapping_add(s1).wrapping_add(choice).wrapping_add(k_constants[row]).wrapping_add(message_block[row]);
		let temp2 = s0.wrapping_add(majority);

		h = g;
		g = f;
		f = e;
		e = d.wrapping_add(temp1);
		d = c;
		c = b;
		b = a;
		a = temp1.wrapping_add(temp2);
	}

	a = a.wrapping_add(hashes[0]);
	b = b.wrapping_add(hashes[1]);
	c = c.wrapping_add(hashes[2]);
	d = d.wrapping_add(hashes[3]);
	e = e.wrapping_add(hashes[4]);
	f = f.wrapping_add(hashes[5]);
	g = g.wrapping_add(hashes[6]);
	h = h.wrapping_add(hashes[7]);

	format!("{a:08x}{b:08x}{c:08x}{d:08x}{e:08x}{f:08x}{g:08x}{h:08x}")
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 as usize {
		panic!("ERROR: Expected an argument with length > 0 in quotes.\nExample: `cargo run \"this is an example\"`");
	}
	let input = &args[1];
	let mut message_block = create_message_block(input.to_string());
	fill_message_block(&mut message_block);
	let hash = calc_hash(&message_block);
	println!("{hash}");
}
