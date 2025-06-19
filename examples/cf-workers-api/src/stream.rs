use web_sys::ReadableStream;

// Global storage for the ReadableStream - safe in single-threaded WASM
static mut GLOBAL_STREAM: Option<ReadableStream> = None;

// Helper functions to safely access the global stream
pub fn set_global_stream(stream: ReadableStream) {
    unsafe {
        GLOBAL_STREAM = Some(stream);
    }
}

pub fn take_global_stream() -> Option<ReadableStream> {
    unsafe { GLOBAL_STREAM.take() }
}
