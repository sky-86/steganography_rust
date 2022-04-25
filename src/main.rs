use std::convert::TryFrom;
use std::fs;
use std::io::Read;

struct Image {
    magic_num: String,
    comment: String,
    dim: String,
    range: String,
    pixels: String,
}

// takes the img path, returns the bytes of the img
fn read_bytes(filename: String) -> Vec<u8> {
    let mut contents = Vec::new();
    let mut file = fs::File::open(&filename).expect("Error opening file");
    file.read_to_end(&mut contents).expect("Unable to read");

    contents.reverse();
    contents
}

// returns group of bytes based on <eol>
fn get_line_of_bytes(bytes: &mut Vec<u8>) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();
    while !bytes.ends_with(&[10]) {
        match bytes.pop() {
            Some(x) => line.push(x),

            None => break,
        }
    }
    match bytes.pop() {
        Some(x) => line.push(x),

        None => (),
    }
    line
}

fn string_to_binary(s: &String) -> String {
    let mut binary = String::new();

    for c in s.clone().into_bytes() {
        binary += &format!("0{:b}", c);
    }
    binary
}

// create a binary header for the msg, should be 4 bytes long
fn create_msg_header(length: &u16, depth: &u16) -> String {
    let mut length: String = format!("{:b}", &length);
    length = format!("{:0>16}", &length);
    let mut depth: String = format!("{:b}", &depth);
    depth = format!("{:0>16}", &depth);

    length + &depth
}

// Changes the pixel values to hide the msg
fn encode_msg(pixels: &mut Vec<u8>, header: &String, msg: &String) {}

fn main() {
    let mut img_bytes = read_bytes(String::from("test_p6.ppm"));

    let mut magic_num = get_line_of_bytes(&mut img_bytes);
    let mut comment = get_line_of_bytes(&mut img_bytes);
    let mut dimensions = get_line_of_bytes(&mut img_bytes);
    let mut color_range = get_line_of_bytes(&mut img_bytes);

    let mut pixels = get_line_of_bytes(&mut img_bytes);

    let msg = string_to_binary(&String::from("Hello World"));
    let msg_header = create_msg_header(&u16::try_from(msg.len()).unwrap(), &32);
}
