use std::{ vec};

use eframe::egui;
use egui::ViewportBuilder;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // Configure the native window options
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([750.0, 500.0]),
        ..Default::default()
    };

    // eframe::run_native creates and runs a native desktop application
    eframe::run_native(
        "Suduko",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))), // <-- wrap in Ok()
    )
}

// MyApp struct holds the application state
struct MyApp {
    board: Vec<Vec<u8>>, 
    board_size: u8,
    sub_grid_size: u8,

    grid_size_in: bool,

    selected_grid_size: u8,
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
            grid_size_in: false,
            selected_grid_size: default_size as u8,
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

                //dropdown for grid size selection
                ui.label("Selected Grid Size:");

                egui::ComboBox::from_label("")
                    .selected_text(format!("{}x{}", self.selected_grid_size, self.selected_grid_size))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_grid_size, 4, "4x4");
                        ui.selectable_value(&mut self.selected_grid_size, 9, "9x9");
                        ui.selectable_value(&mut self.selected_grid_size, 16, "16x16");
                        ui.selectable_value(&mut self.selected_grid_size, 25, "25x25");
                    });

                if ui.button("Create Grid").clicked() {
                    self.board_size = self.selected_grid_size;
                    self.sub_grid_size = (self.selected_grid_size as f32).sqrt() as u8;
                    self.board = vec![vec![0; self.board_size as usize]; self.board_size as usize];
                    self.grid_size_in = true;
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

                                    //checks if valid
                                    let is_valid = check_valid(&app.board, actual_row, actual_col, app.board_size, app.sub_grid_size);

                                    let button = egui::Button::new(
                                        //xet text color
                                        egui::RichText::new(&cell_value).color(egui::Color32::BLACK)
                                        )
                                        .min_size(egui::vec2(35.0, 35.0))
                                        .fill(
                                            if is_valid { egui::Color32::WHITE } 
                                            else { egui::Color32::RED });

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

fn check_valid(board: &Vec<Vec<u8>>, row: usize, col: usize, boarder_size: u8, sub_grid_size: u8) -> bool{
    let value = board[row][col];

    // if empty it is valid
    if value == 0 {
        return true;
    }

    //checks rows for duplicates
    for c in 0..boarder_size as usize{
        if c != col && board[row][c] == value{
            return false;
        }
    }

    //checks col for duplocates
    for r in 0..boarder_size as usize{
        if r != row && board[r][col] == value{
            return false;
        }
    }

    //checks sub grid for duplicates 
    // rounds towards zero and finds start of subgrid so 9x9 0,3,6
    let sub_row_start = (row /sub_grid_size as usize) * sub_grid_size as usize;
    let sub_col_start = (col/ sub_grid_size as usize) * sub_grid_size as usize;

    for r in sub_row_start..sub_row_start + sub_grid_size as usize{
        for c in sub_col_start..sub_col_start + sub_grid_size as usize{
            if (r != row || c != col) && board[r][c] == value{
                return  false;
            }
        }
    }

    true

}