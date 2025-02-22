use motoko::Share;
use motoko_proc_macro::parse_static;
use wasm_bindgen::prelude::*;

mod canvas;
mod console;
mod context;
mod document;
mod event;
mod gamepad;
mod navigator;
mod window;

mod movm;

//#[macro_use]
use motoko::{
    dynamic::{Dynamic, Result},
    value::Value_,
};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    web_sys::console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn draw_on_canvas(canvas_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let window_value = window::WindowValue {
        window: window.clone(),
    }
    .into_value()
    .share();

    let document = window.document().expect("should have a document on window");

    let document_value: Value_ = document::DocumentValue {
        document: document.clone(),
    }
    .into_value()
    .share();

    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let canvas2 = canvas::CanvasValue {
        canvas: canvas.clone(),
    };
    let canvas_value: Value_ = canvas2.into_value().share();

    //
    // Now we have a Motoko value for a CanvasValue that
    // we can implement with the motoko::Dynamic trait.
    // It will draw on the actual HTML canvas, and be
    // scriptable with Motoko code running in the VM.
    //

    // PROGRAM as Motoko:
    // let c = canvas.getContext("2d");
    // c.beginPath();
    // c.arc(137.0, 137.0, 42.666, 0.0, 3.0 * std::f64::consts::PI);
    // c.stroke();
    //
    let program = parse_static!("var frames = 0; ignore window.requestAnimationFrame(func(t){ frames := frames + 1; let gs = window.navigator.getGamepads(); for (g in gs.iter()) { } }); window.addEventListener(\"keydown\", func(e){ consoleLog(\"Motoko key press\"); }); window.addEventListener(\"click\", func(e){ consoleLog(\"Motoko mouse click\"); }); consoleLog(\"hello from Motoko\"); let c = canvas.getContext(\"2d\"); consoleLog(\"hello from Motoko 2\"); consoleLog(\"hello from Motoko 3\"); c.beginPath(); consoleLog(\"hello from Motoko 4\"); c.arc(137.0, 137.0, 42.666, 0.0, 9.42); c.stroke(); consoleLog(\"hello from Motoko 5\"); var x = 666;").clone();

    movm::update(|core| {
        core.eval_open_block(
            vec![
                ("canvas", canvas_value),
                (
                    "consoleLog",
                    console::ConsoleLogValue {}.into_value().share(),
                ),
                ("document", document_value),
                ("window", window_value),
            ],
            program,
        )
        .expect("program evaluation.");
    });

    web_sys::console::log_1(&JsValue::from_str(format!("{:?}", movm::get()).as_str()));

    /*
        PROGRAM as Rust:
        --------------------
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        context.begin_path();
        context.arc(137.0, 137.0, 42.666, 0.0, 3.0 * std::f64::consts::PI)?;
        context.stroke();
    */
    Ok(())
}
