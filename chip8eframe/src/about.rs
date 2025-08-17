pub struct About {}
impl About {
    pub fn about() -> String {
        "The Chip-8 is an emulator. This one is written in rust and presented with egui."
            .to_string()
    }
}
