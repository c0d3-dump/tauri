use tauri::{Event, Webview, WebviewEvent};

pub fn webview(_window: &Webview, event: &WebviewEvent) {
    dbg!(event);
}

pub fn download_started(event: Event) {
    let data = event.payload();
    dbg!(data);
}
