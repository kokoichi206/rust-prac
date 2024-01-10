use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <image>", args[0]);
        return;
    }

    let infile = args[1].clone();
    let outfile = format!("{}-filtered.jpg", infile);
    println!("Reading image {}", infile);
    println!("Writing image {}", outfile);

    let mut img = image::open(infile).expect("failed to open image");
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let c : Rgba<u8> = img.get_pixel(x, y);
            // negative positive filter
            let c = Rgba([
                255 - c[0], // red
                255 - c[1], // green
                255 - c[2], // blue
                c[3]
            ]);
            img.put_pixel(x, y, c);
        }
    }

    img.save(outfile).unwrap()
}
