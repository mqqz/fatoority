use diplomat_runtime::DiplomatWrite;

pub fn written(f: impl FnOnce(&mut DiplomatWrite)) -> String {
    struct Writer(*mut DiplomatWrite);
    impl Drop for Writer {
        fn drop(&mut self) {
            unsafe {
                diplomat_runtime::diplomat_buffer_write_destroy(self.0);
            }
        }
    }
    let out = Writer(diplomat_runtime::diplomat_buffer_write_create(0));
    unsafe {
        f(&mut *out.0);
        std::str::from_utf8((*out.0).as_bytes()).unwrap().to_owned()
    }
}

/// Reads a `fn(&self, out: &mut DiplomatWrite) -> Option<()>` optional string.
#[allow(dead_code)] // Not every test crate reads optional strings.
pub fn optional_written(f: impl FnOnce(&mut DiplomatWrite) -> Option<()>) -> Option<String> {
    let mut present = None;
    let value = written(|out| present = f(out));
    present.map(|()| value)
}
