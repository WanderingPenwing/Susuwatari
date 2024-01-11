use ksni::{self, Icon};
use arboard::Clipboard;
use std::io::Cursor;
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

const BUFFER_LENGTH: usize = 10;
const LINE_LENGTH: usize = 30;
const ICON_WIDTH: i32 = 32;
const ICON_HEIGHT: i32 = 32;


#[derive(Debug)]
struct MyTray {
    items : Vec<String>,
}


impl ksni::Tray for MyTray {
    fn icon_pixmap(&self) -> Vec<Icon> {
        let cursor_icon = Cursor::new(include_bytes!("/etc/susuwatari/icon.png"));
        let decoder_icon = png::Decoder::new(cursor_icon);
        let (info_icon, mut reader_icon) = decoder_icon.read_info().expect("Failed reading icon data");
        let mut buf_icon = vec![0; info_icon.buffer_size()];
        reader_icon.next_frame(&mut buf_icon).expect("Failed getting icon frame data");

        vec![Icon {
            width: ICON_WIDTH,
            height: ICON_HEIGHT,
            data: buf_icon,
        }]
    }
    fn title(&self) -> String {
        { "MyTray" }.into()
    }
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        
        let item_menu: Vec<ksni::MenuItem<Self>> = self
            .items
            .iter()
            .map(|item| {
                let item_clone = item.clone();
                StandardItem {
                    label: format_title(&item_clone),
                    activate: Box::new(move |_| set_clipboard_text(&item_clone)),
                    ..Default::default()
                }
                .into()
            })
            .collect();
            
        let mut menu = Vec::new();
        menu.push(SubMenu {
            label: "Susuwatari".into(),
            ..Default::default()
        }
        .into());
        menu.push(MenuItem::Separator);
        menu.extend(item_menu);
        menu.push(MenuItem::Separator);
        menu.push(StandardItem {
            label: "Exit".into(),
            icon_name: "application-exit".into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        }
        .into());
		
		menu
    }
}

fn format_title( text : &str) -> String {
	let line = str::replace(text, "\n", "");
    let end = line.chars().map(|c| c.len_utf8()).take(LINE_LENGTH).sum();
    line[..end].to_string()
}


fn shift_fifo(string_to_add: &str, vector: &Vec<String>) -> Vec<String> {
	let mut new_vect = vector.clone();
    let len = vector.len();
    for i in (1..len).rev() {
        new_vect[i] = new_vect[i - 1].clone();
    }
    new_vect[0] = string_to_add.to_string(); 
    
    new_vect
}


fn main() {
	println!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);
	
	let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
	let mut last_item = String::new();
	
    let service = ksni::TrayService::new(MyTray {
        items: vec!["".to_string(); BUFFER_LENGTH],
    });
    let handle = service.handle();
    service.spawn();

    // Run forever
    loop {
        if let Ok(clipboard_text) = clipboard.get_text() {
            if last_item != clipboard_text {
                last_item = clipboard_text;
                handle.update(|tray: &mut MyTray| {
					if !tray.items.contains(&last_item,) {
						tray.items = shift_fifo(&last_item, &tray.items);
					}
                });
            }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}


fn set_clipboard_text( txt : &str) {
	let mut _clipboard = Clipboard::new().expect("Failed to initialize clipboard");
	_clipboard.set_text(txt).unwrap();
}
