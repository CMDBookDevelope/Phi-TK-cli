// Minimal IPC channel: the CLI prints machine-readable events on stdout, exactly
// like the desktop app's `ipc::client`. The GUI wrapper parses these lines; when
// running headless they are simply part of the console output.
pub mod client {
    use serde::Serialize;

    pub fn send<T: Serialize>(value: T) {
        match serde_json::to_string(&value) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Serialization error: {}", e),
        }
    }
}
