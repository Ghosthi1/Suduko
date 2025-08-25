use std::{default, thread::sleep, vec};

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // Configure the native window options
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(750.0, 500.0)), // Set window size to 1000x1000 pixels
        ..Default::default()
    };

    // eframe::run_native creates and runs a native desktop application
    eframe::run_native(
        "Suduko",
        options,
        Box::new(|_cc| Box::new(MyApp::default())), // _cc is the creation context, contains OpenGL context, etc.
    )
}

// MyApp struct holds the application state
struct MyApp {
    board: Vec<Vec<u8>>, 
    board_size: u8,
    grid_size_input: String,
}

// Initializes the struct variables
impl Default for MyApp {
    fn default() -> Self {
        // 9 by 9 default
        let default_size: usize = 9;
        Self {
            board: vec![vec![0; default_size]; default_size],
            board_size: default_size as u8,
            grid_size_input: String::new(),
        }
    }
}

// Implement the eframe::App trait for MyApp
// This trait defines how the app should update and render
impl eframe::App for MyApp {
    // update() is called every frame - this is where you define your UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Create panels/windows
        egui::CentralPanel::default().show(ctx, |ui| {

            //makes buttons horizontal
            ui.horizontal(|ui| {
                // Reset board to 0
                if ui.button("Reset").clicked() {
                    self.board = vec![vec![0; self.board_size as usize]; self.board_size as usize];  // Modify app state
                }

                // Draws the board
                if ui.button("Draw Board").clicked() {
                    draw_board();
                }

                // Takes grid size form user
                ui.label("Enter grid size:");
                ui.text_edit_singleline(&mut self.grid_size_input);
                if ui.button("Submit").clicked() {
                    if let Ok(num) = self.grid_size_input.parse::<u8>() {
                        //size limit 
                        if num > 0 && num <= 30 {
                            self.board_size = num;  
                            self.board = vec![vec![0; self.board_size as usize]; self.board_size as usize]; //Resizes the board 
                            self.grid_size_input.clear();
                        }
                    }
                }


            });


        });
    }
}

fn draw_board(){
    
}
