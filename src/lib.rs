use rodio::{self, Sink};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

#[no_mangle]
pub extern "C" fn play() {
	

	println!("Hi from rust");
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	let source_one = read_source("06_squiggle.ogg");
	let source_four = read_source("06_squiggle.ogg");
	let source_two = read_source("04_guiro.ogg");
	let source_three = read_source("04_guiro.ogg");

	sink.append(source_one);
	sink.append(source_two);
	sink.append(source_four);
	sink.append(source_three);

	sink.sleep_until_end();
}

fn read_source(name: impl Into<String>) -> Decoder<BufReader<File>> {
	let base_path = "C:\\Users\\Johnny\\Desktop\\Sounds\\notifications";
	let file = BufReader::new(File::open(format!("{}\\{}", base_path, name.into())).unwrap());
	let source = Decoder::new(file).unwrap();
	source
}