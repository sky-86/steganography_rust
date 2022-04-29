use clap::Parser;
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
    let mut file = fs::File::open(&filename).expect(&filename);
    file.read_to_end(&mut contents).expect("Unable to read");

    contents
}

// convert a string into binary
fn string_to_binary(s: &String) -> String {
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
fn encode_msg(pixels: &[u8], header: &String, msg: &String, depth: &usize) -> Vec<u8> {
    let head = pixels.into_iter().take(*depth);
    let body = pixels.into_iter().skip(*depth).take(msg.len());
    let rest = pixels.into_iter().skip(*depth).skip(msg.len());
    let mut header_iter = header.chars();
    let mut msg_iter = msg.chars();
    let mut new_pixels = Vec::new();

    for (i, x) in head.enumerate() {
        if i < 32 {
            new_pixels.push(encode_bit(x, header_iter.next().unwrap()));
        } else {
            new_pixels.push(*x);
        }
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

fn encode_loop(og_img_path: &String, new_img_path: &String, user_msg: &String, depth: &usize) {
    // read img bytes
    let img_bytes = read_bytes(og_img_path.to_string());

    // get an array of bytes for each line in the img file
    let mut lines: Vec<_> = img_bytes.split_inclusive(|x| *x == 10).collect();

    // gets the array of pixels
    let pixels = lines.pop().unwrap();

    // convert message into binary string
    let msg = string_to_binary(&user_msg);
    // create a binary header string
    let msg_header = create_msg_header(
        &u16::try_from(msg.len()).unwrap(),
        &u16::try_from(*depth).unwrap(),
    );

    // encodes pixels with the msg_header and the msg
    let mut new_pixels = encode_msg(&pixels, &msg_header, &msg, &depth);

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

fn decode_loop(img_path: &String) {
    // read img bytes
    let img_bytes = read_bytes(img_path.to_string());

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
    println!("Hidden Message: {}", msg);
}

fn bin_string_to_ascii(bin_str: &String) -> Result<Vec<u8>, ParseIntError> {
    (0..bin_str.len())
        .step_by(8)
        .map(|i| u8::from_str_radix(&bin_str[i..i + 8], 2))
        .collect()
}

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Skyler Favors",
    version = "0.1",
    about = "A steganography project"
)]
struct Arguments {
    #[clap(forbid_empty_values = true)]
    img_path: String,

    #[clap(short, long)]
    depth: Option<usize>,
    #[clap(short, long)]
    save_path: Option<String>,
    #[clap(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    // check which args were given
    // determine if it should read or write
    if let (Some(s), Some(m), Some(d)) = (&args.save_path, &args.message, &args.depth) {
        println!("write");
        // stops the message from overwriting the header
        let base = *d + 32;
        encode_loop(&args.img_path, &s, &m, &base);
    } else {
        println!("read");
        decode_loop(&args.img_path);
    }
}
