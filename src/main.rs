use image;
use image::GenericImageView;
use image::DynamicImage;
use reqwest;
use std::fs::File;
use std::io::{stdin,stdout,Write};

fn main() {
    let url = get_input();
    write_img(&url);
    let img = image::open("image.png").expect("Error Opening File.");
    create_art(img);
}

fn create_art(img: DynamicImage) {
    let chars = [' ',',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];
    let pixs = img.pixels();
    let mut brightness: usize;
    for i in pixs {
        if i.0 == 0 {
            println!("");
        }
        brightness = usize::from(summate_and_normalize(i.2.0));
        print!("{0}",chars[brightness]);
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
