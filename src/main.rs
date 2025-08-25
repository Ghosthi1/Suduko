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
    board: [[u8; 9]; 9],
    board_size: u8,
    grid_size_input: String,
}

// Initializes the struct variables
impl Default for MyApp {
    fn default() -> Self {
        Self {
            board: [[0; 9]; 9],
            board_size: 9,
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
                // Reset board
                if ui.button("Reset").clicked() {
                    self.board = [[0; 9]; 9];  // Modify app state
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
                        self.board_size = num;
                    }
                }


            });


        });
    }
}

fn draw_board(){
    
}
