use {
    std::io::Cursor,
    std::sync::mpsc,
    std::time::Duration,
    tray_item::TrayItem,
    tray_item::IconSource,
    arboard::Clipboard
};

#[derive(Clone)]
enum Message {
    Quit,
    ItemE,
    ItemD,
    ItemC,
    ItemB,
    ItemA
}

fn create_tray(_tx: mpsc::SyncSender<Message>, _menu_labels: [&str; 5]) -> TrayItem {
	
    let cursor = Cursor::new(include_bytes!("../resources/icon.png"));
    let decoder = png::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let icon = IconSource::Data {
        data: buf,
        height: IMAGE_SIZE,
        width: IMAGE_SIZE,
    };

    let mut tray = TrayItem::new("Susuwatari", icon).unwrap();

    tray.add_label("Susuwatari").unwrap();

    tray.inner_mut().add_separator().unwrap();

    let menu_items = vec![
        (_menu_labels[0], Message::ItemA),
        (_menu_labels[1], Message::ItemB),
        (_menu_labels[2], Message::ItemC),
        (_menu_labels[3], Message::ItemD),
        (_menu_labels[4], Message::ItemE),
    ];

    for (label, message) in menu_items {
        let tx_clone = _tx.clone();
        tray.add_menu_item(label, move || {
            tx_clone.send(message.clone()).unwrap();
        }).unwrap();
    }

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = _tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    }).unwrap();

    tray
}

const TIMEOUT_DURATION_MS: u64 = 100;
const IMAGE_SIZE: i32 = 32;

fn main() {
    gtk::init().expect("Failed to initialize GTK");
    
    let (tx, rx) = mpsc::sync_channel::<Message>(2);
    
    let menu_labels: [&str; 5] = ["", "", "", "", ""];

    create_tray(tx.clone(), menu_labels);
	
    let mut last_item = String::new();
    let timeout_duration = Duration::from_millis(TIMEOUT_DURATION_MS);

    loop {	
	    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
	    
		if last_item != clipboard.get_text().unwrap() {
			last_item = clipboard.get_text().unwrap();
			println!("Clipboard text was: {}", last_item);
			
		}
		
        match rx.recv_timeout(timeout_duration) {
			Ok(Message::Quit) => {
				println!("Quit");
				break;
			},
			Ok(Message::ItemA) => {
				println!("ItemA !");
			},
			Ok(Message::ItemB) => {
				println!("ItemB !");
			},
			Ok(Message::ItemC) => {
				println!("ItemC !");
			},
			Ok(Message::ItemD) => {
				println!("ItemD !");
			},
			Ok(Message::ItemE) => {
				println!("ItemE !");
			},
			Err(_) => {}
		}
    }
}
