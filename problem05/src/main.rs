use std::fmt;
use std::io::File;
use std::io::BufferedReader;

struct DnaEntry {
	label: String,
	dna: String,
	percentage: f32
}

impl fmt::Show for DnaEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} ({}%): {}", self.label, self.percentage, self.dna)
	}
}

impl DnaEntry {
	fn calculate_percentage(&mut self) {
		let mut g_c_count = 0i;
		for character in self.dna.chars() {
			match character {
				'G' | 'C' => g_c_count += 1,
				_ => {}
			};
		}

		self.percentage = ((g_c_count as f32) / (self.dna.len() as f32)) * 100f32;
	}
}

fn main() {
	let path = Path::new("input.txt");
	let mut file = BufferedReader::new(File::open(&path));
	let mut current_entry: Option<DnaEntry> = None;
	let mut entries = Vec::new();

	for possible_line in file.lines() {
		let line = possible_line.unwrap();
		let trimmed_line = line.trim();

		if trimmed_line.char_at(0) == '>' {
			push_entry(current_entry, &mut entries);

			let new_entry = DnaEntry {
				label: String::from_str(trimmed_line.slice(1, trimmed_line.len())),
				dna: String::new(),
				percentage: 0f32
			};

			current_entry = Some(new_entry);
		}
		else {
			let mut entry = current_entry.unwrap();

			entry.dna.push_str(trimmed_line.as_slice());
			entry.calculate_percentage();
			current_entry = Some(entry);
		}
	}

	push_entry(current_entry, &mut entries);

	let highest_entry = get_highest_percentage(entries);
	println!("{}", highest_entry);
}

fn get_highest_percentage(mut entries: Vec<DnaEntry>) -> Option<DnaEntry> {
	let mut candidate: Option<DnaEntry> = None;

	loop {
		let possible_entry = entries.pop();
		if possible_entry.is_none() {
			break;
		}

		let entry = possible_entry.unwrap();
		candidate = match candidate {
			Some(x) => if entry.percentage > x.percentage {Some(entry)} else {Some(x)},
			None => Some(entry)
		};
	}

	candidate
}

fn push_entry(possible_entry: Option<DnaEntry>, vector: &mut Vec<DnaEntry>) {
	match possible_entry {
		Some(entry) => vector.push(entry),
		None => {}
	}
}