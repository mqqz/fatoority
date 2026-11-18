//! Diplomat invoice bindings. Core types remain independent of the binding framework.

struct ErrorData {
    code: i32,
    message: String,
    details: String,
}

pub(crate) fn core_error(error: impl Into<fatoora_core::Error>) -> Box<ffi::BindingError> {
    let error = error.into();
    Box::new(ffi::BindingError(ErrorData {
        code: error.kind() as i32,
        message: error.to_string(),
        details: error.details_json(),
    }))
}

pub(crate) fn context_error(
    error: impl Into<fatoora_core::Error>,
    context: &str,
) -> Box<ffi::BindingError> {
    let mut error = core_error(error);
    error.0.message = format!("{context}: {}", error.0.message);
    error
}
pub(crate) fn local_error(code: i32, message: &str) -> Box<ffi::BindingError> {
    Box::new(ffi::BindingError(ErrorData {
        code,
        message: message.into(),
        details: "{\"type\":\"binding_error\"}".into(),
    }))
}

pub(crate) fn text(bytes: &diplomat_runtime::DiplomatStr) -> Result<&str, Box<ffi::BindingError>> {
    let value =
        std::str::from_utf8(bytes).map_err(|_| local_error(1, "text must be valid UTF-8"))?;
    if value.contains('\0') {
        return Err(local_error(1, "text must not contain NUL"));
    }
    Ok(value)
}
pub(crate) fn optional_text(
    bytes: Option<&diplomat_runtime::DiplomatStr>,
) -> Result<Option<String>, Box<ffi::BindingError>> {
    bytes.map(|v| text(v).map(str::to_owned)).transpose()
}
pub(crate) fn write(
    value: impl AsRef<str>,
    out: &mut diplomat_runtime::DiplomatWrite,
) -> Result<(), Box<ffi::BindingError>> {
    use std::fmt::Write;
    let value = value.as_ref();
    if value.contains('\0') {
        return Err(local_error(1, "text must not contain NUL"));
    }
    out.write_str(value)
        .map_err(|_| local_error(9, "output write failed"))
}
/// Writes an optional string for a `fn(&self, out: &mut DiplomatWrite) -> Option<()>`
/// method, which every generated binding surfaces as a nullable string.
///
/// This shape has no error channel, and needs none: `DiplomatWrite` never
/// fails a write (a failed grow is flagged on the writer and reported by the
/// binding reading it back). Embedded NUL characters are written as-is; only
/// the C facade needs NUL-free output, and it rejects them itself.
pub(crate) fn write_optional(
    value: Option<impl AsRef<str>>,
    out: &mut diplomat_runtime::DiplomatWrite,
) -> Option<()> {
    use std::fmt::Write;
    let _ = out.write_str(value?.as_ref());
    Some(())
}
/// Maps a core [`Environment`](fatoora_core::config::Environment) to its FFI
/// enum. The core enum is `#[non_exhaustive]`; an environment these bindings do
/// not know yet is reported as an internal error rather than silently mapped
/// to a different environment.
pub(crate) fn environment_to_ffi(
    value: fatoora_core::config::Environment,
) -> Result<ffi::Environment, Box<ffi::BindingError>> {
    use fatoora_core::config::Environment as Core;
    match value {
        Core::NonProduction => Ok(ffi::Environment::NonProduction),
        Core::Simulation => Ok(ffi::Environment::Simulation),
        Core::Production => Ok(ffi::Environment::Production),
        _ => Err(local_error(
            9,
            "environment is not supported by this binding version",
        )),
    }
}
impl From<ffi::Environment> for fatoora_core::config::Environment {
    fn from(value: ffi::Environment) -> Self {
        match value {
            ffi::Environment::NonProduction => Self::NonProduction,
            ffi::Environment::Simulation => Self::Simulation,
            ffi::Environment::Production => Self::Production,
        }
    }
}
pub(crate) fn boundary<T>(
    f: impl FnOnce() -> Result<T, Box<ffi::BindingError>>,
) -> Result<T, Box<ffi::BindingError>> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(payload) => {
            if let Err(nested) =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(payload)))
            {
                std::mem::forget(nested);
            }
            Err(local_error(9, "internal operation panicked"))
        }
    }
}

/// Readers for `DiplomatWrite` outputs in unit tests.
#[cfg(test)]
pub(crate) mod test_support {
    use diplomat_runtime::DiplomatWrite;

