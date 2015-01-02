fn main() {
    let input = "ACACGAGAGAATATGATTTAGGCTCCGAGAGGTAAAGTCGGATCGTCGGCCATTATCTTCCGAGACTTGTAAGGGACTTCCTTTGAGATGTACCTCTTTTGGATCCTAGTGACGGACAGTAATGTTGATCAGGACGGGGGTCTTTATGAGCGTATAGGTTATGGCCTGCCGGCAACTGGGGCAGACTTTGCGAATCCCTCACTCCTATCTCATTAGGTTATCGTTTTATCGTGCTGGCTTTGGGTTCGCCTTACCGATACCATACTACCGCTCCGCGCCCCATCGCAATCACCGCTAAATGCCAATGGCACATGCAAGCCTTTGCTTTCGCCTTGTCGAACTTAGTAACCCTCCGGTAGAGCCCTACCTAACAGTCCTCACAGATGAAATCCACGATAGATCGAGGCATTGCGCCGATTAAGTTTGCACCTTCTCTGATATCACTTAGAGAACGTGATACGTATTCGAGAGCCTCGCGCAGGATCAAATATATCCTATCCAGATATGTTGAGCTCTACCCAGAATATTGAAGGAGATCTCGGTCACTTGACACGGCACATTGACACCCTAGGCGTGAAGTTGCCAATGAACCGCATGGTCTTACTTATGTTTGGCGTTTGAGAGTTAACGTTGTTCTTTCTCGGACAAATTCCGCGAAGCCTAGAGACACACGATGACCAAGATATCGATATCGGACCGTCTATGAATATCGCATCTGCGGGTTGGCGTCCGCCACTAACGCGTTTAATTGACCCTCGTTGAATCGCGCACAAGAATGGTCCTTGATGAGAGTAGGGGTCGAAGGTTTCGACTAGTACCAACTAAAAGTAACTGGCACCAACGCATTTTATGAGCGGATCTCATTTGACGATTATGAGGACAATCGATCAGCTTAGACACACGTGACTGATGCATGTGCAATAGCTGGGCGCCTAACCAGATC";
    let mut output: String = String::new();

    for character in input.chars() {
    	let rna_character = match character {
    		'T' => 'U',
    		_ => character
    	};

    	output.push(rna_character);
    }

    println!("Result: {}", output);
}