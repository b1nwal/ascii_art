use image;
use image::GenericImageView;
use reqwest;
use std::fs::File;

fn main() {
    let url = "https://cdn.discordapp.com/attachments/628804839714979850/1014057928422076417/BZH3_HQ9.png";
    get_img(url);
    let img = image::open("image.png").expect("Error Opening File.");
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

fn summate_and_normalize(x: [u8;4]) -> u8 {
    let t: u8 = ((x[0]/4 + x[1]/4 + x[2]/4) / 3) + 3 & !3;
    return t;
}

fn get_img(url: &str) -> File {
    let mut img = File::create("image.png").expect("Error Creating File");
    reqwest::blocking::get(url)
        .unwrap()
        .copy_to(&mut img)
        .unwrap();
    return img;
}
