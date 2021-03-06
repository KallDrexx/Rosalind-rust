fn main() {
	let total_months = 86i;
	const LIFESPAN: uint = 20u;

	let mut rabbit_ages = [0i; LIFESPAN];
	rabbit_ages[0] = 1;

	//println!("Month {}: {}", 1i, rabbit_ages);

	for _ in range(1, total_months) {
		let mut new_rabbits = rabbit_ages[rabbit_ages.len() - 1];

		for index in range(1, LIFESPAN).rev() {
			let aged_rabbits = rabbit_ages[(index as uint) - 1];

			if index > 1 {
				new_rabbits += aged_rabbits;
			}

			rabbit_ages[(index as uint)] = aged_rabbits;
			
		}

		rabbit_ages[0] = new_rabbits;

		//println!("Month {}: {}", month + 1, rabbit_ages);
	}

	let mut total_rabbits = 0i;
	for &num in rabbit_ages.iter() {
		total_rabbits += num;
	}

    println!("Population: {}", total_rabbits);
}
