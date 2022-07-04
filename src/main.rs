use std::{fs::OpenOptions, io::Write};

use image::GenericImageView;

static ASCII_LUMINANCE_STR: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`\'. ";

fn main() {
    let img = image::open("kirby.png").unwrap();
    let (height, width) = img.dimensions();
    println!("{} {}", height, width);
    let mut canvas = vec![vec!['#'; height as usize]; width as usize];
    let mut f = OpenOptions::new().write(true).open("test.txt").unwrap();

    for i in 0..width - 1 {
        for j in 0..height - 1 {
            let pixel = img.get_pixel(j, i);

            let r = pixel[0] as f64;
            let g = pixel[1] as f64;
            let b = pixel[2] as f64;

            let mut lum = 0.299 * r + 0.587 * g + 0.114 * b;
            lum = (lum / 3.65).floor();
            let index = lum as usize;
            let char = ASCII_LUMINANCE_STR.chars().nth(index).unwrap();
            canvas[i as usize][j as usize] = char;
        }
    }

    for i in 0..width - 1 {
        let row = &canvas[i as usize];
        let mut s: String = row.into_iter().collect();
        s += "\n";
        f.write_all(s.as_bytes()).unwrap();
    }
}
