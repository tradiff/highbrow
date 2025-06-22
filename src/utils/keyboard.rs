use gdk4::{Display, ModifierType};
use gtk4::glib::MainLoop;
use gtk4::{Application, ApplicationWindow, prelude::*};
use std::sync::{Arc, Mutex};

// Check if modifier keys (Alt or Ctrl) are currently pressed
pub fn is_modifier_pressed(app: &Application) -> bool {
    let result_state = Arc::new(Mutex::new(ModifierType::empty()));
    let result_clone = result_state.clone();

    // Create a glib MainLoop to block until focus event
    let main_loop = MainLoop::new(None, false);
    let loop_clone = main_loop.clone();

    // Build a tiny window to trigger focus
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1)
        .default_height(1)
        .decorated(false)
        .build();
    window.show();

    // Listen for the window becoming active
    window.connect_notify(Some("is-active"), move |win, pspec| {
        if pspec.name() == "is-active" && win.is_active() {
            // Capture modifier state from GDK
            if let Some(display) = Display::default() {
                if let Some(seat) = display.default_seat() {
                    if let Some(keyboard) = seat.keyboard() {
                        let state = keyboard.modifier_state();
                        *result_clone.lock().unwrap() = state;
                    }
                }
            }
            // Close the temporary window and quit the blocking MainLoop
            win.close();
            loop_clone.quit();
        }
    });

    // Run until quit() is called
    main_loop.run();

    // Retrieve and test the result
    let state = *result_state.lock().unwrap();
    state.contains(ModifierType::ALT_MASK) || state.contains(ModifierType::CONTROL_MASK)
}
