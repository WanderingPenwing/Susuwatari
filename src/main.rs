use rdev::{grab, Event, EventType, Key, simulate};
use tokio::sync::mpsc;
use tokio::task;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use arboard::Clipboard;

enum Hotkey {
	ControlPressed,
	ControlReleased,
	ModPressed,
	ModReleased,
	KeyC,
	KeyV,
}

struct History {
	entries: Vec<String>,
	clipboard: Clipboard,
}

impl History {
	fn new() -> Self {
		Self {
			entries: vec![],
			clipboard: Clipboard::new().unwrap(),
		}
	}
	
	fn update(&mut self) {
		let clip = self.clipboard.get_text().unwrap();
		
		for entry in &self.entries {
			if &clip == entry {
				return
			}
		}
		
		self.entries.insert(0, clip);
	}
	
	async fn paste(&mut self) {
		if self.entries.len() == 0 {
			return
		}
		match self.select_entry().await {
			Ok(entry) => {
				let _ = self.clipboard.set_text(entry);
			}
			Err(why) => {
				eprintln!("error pasting clipboard : {}", why);
			}
		}
	}
	
	async fn select_entry(&self) -> Result<String, Box<dyn std::error::Error>> {
		
		
		let mut formated_data : Vec<String> = vec![];
		for (index, entry) in self.entries.iter().enumerate() {
			formated_data.push(format!("{}- {}",index, entry.split('\n').next().unwrap_or("err")));
		}

		let input_data = formated_data.join("\n");
		// Prepare the command
		let mut cmd = Command::new("sh")
			.arg("-c")
			.arg("marukuru -l 10")
			.stdin(std::process::Stdio::piped())
			.stdout(std::process::Stdio::piped())
			.spawn()?;
		
		let stdin = cmd.stdin.as_mut().ok_or("Failed to open stdin")?;
		stdin.write_all(input_data.as_bytes()).await?;
		
		// Await the command to complete
		let output = cmd.wait_with_output().await?;
		
		let result = String::from_utf8(output.stdout)?;
		
		let index_str = result.split('-').next().unwrap_or("0");
		
		let index = index_str.parse::<usize>().unwrap_or(0);
		
		Ok(self.entries[index].clone())
	}
}



#[tokio::main]
async fn main() {
	// Create a channel for communication between the grab callback and the main task
	let (tx, mut rx) = mpsc::channel(100);

	// Define the callback function
	let callback = move |event: Event| -> Option<Event> {
		let tx = tx.clone();
		task::spawn(async move {
			match event.event_type {
				EventType::KeyPress(Key::KeyC) => {
					tx.send(Hotkey::KeyC).await.unwrap();
				}
				EventType::KeyPress(Key::KeyV) => {
					tx.send(Hotkey::KeyV).await.unwrap();
				}
				EventType::KeyPress(Key::MetaLeft) => {
					tx.send(Hotkey::ModPressed).await.unwrap();
				}
				EventType::KeyRelease(Key::MetaLeft) => {
					tx.send(Hotkey::ModReleased).await.unwrap();
				}
				EventType::KeyPress(Key::ControlLeft) => {
					tx.send(Hotkey::ControlPressed).await.unwrap();
				}
				EventType::KeyRelease(Key::ControlLeft) => {
					tx.send(Hotkey::ControlReleased).await.unwrap();
				}
				_ => {}
			}
		});
		Some(event)
	};
	
	let _ = simulate(&EventType::KeyRelease(Key::Return));

	// Spawn a task to handle the event grabbing
	tokio::spawn(async move {
		loop {
			if let Err(error) = grab(callback.clone()) {
				println!("Error: {:?}", error);
				return
			}
		}
	});

	let mut ctrl_pressed = false;
	let mut mod_pressed = false;
	let mut history = History::new();
	
	// Main task to handle state updates
	while let Some(hotkey) = rx.recv().await {
		match hotkey {
			Hotkey::ControlPressed => {
				ctrl_pressed = true;
			}
			Hotkey::ControlReleased => {
				ctrl_pressed = false;
			}
			Hotkey::ModPressed => {
				mod_pressed = true;
			}
			Hotkey::ModReleased => {
				mod_pressed = false;
			}
			Hotkey::KeyC => {
				if ctrl_pressed {
					history.update();
				}
			}
			Hotkey::KeyV => {
				if mod_pressed {
					history.paste().await;
				}
			}
		}
	}
}
