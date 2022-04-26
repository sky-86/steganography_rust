use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env;

// takes the img path, returns the bytes of the img
fn read_bytes(filename: String) -> Vec<u8> {
    let mut contents = Vec::new();
    let mut file = fs::File::open(&filename).expect("Error opening file");
    file.read_to_end(&mut contents).expect("Unable to read");

    contents
}

// convert a string into binary
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
fn encode_msg(pixels: &[u8], header: &String, msg: &String) -> Vec<u8> {
    let head = pixels.into_iter().take(32);
    let body = pixels.into_iter().skip(32).take(msg.len());
    let rest = pixels.into_iter().skip(32 + msg.len());
    let mut header_iter = header.chars();
    let mut msg_iter = msg.chars();
    let mut new_pixels = Vec::new();

    for i in head {
        new_pixels.push(encode_bit(i, header_iter.next().unwrap()));
    }

    for i in body {
        new_pixels.push(encode_bit(i, msg_iter.next().unwrap()));
    }

    for i in rest {
        new_pixels.push(*i);
    }
    new_pixels
}

fn encode_bit(bit: &u8, bin: char) -> u8 {
    if bin == '0' {
        // make even
        if bit % 2 == 0 {
            *bit
        } else {
            *bit + 1
        }
    } else {
        // make false
        if bit % 2 == 0 {
            if *bit == 255 {
                *bit - 1
            } else {
                *bit + 1
            }
        } else {
            *bit
        }
    }
}

fn encode_loop() {
    let img_bytes = read_bytes(String::from("test_p6.ppm"));

    // get an array for each line in the img file
    let mut lines: Vec<_> = img_bytes.split_inclusive(|x| *x == 10).collect();

    let pixels = lines.pop().unwrap();

    let msg = string_to_binary(&String::from("Hello World"));
    let msg_header = create_msg_header(&u16::try_from(msg.len()).unwrap(), &32);

    let mut new_pixels = encode_msg(&pixels, &msg_header, &msg);

    let mut new_img: Vec<u8> = Vec::new();

    for l in lines {
        new_img.extend_from_slice(l);
    }
    new_img.append(&mut new_pixels);

    let mut buffer = File::create("new_img.ppm").expect("Error creating file");

    buffer.write_all(&new_img);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];

    if flag == '-r' {
        // read flag, decode
        // take one param, the img path

    } else if flag == '-w' {
        // write flag, encode
        // takes two params, og img and the new img path
        
        
    } else {
        // no flag give, print usage message
    }
}
