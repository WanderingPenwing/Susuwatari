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
    ItemA,
}

const TIMEOUT_DURATION_MS: u64 = 100;
const IMAGE_SIZE: i32 = 32;

// Function to create the tray with menu items
fn create_tray(message_sender: mpsc::SyncSender<Message>, menu_labels: Vec<String>) -> TrayItem {
    let icon = load_tray_icon();
    let mut tray = initialize_tray("Susuwatari", icon);

    add_menu_items(&mut tray, &message_sender, &menu_labels);

    tray
}

// Load tray icon from resources
fn load_tray_icon() -> IconSource {
    let cursor = Cursor::new(include_bytes!("../resources/icon.png"));
    let decoder = png::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    IconSource::Data {
        data: buf,
        height: IMAGE_SIZE,
        width: IMAGE_SIZE,
    }
}

// Initialize tray with a label and separator
fn initialize_tray(label: &str, icon: IconSource) -> TrayItem {
    let mut tray = TrayItem::new(label, icon).unwrap();
    tray.add_label(label).unwrap();
    tray.inner_mut().add_separator().unwrap();
    tray
}

// Add menu items to the tray
fn add_menu_items(tray: &mut TrayItem, message_sender: &mpsc::SyncSender<Message>, menu_labels: &Vec<String>) {
    let menu_items = menu_labels
        .iter()
        .cloned()
        .zip(vec![
            Message::ItemA,
            Message::ItemB,
            Message::ItemC,
            Message::ItemD,
            Message::ItemE,
        ])
        .collect::<Vec<_>>();

    for (label, message) in menu_items {
        add_menu_item(tray, &label, &message_sender, message);
    }

    tray.inner_mut().add_separator().unwrap();

    add_menu_item(tray, "Quit", &message_sender, Message::Quit);
}

// Add a menu item to the tray
fn add_menu_item(tray: &mut TrayItem, label: &str, message_sender: &mpsc::SyncSender<Message>, message: Message) {
    let message_sender_clone = message_sender.clone();
    tray.add_menu_item(label, move || {
        message_sender_clone.send(message.clone()).unwrap();
    }).unwrap();
}

// Function to shift elements in a vector, used for FIFO operation
fn shift_fifo(string_to_add: &str, vector: &mut Vec<String>) {
    let len = vector.len();
    for i in (1..len).rev() {
        vector[i] = vector[i - 1].clone();
    }
    vector[0] = string_to_add.to_string();
}

// Main function handling GTK initialization, clipboard, and event loop
fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let (message_sender, message_receiver) = mpsc::sync_channel::<Message>(2);
    let mut menu_labels: Vec<String> = vec!["".to_string(); 5];

    create_tray(message_sender.clone(), menu_labels.clone());

    let mut last_item = String::new();
    let timeout_duration = Duration::from_millis(TIMEOUT_DURATION_MS);

    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");

    loop {
        if let Ok(clipboard_text) = clipboard.get_text() {
            if last_item != clipboard_text {
                last_item = clipboard_text;
                if !menu_labels.contains(&last_item) {
                    shift_fifo(&last_item, &mut menu_labels);
                    println!("{:?}", menu_labels);
                }
            }
        } else {
            // Handle clipboard error
            eprintln!("Failed to get clipboard");
        }

        if handle_message(&message_receiver, &timeout_duration) {
			break;
		}
    }
}

// Function to handle messages received through the message channel
fn handle_message(message_receiver: &mpsc::Receiver<Message>, timeout_duration: &Duration) -> bool {
    match message_receiver.recv_timeout(*timeout_duration) {
        Ok(Message::Quit) => {
            println!("Quit");
            true // Signal to break the loop
        },
        Ok(Message::ItemA) => {
            println!("ItemA !");
            false
        },
        Ok(Message::ItemB) => {
            println!("ItemB !");
            false
        },
        Ok(Message::ItemC) => {
            println!("ItemC !");
            false
        },
        Ok(Message::ItemD) => {
            println!("ItemD !");
            false
        },
        Ok(Message::ItemE) => {
            println!("ItemE !");
            false
        },
        Err(_) => false
    }
}
