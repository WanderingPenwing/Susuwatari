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


fn create_tray(_message_sender: mpsc::SyncSender<Message>, _menu_labels: Vec<String>) -> TrayItem {
	
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

    let menu_items = _menu_labels.iter().cloned().zip(vec![
        Message::ItemA,
        Message::ItemB,
        Message::ItemC,
        Message::ItemD,
        Message::ItemE,
    ]).collect::<Vec<_>>();

    for (label, message) in menu_items {
        let message_sender_clone = _message_sender.clone();
        tray.add_menu_item(&label, move || {
            message_sender_clone.send(message.clone()).unwrap();
        }).unwrap();
    }

    tray.inner_mut().add_separator().unwrap();

    let quit_message_sender = _message_sender.clone();
    tray.add_menu_item("Quit", move || {
        quit_message_sender.send(Message::Quit).unwrap();
    }).unwrap();

    tray
}


fn shift_fifo(string_to_add: &str, vector: &mut Vec<String>) {
    for i in (1..vector.len()).rev() {
        std::mem::swap(&mut vector[i], &mut vector[i - 1]);
    }
    vector[0] = string_to_add.to_string();
}


const TIMEOUT_DURATION_MS: u64 = 100;
const IMAGE_SIZE: i32 = 32;


fn main() {
    gtk::init().expect("Failed to initialize GTK");
    
    let (message_sender, message_receiver) = mpsc::sync_channel::<Message>(2);
    
    let mut menu_labels: Vec<String> = vec!["".to_string(); 5];

    create_tray(message_sender.clone(), menu_labels.clone());
	
    let mut last_item = String::new();
    let timeout_duration = Duration::from_millis(TIMEOUT_DURATION_MS);
    
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");

    loop {	
	    
		if last_item != clipboard.get_text().expect("Failed to get clipboard") {
			last_item = clipboard.get_text().unwrap();
			if !menu_labels.contains(&last_item) {
				shift_fifo(&last_item, &mut menu_labels);
				println!("{:?}", menu_labels);
			}
		}
		
        match message_receiver.recv_timeout(timeout_duration) {
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
