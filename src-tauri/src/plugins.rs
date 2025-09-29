use crate::ShortcutState;

use tauri::Runtime;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub fn global_shortcut_plugin<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    let ctrl_t_shortcut: Shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyT);

    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(ctrl_t_shortcut)
        .unwrap()
        .with_handler(move |app, shortcut, event| {
            if shortcut == &ctrl_t_shortcut {
                match event.state() {
                    ShortcutState::Pressed => {}
                    ShortcutState::Released => {
                        app.exit(0);
                    }
                }
            }
        })
        .build()
}
