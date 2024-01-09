use ksni;

#[derive(Debug)]
struct MyTray {
    items : Vec<String>,
}

const BUFFER_LENGTH : usize = 5;

impl ksni::Tray for MyTray {
    fn icon_name(&self) -> String {
        "help-about".into()
    }
    fn title(&self) -> String {
        { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
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
                    label: item_clone.clone(),
                    activate: Box::new(move |_| println!("{}", item_clone)),
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

fn main() {
    let service = ksni::TrayService::new(MyTray {
        items: vec!["".to_string(); BUFFER_LENGTH],
    });
    let handle = service.handle();
    service.spawn();

    std::thread::sleep(std::time::Duration::from_secs(5));
    // We can modify the tray
    handle.update(|tray: &mut MyTray| {
        tray.items[0] = "kenobi".to_string();
    });
    // Run forever
    loop {
        std::thread::park();
    }
}
