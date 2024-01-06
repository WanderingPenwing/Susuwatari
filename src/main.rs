use {
    std::io::Cursor,
    std::sync::mpsc,
    tray_item::TrayItem,
    tray_item::IconSource,
    arboard::Clipboard
};

enum Message {
    Quit,
    ItemE,
    ItemD,
    ItemC,
    ItemB,
    ItemA
}

fn main() {
    gtk::init().unwrap();

    let cursor = Cursor::new(include_bytes!("../resources/icon.png"));
    let decoder = png::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0;info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let icon = IconSource::Data{data: buf, height: 32, width: 32};
    
     let mut tray = TrayItem::new("Susuwatari", icon).unwrap();

    tray.add_label("Susuwatari").unwrap();

    let (tx, rx) = mpsc::sync_channel::<Message>(2);
    let itema_tx = tx.clone();
    tray.add_menu_item("", move || {
        itema_tx.send(Message::ItemA).unwrap();
    })
    .unwrap();
	
	let itemb_tx = tx.clone();
    tray.add_menu_item("", move || {
        itemb_tx.send(Message::ItemB).unwrap();
    })
    .unwrap();

	let itemc_tx = tx.clone();
    tray.add_menu_item("", move || {
        itemc_tx.send(Message::ItemC).unwrap();
    })
    .unwrap();
    
	let itemd_tx = tx.clone();
    tray.add_menu_item("", move || {
        itemd_tx.send(Message::ItemD).unwrap();
    })
    .unwrap();
    
	let iteme_tx = tx.clone();
    tray.add_menu_item("", move || {
        iteme_tx.send(Message::ItemE).unwrap();
    })
    .unwrap();
 
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break
            },
            Ok(Message::ItemA) =>{
                println!("ItemA !");
            },
            Ok(Message::ItemB) =>{
                println!("ItemB !");
            },
            Ok(Message::ItemC) =>{
                println!("ItemC !");
            },
            Ok(Message::ItemD) =>{
                println!("ItemD !");
            },
            Ok(Message::ItemE) =>{
                println!("ItemE !");
            },
            _ => {}
        }
    }

}
