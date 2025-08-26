use std::{ vec};

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
    sub_grid_size: u8,

    grid_size_input: String,
    grid_size_in: bool,
}

// Initializes the struct variables
impl Default for MyApp {
    fn default() -> Self {
        // 9 by 9 default
        let default_size: usize = 9;
        Self {
            board: vec![vec![0; default_size]; default_size],
            board_size: default_size as u8,
            sub_grid_size: (default_size as f32).sqrt() as u8,
            grid_size_input: String::new(),
            grid_size_in: false,
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

                // Takes grid size form user
                ui.label("Enter grid size:");
                ui.text_edit_singleline(&mut self.grid_size_input);
                if ui.button("Submit").clicked() {
                    if let Ok(num) = self.grid_size_input.parse::<u8>() {
                        let sqrt_num = (num as f32).sqrt();
                        // Check if it's a perfect square
                        if num > 0 && num <= 25 && sqrt_num.fract() == 0.0 {
                            self.board_size = num;
                            self.sub_grid_size = sqrt_num as u8;
                            self.board = vec![vec![0; self.board_size as usize]; self.board_size as usize];
                            self.grid_size_input.clear();
                            self.grid_size_in = true;
                        } else {
                            
                        }
                    }
                }
            });

            // Call the draw_board function
            if self.grid_size_in {
                draw_board(self, ui);
            } else {
                ui.label("Please enter a grid size to start.");
            }
        });
    }
}

fn draw_board(app: &mut MyApp, ui: &mut egui::Ui) {
    //calculates the sub grid amount of rows and colms
    let sub_grid_per_side = app.board_size / app.sub_grid_size;  
    
    // outer grid for sub grids
    egui::Grid::new("outer sudoku grid")
        .num_columns(sub_grid_per_side as usize)
        .spacing([8.0, 8.0]) // Spacing between sub-grids
        .show(ui, |ui| {
            for sub_row in 0..sub_grid_per_side {
                for sub_col in 0..sub_grid_per_side {

                    //inner grid for indv cells
                    egui::Grid::new(format!("sub grid {}, {}", sub_row, sub_col))
                        .num_columns(app.sub_grid_size as usize)
                        .spacing([1.0,1.0]) //small spacing
                        .show(ui, |ui|{
                            for cell_row in 0..app.sub_grid_size{
                                for cell_col in 0..app.sub_grid_size{
                                    
                                    //calculate actual board position
                                    let actual_row = (sub_row * app.sub_grid_size + cell_row) as usize;
                                    let actual_col = (sub_col * app.sub_grid_size + cell_col) as usize;

                                    let cell_value = if app.board[actual_row][actual_col] == 0 {
                                        " ".to_string()
                                    } else {
                                        app.board[actual_row][actual_col].to_string()
                                    };

                                    let button = egui::Button::new(&cell_value)
                                        .min_size(egui::vec2(35.0, 35.0));

                                    if ui.add(button).clicked() {
                                        app.board[actual_row][actual_col] += 1;
                                        if app.board[actual_row][actual_col] > app.board_size {
                                            app.board[actual_row][actual_col] = 0;
                                        }
                                    }

                                }
                                ui.end_row();
                            }
                        });
                }
                ui.end_row();
            }
        });
}

