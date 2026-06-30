use std::error::Error;

use image::ImageReader;

const __ASCII_CHARS: &str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const __SIMPLE_ASCII_CHARS: &str = " .:-=+*#%@";

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("./assets/image1.jpg")?.decode()?; // D => 32 : 17
    let rgb_img = image::imageops::resize(
        &img.to_rgb8(),
        120,
        64,
        image::imageops::FilterType::CatmullRom,
    );

    let chars : Vec<char> = __SIMPLE_ASCII_CHARS.chars().collect();
    
    for x in 0..rgb_img.height()
    {
        for y in 0..rgb_img.width()
        {
            let [r, g, b] = rgb_img.get_pixel(y, x).0;
            //calculate luminance
            let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
            let index = brightness as usize * (chars.len() - 1) / 255;
            print!("{}", chars[index]);
        }
        println!()
    }
    Ok(())
}
