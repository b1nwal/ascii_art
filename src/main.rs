use image;
use image::GenericImageView;
use image::DynamicImage;
use reqwest;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{stdin,stdout,Write};

fn main() {
    let url = get_input();
    write_img(&url);
    let img = image::open("image.png").expect("Error Opening File.");
    create_art(img);
    fs::remove_file("image.png").unwrap();
}

fn create_art(img: DynamicImage) {
    let chars = [b' ', b',', b':', b';', b'I', b'l', b'!', b'i', b'>', b'<', b'~', b'+', b'_', b'-', b'?', b']', b'[', b'}', b'{', b'1', b')', b'(', b'|', b'\\', b'/', b't', b'f', b'j', b'r', b'x', b'n', b'u', b'v', b'c', b'z', b'X', b'Y', b'U', b'J', b'C', b'L', b'Q', b'0', b'O', b'Z', b'm', b'w', b'q', b'p', b'd', b'b', b'k', b'h', b'a', b'o', b'*', b'#', b'M', b'W', b'&', b'8', b'%', b'B', b'@', b'$'];
    let mut art = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("result/result.txt")
        .unwrap();
    let pixs = img.pixels();
    let mut brightness: usize;
    let mut character: u8;
    for i in pixs {
        if i.0 == 0 {
            art.write(b"\n").unwrap();
        }
        brightness = usize::from(summate_and_normalize(i.2.0));
        character = chars[brightness];
        art.write(&[character]).unwrap();
    }
}

fn get_input() -> String {
    let mut url = String::new();
    println!("Please enter a URL to a low-res PNG file (128x128 is ideal)");
    let _=stdout().flush();
    stdin().read_line(&mut url).expect("Invalid String");
    url.pop();
    return url;
}

fn summate_and_normalize(x: [u8;4]) -> u8 {
    let t: u8 = ((x[0]/4 + x[1]/4 + x[2]/4) / 3) + 3 & !3;
    return t;
}

fn get_img(url: &str) -> reqwest::blocking::Response {
    let _res = reqwest::blocking::get(url);
    let res = match _res {
        Ok(a) => a,
        Err(_) => {let url = get_input(); get_img(&url)},
    };
    return res;
}

fn write_img(url: &str) -> File {
    let mut res = get_img(url);
    let mut img = File::create("image.png").expect("Error Creating File");
    res.copy_to(&mut img).unwrap();
    return img;
}
