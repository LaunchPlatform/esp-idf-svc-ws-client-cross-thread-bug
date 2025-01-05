use esp_idf_svc::ws::client::{
    EspWebSocketClient, EspWebSocketClientConfig, FrameType, WebSocketEvent, WebSocketEventType,
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let config = EspWebSocketClientConfig {
        ..Default::default()
    };
    let timeout = Duration::from_secs(10);
    let mut client =
        Arc::new(EspWebSocketClient::new("ws://example.com", &config, timeout, move |event| {
            // Do something with the event
        }).unwrap());

    thread::Builder::new()
        .stack_size(60000)
        .spawn(move || {
        // ^ And here we have the error:
        // `*mut esp_websocket_client` cannot be shared between threads safely
            loop {
                client.send(FrameType::Text(false), "hello".as_bytes());
                thread::sleep(Duration::from_secs(10));
            }
        })
        .unwrap()
        .join()
        .unwrap()
}
