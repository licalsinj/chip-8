use chip8sys::chip8::Chip8Sys;
use chip8sys::chip8error::Chip8Error;
use egui::Color32;
use egui_extras::{Column, TableBuilder};

// if we add new fields, give them default values when deserializing old state
pub struct Chip8App {
    chip8: Chip8Sys,
    zoom: f32,
    background_color: Color32,
    pixel_color: Color32,
}

impl Default for Chip8App {
    fn default() -> Self {
        Self {
            // Example stuff:
            chip8: Chip8Sys::new_chip_8(),
            zoom: 20.0,
            background_color: Color32::BLACK,
            pixel_color: Color32::GREEN,
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
    fn table(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        // table = table.sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Index");
                });
                header.col(|ui| {
                    ui.strong("Register");
                });
                header.col(|ui| {
                    ui.strong("Stack");
                });
                header.col(|ui| {
                    ui.strong("Keys");
                });
            })
            .body(|mut body| {
                for row_index in 0..self.chip8.register.len() {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", row_index));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", self.chip8.register[row_index]));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:04X}", self.chip8.stack[row_index]));
                        });
                        row.col(|ui| {
                            if self.chip8.keys[row_index] {
                                ui.label("Pressed");
                            } else {
                                ui.label("");
                            }
                        });
                    });
                }
            });
    }
}

impl eframe::App for Chip8App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        // TODO: Not sure how I want to handle all these yet...
        // maybe log them in their own window?
        match self.chip8.run() {
            Ok(_) => (),
            Err(e) => match e {
                // if the N of 0xN___ is invalid it will return this and the N provided
                Chip8Error::InvalidFirstByte(_) => (),
                // If the X register should be <= 0xF
                Chip8Error::InvalidRegisterX(_) => (),
                // if the N in 0x8XYN is invalid it will return this and the N provided
                Chip8Error::Invalid0x8XYN(_) => (),
                // if the N in 0x8XYN is invalid it will return this and the N provided
                Chip8Error::Invalid0xENNN(_, _) => (),
                // if the N in 0x8XYN is invalid it will return this and the N provided
                Chip8Error::Invalid0xFNNN(_, _) => (),
                // If the register we're waiting for is somehow > 0xF
                Chip8Error::InvalidWaitRegister(_) => (),
            },
        }

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

            let width = self.zoom;
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
                        color = self.pixel_color;
                    } else {
                        color = self.background_color;
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

        egui::Window::new("Compute Info").show(ctx, |ui| {
            ctx.set_pixels_per_point(2.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.horizontal(|ui| {
                    ui.label(format!("Program Counter: {}", &self.chip8.program_counter));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Register I: {}", &self.chip8.register_i));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Stack Pointer: {}", &self.chip8.stack_pointer));
                });

                ui.separator();
                self.table(ui);
                ui.separator();
            });
        });
        egui::Window::new("Screen Config").show(ctx, |ui| {
            ctx.set_pixels_per_point(2.0);
            ui.add(egui::Slider::new(&mut self.zoom, 0.0..=25.0).text("Zoom: "));
            ui.label("Pixel: ");
            ui.color_edit_button_srgba(&mut self.pixel_color);
            ui.label("Background: ");
            ui.color_edit_button_srgba(&mut self.background_color);
        });
    }
}
