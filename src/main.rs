use {
    std::io::Cursor,
    tray_item::TrayItem,
    tray_item::IconSource
};

fn main() {
    gtk::init().unwrap();

    let cursor = Cursor::new(include_bytes!("../resources/icon.png"));
    let decoder = png::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0;info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let icon = IconSource::Data{data: buf, height: 32, width: 32};
    
     let mut tray = TrayItem::new("Susuwatari", icon).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    gtk::main();
}
