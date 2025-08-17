pub struct ConfigWindow<'a> {
    pub display: bool,
    pub name: String,
    pub window: egui::Window<'a>,
}
impl<'a> ConfigWindow<'a> {
    pub fn new_screen_config() -> Self {
        let name = "Screen Config";
        Self {
            display: false,
            name: name.to_string(),
        }
    }
}
