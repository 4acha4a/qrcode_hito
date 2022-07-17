use bmp_monochrome::Bmp;
use image::Luma;
use qrcode::QrCode;
use std::env;
use eframe::egui;
use egui_extras::RetainedImage;
use std::thread;


fn main() {
    let bruh = thread::spawn(|| {
                 create_qr();
         });
        bruh.join().unwrap();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
struct MyApp {
    image: RetainedImage,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "test.bmp",
		include_bytes!("test.bmp")
            )
            .unwrap(),
        }
    }
}
fn create_qr(){
    let qr_code = qr_code::QrCode::new(b"sosunok").unwrap();

    let bmp = qr_code.to_bmp();

    let bmp = Bmp::mul(&bmp,23).unwrap();

    let path = env::current_dir().unwrap();

    let path = path
	.into_os_string()
	.into_string()
	.unwrap() + "/src/test.bmp";

    bmp.write(std::fs::File::create(path).unwrap()).unwrap();
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	create_qr();
	egui::CentralPanel::default().show(ctx, |ui| {
            self.image.show(ui);
        });
    }
}
