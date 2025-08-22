#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate console_error_panic_hook;
use std::panic;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Chip-8 Emulator",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(chip8eframe::Chip8App::new(
                cc,
                Ok(stream_handle.mixer()),
            )))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Lets me see the panic message in browser's console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    use eframe::wasm_bindgen::JsCast as _;

    // How I pull audio for desktop. Leaving here for now so I know what to do if I need it.
    // let _stream_handle: rodio::OutputStream =
    //      rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    /* let stream_handle: rodio::OutputStream = rodio::OutputStreamBuilder::from_default_device()
        .expect("should find default device")
        .open_stream()
        .expect("Should open default audio stream");
    // */

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    Ok(Box::new(chip8eframe::Chip8App::new(
                        cc,
                        Err("No Mixer".to_string()),
                        // Tried just connecting straight to the stream to avoid borrow issues
                        // However wasm doesn't have default audio devices...
                        // Leaving here for reference
                        /*
                        rodio::OutputStreamBuilder::from_default_device()
                            .expect("should find default device")
                            .open_stream()
                            .expect("Should open default audio stream")
                            .mixer(),
                        */
                    )))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
