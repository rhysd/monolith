// This is a dummy client to align interfaces with upstream. In upstream codebase, many functions
// take reqwest::blocking::Client as parameter, but in Wasm async functions, no client is defined
// because we use fetch() JavaScript function directly.
// To fill the gap and align interfaces to upstream for better maintenanceability, make a dummy
// struct to represent a client.
//
// Some context information may be stored in this struct.
// e.g.
//   - Configurable timeout
//   - Error handling function
pub struct Client;

impl Client {
    pub fn new() -> Client {
        Client
    }
}
