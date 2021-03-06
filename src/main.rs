use std::{
    fs::File,
    io::{stdin, Write},
};

use image::{
    imageops::FilterType,
    DynamicImage, GenericImageView,
};

static ASCII_LUMINANCE_STR: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`\'. ";

fn main() {
    //get input first
    let mut img;

    loop {
        let mut s = String::new();
        println!("Image path : ");
        stdin().read_line(&mut s).expect("Please enter a file name");

        //remove EoL characters
        s = s.replace("\n", "").replace("\r", "");

        match image::open(&s) {
            Ok(r) => {
                img = r;
                break;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    let scale;

    loop {
        println!("scale: ");

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Please enter a float");

        match line.trim().parse() {
            Ok(s) => {
                scale = s;
                break;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    img = image_scaler(&img, scale);
    image_ascii_convertor(&img);
}

fn image_scaler(img: &DynamicImage, scale: f32) -> DynamicImage {
    let (height, width) = img.dimensions();
    let (nheight, nwidth) = (height as f32 * scale, width as f32 * scale);
    img.resize(nwidth as u32, nheight as u32, FilterType::Lanczos3)
}

fn image_ascii_convertor(img: &DynamicImage) {
    let (height, width) = img.dimensions();
    let mut canvas = vec![vec!['#'; height as usize]; width as usize];
    std::fs::remove_file("output.txt").expect("File delete failed");
    let mut f = File::create("output.txt").unwrap();

    //convert image to 2d vector of geryscale
    for i in 0..width - 1 {
        for j in 0..height - 1 {
            let pixel = img.get_pixel(j, i);

            let r = pixel[0] as f64;
            let g = pixel[1] as f64;
            let b = pixel[2] as f64;

            //don't ask why is this forumla, someone figured this out
            let mut lum = 0.299 * r + 0.587 * g + 0.114 * b;
            lum = (lum / 3.65).floor();
            let index = lum as usize;
            let char = ASCII_LUMINANCE_STR.chars().nth(index).unwrap();
            canvas[i as usize][j as usize] = char;
        }
    }

    let mut s = String::new();
    println!("Generate for comment? (y/n)");
    stdin().read_line(&mut s).expect("Please enter y / n");

    //write file
    if s.trim() == "y"{ 
        print_for_java_comment(f, width, canvas);
    } else{
        normal_print(f, width, canvas);
    }
}

//yeah I know this fuction is ugly af
fn print_for_java_comment(mut f:File, width:u32, canvas: Vec<Vec<char>>){
    f.write_all("\"\\n\"+\n".as_bytes()).unwrap();

    for i in 0..width - 1 {
        let row = &canvas[i as usize];
        let mut s: String = row.into_iter().collect();
        s = s
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\'", "\\\'");
        s = "\"".to_owned() + &s + "\\n\"+\n";
        if i == width - 2 {
            s = s.replace("+", ";");
        };
        f.write_all(s.as_bytes()).unwrap();
    }
}

fn normal_print(mut f:File, width:u32, canvas: Vec<Vec<char>>){
    for i in 0..width - 1 {
        let row = &canvas[i as usize];
        let mut s: String = row.into_iter().collect();
        s += "\n";
        f.write_all(s.as_bytes()).unwrap();
    }
}