fn main() {
	let total_months = 86i;
	let lifespan = 20i;

	let mut rabbit_ages = Vec::new();

	rabbit_ages.push(1i);
	for x in range(1, lifespan) {
		rabbit_ages.push(0i);
	}

	//println!("Month {}: {}", 1i, rabbit_ages);

	for month in range(1, total_months) {
		let mut new_rabbits = rabbit_ages[rabbit_ages.len() - 1];

		for index in range(1, lifespan).rev() {
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
	for num in rabbit_ages.drain() {
		total_rabbits += num;
	}

    println!("Population: {}", total_rabbits);
}
