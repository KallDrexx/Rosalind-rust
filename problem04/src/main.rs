fn main() {
	let num_months = 34i;
	let reproduction_rate = 4i;
    let result = calculate_reproduction(num_months - 2, reproduction_rate, 1, 1);
    println!("{}", result);
}

fn calculate_reproduction(num_months: int, rate: int, last_population: int, last_two_population: int) -> int {
	println!("Population: {}", last_population);
	if num_months == 0 {
		return last_population;
	}

	let new_population = (last_two_population * rate) + last_population;

	calculate_reproduction(num_months - 1, rate, new_population, last_population)
}
