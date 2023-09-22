use opencv::{
    highgui, imgcodecs, imgproc,
    prelude::{Mat, MatTraitConst},
};

fn main() {
    let window = "window";
    highgui::named_window(window, 1).expect("Failed to create window");

    let image =
        imgcodecs::imread("arrow.png", imgcodecs::IMREAD_COLOR).expect("Failed to read image");

    if image.data().is_null() {
        panic!("Failed to read image");
    }

    // gray
    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 1)
        .expect("Failed to convert image color");

    // bin
    let mut bin = Mat::default();
    imgproc::threshold(&gray, &mut bin, 20.0, 255.0, imgproc::THRESH_BINARY)
        .expect("Failed to convert to threshold");

    // TODO: feature detection

    loop {
        highgui::imshow(window, &bin).expect("Failed to show image");
        let key = highgui::wait_key(10).expect("Failed to wait key");

        if key > 0 && key != 255 {
            break;
        }
    }
}
