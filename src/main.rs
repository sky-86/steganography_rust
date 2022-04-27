use std::convert::TryFrom;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::num::ParseIntError;

// takes the img path, returns the bytes of the img
fn read_bytes(filename: String) -> Vec<u8> {
    let mut contents = Vec::new();
    let mut file = fs::File::open(&filename).expect("Error opening file");
    file.read_to_end(&mut contents).expect("Unable to read");

    contents
}

// convert a string into binary
fn string_to_binary(s: &String) -> String {
    println!("string: {}", s);
    let mut binary = String::new();

    for c in s.clone().into_bytes() {
        binary += &format!("{:0>8}", &format!("0{:b}", c));
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

fn encode_loop(og_img_path: &String, new_img_path: &String, user_msg: &String) {
    // read img bytes
    let img_bytes = read_bytes(og_img_path.to_string());

    // get an array of bytes for each line in the img file
    let mut lines: Vec<_> = img_bytes.split_inclusive(|x| *x == 10).collect();

    // gets the array of pixels
    let pixels = lines.pop().unwrap();

    // convert message into binary string
    let msg = string_to_binary(&user_msg);
    // create a binary header string
    let msg_header = create_msg_header(&u16::try_from(msg.len()).unwrap(), &32);

    // encodes pixels with the msg_header and the msg
    let mut new_pixels = encode_msg(&pixels, &msg_header, &msg);

    // create a new vector to hold the new img
    let mut new_img: Vec<u8> = Vec::new();

    // add all the old img data, (except the pixels) to the new array
    for l in lines {
        new_img.extend_from_slice(l);
    }
    // add the modified pixels
    new_img.append(&mut new_pixels);

    // save the img
    let mut buffer = File::create(new_img_path).expect("Error creating file");
    buffer.write_all(&new_img).expect("error saving");
}

fn decode_loop(og_img_path: &String) {
    // read img bytes
    let img_bytes = read_bytes(og_img_path.to_string());

    // get an array of bytes for each line in the img file
    let mut lines: Vec<_> = img_bytes.split_inclusive(|x| *x == 10).collect();

    // gets the array of pixels
    let pixels = lines.pop().unwrap();

    // msg header in binary
    let msg_header: Vec<&u8> = pixels.into_iter().take(32).collect();

    // extracts the binary data from the pixels
    let mut bin_msg_header = String::new();
    for c in msg_header {
        if c % 2 == 0 {
            bin_msg_header.push('0');
        } else {
            bin_msg_header.push('1');
        }
    }

    // length of the hidden message and the starting bit
    let msg_length: u16 = u16::from_str_radix(&bin_msg_header[0..16], 2).unwrap();
    let msg_depth: u16 = u16::from_str_radix(&bin_msg_header[16..32], 2).unwrap();

    // extract binary data and build binary message
    let msg_pixels: Vec<&u8> = pixels
        .into_iter()
        .skip(msg_depth.into())
        .take(msg_length.into())
        .collect();

    println!("msg_pixels_length: {}", msg_pixels.len());
    println!("msg_length: {}", msg_length);

    let mut bin_msg = String::new();
    for c in msg_pixels {
        if c % 2 == 0 {
            bin_msg.push('0');
        } else {
            bin_msg.push('1');
        }
    }

    // convert binary message into a ascii string
    let msg: String = String::from_utf8(bin_string_to_ascii(&bin_msg).unwrap()).unwrap();
    println!("msg: {}", msg);
}

fn bin_string_to_ascii(bin_str: &String) -> Result<Vec<u8>, ParseIntError> {
    println!("length: {}", bin_str.len());
    (0..bin_str.len())
        .step_by(8)
        .map(|i| u8::from_str_radix(&bin_str[i..i + 8], 2))
        .collect()
}

fn main() {
    // collect the args
    let mut args = env::args();
    let _exe_path = args.next();

    // check for flag
    let flag;
    match args.next() {
        Some(x) => flag = x,
        None => {
            println!("Provide a flag");
            return;
        }
    }

    if flag == "-r" {
        // read flag, decode
        // take one param, the img path
        let og_img_path = &args.next().expect("Provide a image path");
        decode_loop(og_img_path);
    } else if flag == "-w" {
        // write flag, encode
        // takes multiple params, og img, new img, msg, depth

        // get paths from args
        let og_img_path = &args.next().expect("Provide a image path");
        let new_img_path = &args.next().expect("Provide a save location");

        // since args are split by spaces, extra work has to be done to get full msg
        let args_len = &args.len();
        let mut user_msg = String::new();
        for (i, mut s) in args.enumerate() {
            if i + 1 != *args_len {
                s.push(' ');
            }
            user_msg.push_str(&s);
        }

        encode_loop(og_img_path, new_img_path, &user_msg);
    } else {
        // no flag give, print usage message
    }
}
