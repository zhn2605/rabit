mod file_ops;
mod input;

use crate::file_ops::FileManager;

fn main() {
    let file_manager = FileManager::new();
    // let input_handler = input
    
    loop {
        // Eventually change this to raw keyboard shortcuts with crossterm
        file_manager.update();

    }


    println!("Goodbye!")
}