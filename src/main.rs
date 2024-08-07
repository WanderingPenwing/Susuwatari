use std::process::Command;
use std::process::Stdio;
use arboard::Clipboard;

use std::io::Error;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


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
		println!("new entry");
		self.entries.insert(0, clip);
	}
	
	fn paste(&mut self) {
		if self.entries.len() == 0 {
			return
		}
		println!("pasting");
		match self.select_entry() {
			Ok(entry) => {
				let _ = self.clipboard.set_text(entry);
			}
			Err(why) => {
				eprintln!("error pasting clipboard : {}", why);
			}
		}
	}
	
	fn select_entry(&self) -> Result<String, Error> {
		
		let mut formated_data : Vec<String> = vec![];
		for (index, entry) in self.entries.iter().enumerate() {
			formated_data.push(format!("{}- {}",index, entry.split('\n').next().unwrap_or("err")));
		}

		let input_data = formated_data.join("\n");
		
		let mut cmd = Command::new("sh")
	        .arg("-c")
	        .arg("marukuru -l 10")
	        .stdin(Stdio::piped())
	        .stdout(Stdio::piped())
	        .spawn()?;
		
		let stdin = cmd.stdin.as_mut().ok_or_else(|| Error::new(std::io::ErrorKind::Other, "Failed to open stdin"))?;
	    stdin.write_all(input_data.as_bytes())?;
	
	    // Wait for the command to complete and get the output
	    let output = cmd.wait_with_output()?;
	
	    // Convert the output to a string
	    let result = String::from_utf8(output.stdout).unwrap_or("".to_string());
	
	    // Parse the result
	    let index_str = result.split('-').next().unwrap_or("0");
	    let index = index_str.parse::<usize>().unwrap_or(0);
		
		Ok(self.entries[index].clone())
	}
}



fn main() {
	let mut history = History::new();

	let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&term)).unwrap();

    loop {
    	while !term.load(Ordering::Relaxed) {
    		std::thread::sleep(std::time::Duration::from_secs(2));
    		history.update();
    	}

    	history.paste();

    	term.store(false, Ordering::Relaxed);
    }
}
