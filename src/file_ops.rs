use std::fs;
use dirs_next::home_dir;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

pub struct FileManager {
    curr_dir : PathBuf,
}

impl FileManager {
    pub fn new() -> Self {
        let dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self { curr_dir: dir }
    }

    pub fn update(&self) {
        // Get current directory
        println!("{}", self.curr_dir.display());

        // Print current directory with files    
        match fs::read_dir(&self.curr_dir) {
            Ok(entries) => {
                println!("Files and directories:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("\tᶫ{}", entry.path().display());
                    }
                    else {
                        eprintln!("\tᶫ<error>");
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory: {}", e),
        }
    }

    pub fn fuzzy_search(&self) {

    }
}
