use std::fs;
use std::io::Read;

struct Image {
	type: String,
	comment: String,
	dim: String,
	range: String,
	pixels: String,
}

fn read_bytes(filename: String) -> Vec<u8> {
	let mut contents = Vec::new();
	let mut file = fs::File::open(&p6).expect("Error opening file");
	file.read_to_end(&mut contents).expect("Unable to read");
	
	contents
}

// returns group of bytes based on <eol>
fn get_line_of_bytes(num: u8) -> Vec<u8> {

}

fn main() {
	
}
