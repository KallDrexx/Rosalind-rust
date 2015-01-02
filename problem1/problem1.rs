use std::io::File;
use std::io::BufferedReader;

struct Counts {
	count_a: int,
	count_c: int,
	count_g: int,
	count_t: int
}

fn main() {
	let path = Path::new("input.txt");
	let mut file = BufferedReader::new(File::open(&path));

	for line in file.lines() {
		let input = line.unwrap();
		let accumulator = get_counts(input);

		println!("{} {} {} {}", accumulator.count_a, accumulator.count_c, accumulator.count_g, accumulator.count_t);
	}
}

fn get_counts(input: String) -> Counts {
	let mut accumulator = Counts { count_a: 0i, count_c: 0i, count_g: 0i, count_t: 0i };

	for token in input.chars() {
		match token {
			'A' => accumulator.count_a += 1,
			'C' => accumulator.count_c += 1,
			'G' => accumulator.count_g += 1,
			'T' => accumulator.count_t += 1,
			_ => { /* ignore */ }
		}
	}

	accumulator
}