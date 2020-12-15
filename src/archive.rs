extern crate libarchive;

pub fn extract_files(file: String) {
	println!("extracting {}", file);
}

pub fn extract_file_to_stdout(file: String, inarc: String) {
	println!("extracting {} from {}", inarc, file);
}

pub fn list_files(file: String) {
	println!("listing {}", file);
}
