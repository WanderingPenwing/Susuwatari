use tray_icon::{Icon, TrayIconBuilder, menu::Menu};
use image::DynamicImage;
use image::GenericImageView;

fn main() {
    // Open the image file
    if let Ok(img) = image::open("../ressources/icon.png") {
        // Convert the image to RGBA
        let rgba_image = img.to_rgba8();

        // Do whatever you want with the RGBA image here
        // For demonstration purposes, we'll just print its dimensions
        let (width, height) = rgba_image.dimensions();

		let rgba_bytes: Vec<u8> = rgba_image.to_vec();

		let maybe_icon = Icon::from_rgba(rgba_bytes, width, height);

		if let Ok(icon) = maybe_icon {
	 		let tray_menu = Menu::new();
	       	let tray_icon = TrayIconBuilder::new()
	       	    .with_menu(Box::new(tray_menu))
	       	    .with_tooltip("system-tray - tray icon library!")
	       	    .with_icon(icon)
	       	    .build()
	       	    .unwrap();
	    } else {
	    	println!("Failed to convert to icon");
	    }
 		              
    } else {
        println!("Failed to open image file.");
    }
}
