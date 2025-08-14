use chip8sys::chip8::Chip8Sys;
use egui::Color32;

// if we add new fields, give them default values when deserializing old state
pub struct Chip8App {
    // Example stuff:
    label: String,
    value: f32,

    // Actual Chip 8 Implementation
    // #[serde(skip)] // This how you opt-out of serialization of a field
    chip8: Chip8Sys,
}

impl Default for Chip8App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            chip8: Chip8Sys::new_chip_8(),
        }
    }
}

impl Chip8App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut result: Chip8App = Default::default();
        // result.chip8.load_rom("roms/2-ibm-logo.ch8".to_string());
        // result.chip8.load_rom("roms/1-chip8-logo.ch8".to_string());
        // result.chip8.load_rom("roms/3-corax+.ch8".to_string());
        // result.chip8.load_rom("roms/5-quirks.ch8".to_string());
        // When running quirks rom hardcode this memory spot to auto run Chip-8
        // result.chip8.memory[0x1FF] = 1;
        result.chip8.load_rom("roms/walking_man.ch8".to_string());
        result
    }
}

impl eframe::App for Chip8App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        self.chip8.run();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chip-8 Display".to_string());
            let painter = ui.painter();

            /*
            painter.rect_filled(
                egui::Rect {
                    min: egui::Pos2 { x: 25.0, y: 50.0 },
                    max: egui::Pos2 { x: 50.0, y: 75.0 },
                },
                0,
                Color32::BLUE,
            );
            */

            let width = 10.0;
            let mut row = 0.0;
            let mut col = 0.0;
            let x_off = 50.0;
            let y_off = 45.0;
            let col_count = 8;

            for (n, px) in self.chip8.frame_buffer.iter().enumerate() {
                if n % col_count == 0 {
                    row += width;
                    col = 0.0;
                }
                let mut bit_stream: Vec<bool> = Vec::new();

                for b in 0..8 {
                    bit_stream.push(((px << b) & 0b1000_0000) == 0b1000_0000);
                }
                for cell in bit_stream {
                    let x_start = x_off + (col * width);
                    let y_start = y_off + row;
                    let color: Color32;
                    if cell {
                        color = Color32::GREEN;
                    } else {
                        color = Color32::BLACK;
                    };
                    painter.rect_filled(
                        egui::Rect {
                            min: egui::Pos2 {
                                x: x_start,
                                y: y_start,
                            },
                            max: egui::Pos2 {
                                x: x_start + width,
                                y: y_start + width,
                            },
                        },
                        0.0,
                        color,
                    );
                    col += 1.0;
                }
            }
            ctx.request_repaint();
        });

        egui::SidePanel::right("Register Info").show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Program Counter: {}", &self.chip8.program_counter));
            });
            ui.horizontal(|ui| {
                ui.label(format!("Register I: {}", &self.chip8.register_i));
            });
            for n in 0..0xF {
                ui.label(format!("Register {}: {}", n, &self.chip8.register[n]));
            }
        });
    }
}
