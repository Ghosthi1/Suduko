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
}

// Initializes the struct variables
impl Default for MyApp {
    fn default() -> Self {
        Self {
            board: [[0; 9]; 9],
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
            
            // 2. Handle user input
            if ui.button("Reset").clicked() {
                self.board = [[0; 9]; 9];  // Modify app state
            }

            // Draws the board
            if ui.button("Draw Board").clicked() {
                draw_board();
            }

        });
    }
}

fn draw_board(){
    
}
