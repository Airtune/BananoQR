use clap::Parser;
extern crate image;
extern crate qrcode;

use image::{
    imageops::{overlay, resize},
    DynamicImage, GenericImageView, Rgba, RgbaImage,
};
use qrcode::QrCode;

fn generate_qr_code(data: &str) -> DynamicImage {
    // Generate the QR code
    let code = QrCode::new(data).unwrap();

    // Convert the QR code to an image
    let image = code.render::<Rgba<u8>>().build();

    image::DynamicImage::ImageRgba8(image)
}

fn load_logo(logo_path: &str) -> RgbaImage {
    image::open(logo_path)
        .expect("Failed to load logo image")
        .to_rgba8()
}

fn fetch_logo_from_url(url: &str) -> RgbaImage {
    let response = reqwest::blocking::get(url).expect("Failed to fetch logo image");
    let bytes = response.bytes().expect("Failed to read logo image bytes");
    let image = image::load_from_memory(&bytes).expect("Failed to load logo image from bytes");

    image.to_rgba8()
}

fn overlay_logo(qr_code: &DynamicImage, logo: &RgbaImage) -> DynamicImage {
    // Scale the logo to fit within the dimensions of the QR code
    let (qr_width, qr_height) = qr_code.dimensions();
    let (logo_width, logo_height) = logo.dimensions();

    let logo_resized_width = (qr_width * 2) / 5;
    let logo_resized_height = (logo_height * logo_resized_width) / logo_width;
    let logo = resize(
        logo,
        logo_resized_width,
        logo_resized_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Create a copy of the QR code image
    let mut combined_image = qr_code.clone();

    // Calculate the position to center the logo on the QR code
    let x_offset = (qr_width - logo_resized_width) / 2;
    let y_offset = (qr_height - logo_resized_height) / 2;

    // Overlay the logo on top of the QR code
    overlay(&mut combined_image, &logo, x_offset, y_offset);

    combined_image
}

fn save_qr_code_with_logo(image: &DynamicImage, output_path: &str) {
    image.save(output_path).expect("Failed to save image");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct BananoQR {
    amount: f64,
    address: String,
    filename: String,
    model: i8,
}

fn main() {
    let args = BananoQR::parse();

    let address = args.address;
    let amount = args.amount;
    let filename = args.filename;

    let data = format!("ban:{address}?amount={amount}");
    let logo_path = "logo.png";
    let output_path = format!("{filename}.png");

    if args.model == 0 {
        let qr_code = generate_qr_code(&data);
        let logo = load_logo(logo_path);
        let qr_code_with_logo = overlay_logo(&qr_code, &logo);
        save_qr_code_with_logo(&qr_code_with_logo, &output_path);
    } else if args.model == 1 {
        let qr_code = generate_qr_code(&data);
        let logo = fetch_logo_from_url(&format!(
            "https://monkey.banano.cc/api/v1/monkey/{address}?format=png&size=100&background=false"
        ));
        let qr_code_with_logo = overlay_logo(&qr_code, &logo);
        save_qr_code_with_logo(&qr_code_with_logo, &output_path);
    }
}