    /// Returns what `f` writes into a fresh `DiplomatWrite`.
    pub(crate) fn written(f: impl FnOnce(&mut DiplomatWrite)) -> String {
        struct Writer(*mut DiplomatWrite);
        impl Drop for Writer {
            fn drop(&mut self) {
                // SAFETY: the buffer was created by `diplomat_buffer_write_create`.
                unsafe { diplomat_runtime::diplomat_buffer_write_destroy(self.0) }
            }
        }
        let out = Writer(diplomat_runtime::diplomat_buffer_write_create(0));
        // SAFETY: the buffer stays alive until `out` is dropped.
        unsafe {
            f(&mut *out.0);
            String::from_utf8((*out.0).as_bytes().to_vec()).unwrap()
        }
    }

    /// Reads a `fn(&self, out: &mut DiplomatWrite) -> Option<()>` optional string.
    pub(crate) fn optional_written(
        f: impl FnOnce(&mut DiplomatWrite) -> Option<()>,
    ) -> Option<String> {
        let mut present = None;
        let value = written(|out| present = f(out));
        present.map(|()| value)
    }
}

#[cfg(test)]
mod tests {
    use super::{boundary, environment_to_ffi, ffi::Environment, write_optional};
    use fatoora_core::config::Environment as Core;

    #[test]
    fn environments_round_trip() {
        for env in [
            Environment::NonProduction,
            Environment::Simulation,
            Environment::Production,
        ] {
            let core = Core::from(env);
            assert_eq!(environment_to_ffi(core).ok(), Some(env));
        }
        assert_eq!(Core::from(Environment::Simulation), Core::Simulation);
    }

    #[test]
    fn panic_after_consumption_returns_internal_error() {
        let mut state = Some("owned builder state");
        let error = boundary::<()>(|| {
            let _owned = state.take();
            panic!("operation failed");
        })
        .err()
        .unwrap();
        assert_eq!(error.code(), 9);
        assert!(state.is_none());
    }

    #[test]
    fn panicking_payload_destructor_is_contained() {
        struct Payload;
        impl Drop for Payload {
            fn drop(&mut self) {
                panic!("payload destructor");
            }
        }
        let error = boundary::<()>(|| std::panic::panic_any(Payload))
            .err()
            .unwrap();
        assert_eq!(error.code(), 9);
    }

    #[test]
    fn optional_strings_write_only_when_present() {
        let read = |value: Option<&str>| {
            let out = diplomat_runtime::diplomat_buffer_write_create(0);
            // SAFETY: `out` is a live buffer until the destroy call below.
            unsafe {
                let present = write_optional(value, &mut *out);
                let text = String::from_utf8((*out).as_bytes().to_vec()).unwrap();
                diplomat_runtime::diplomat_buffer_write_destroy(out);
                (present, text)
            }
        };
        assert_eq!(read(None), (None, String::new()));
        assert_eq!(read(Some("")), (Some(()), String::new()));
        assert_eq!(read(Some("a\0b")), (Some(()), "a\0b".into()));
    }
}

#[diplomat::bridge]
#[diplomat::abi_rename = "fatoora_{0}"]
#[diplomat::attr(cpp, namespace = "fatoora")]
pub mod ffi {
    use super::ErrorData;
    use std::fmt::Write;
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::attr(kotlin, error)]
    pub struct BindingError(pub(super) ErrorData);
    impl BindingError {
        pub fn code(&self) -> i32 {
            self.0.code
        }
        pub fn message(&self, out: &mut diplomat_runtime::DiplomatWrite) {
            let _ = out.write_str(&self.0.message.replace('\0', "\\0"));
        }
        pub fn details_json(&self, out: &mut diplomat_runtime::DiplomatWrite) {
            let _ = out.write_str(&self.0.details);
        }
    }
    /// ZATCA environment selecting the API base URL and certificate profile.
    ///
    /// Discriminants match the `u8` codes used before 0.6. .NET names it
    /// `ZatcaEnvironment` so it cannot be confused with `System.Environment`.
    #[derive(Debug, PartialEq, Eq)]
    #[diplomat::attr(dotnet, rename = "ZatcaEnvironment")]
    pub enum Environment {
        NonProduction = 0,
        Simulation = 1,
        Production = 2,
    }
}
