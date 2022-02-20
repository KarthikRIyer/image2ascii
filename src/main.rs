use std::ops::Index;
use std::f64;
use image::io::Reader as ImageReader;
use std::fs;

fn main() {
    let resize_factor: f64 = 3 as f64;

    let density = "Ñ@#W$9876543210?!abc;:+=-,._  ";
    let density_len = (density.chars().count() - 1) as f64;
    let mut img = ImageReader::open("dog.jpeg").unwrap().decode().unwrap();

    let new_width = (img.width() as f64 / resize_factor) as u32;
    let new_height = (img.height() as f64 / resize_factor) as u32;

    img = img.thumbnail(new_width, new_height);
    let width = img.width();
    let height = img.height();
    let img_grayscale = img.grayscale();
    let img_rows = img_grayscale.as_luma8().unwrap().rows();
    let img_rows_count = img_rows.len();
    let mut text_list: Vec<String> = Vec::with_capacity(img_rows_count);

    for row in img_rows {
        let mut row_str = String::from("");
        for pix in row {
            let value = *pix.index(0) as f64;
            let char_index = map_range((0.0, 255.0), (density_len, 0.0), value).floor() as usize;
            let char_to_insert = density.chars().nth(char_index).unwrap();
            row_str.push(char_to_insert);
        }
        text_list.push(row_str);
    }

    let font_size = 10;

    let mut svg_str = String::new();
    svg_str.push_str("<?xml version=\"1.0\" ?>\n");
    svg_str.push_str(&*format!("<svg width=\"{}\" height=\"{}\" version=\"4.0\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n", width * font_size, height * font_size));
    svg_str.push_str("<rect width=\"100%\" height=\"100%\" fill=\"black\"/>\n");

    let mut y_pos = font_size;

    for line in text_list {
        y_pos += (font_size) as u32;
        svg_str.push_str(&*format!("<text x=\"0\" y=\"{}\" font-size=\"{}\" fill=\"white\" font-family=\"monospace\" letter-spacing=\"4\" xml:space=\"preserve\">{}</text>\n", y_pos, font_size, line));
    }
    svg_str.push_str("</svg>");

    fs::write("image.svg", svg_str).expect("Unable to write file.");
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
