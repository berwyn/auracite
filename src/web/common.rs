macro_rules! static_resource {
    ($fnname:ident, $debug_path:expr, $compiled_path:expr) => {
        #[cfg(debug_assertions)]
        pub fn $fnname() -> Cow<'static, [u8]> {
            use std::borrow::Cow;
            use std::fs::File;
            use std::io::Read;

            let mut buf = vec![];
            File::open($debug_path).unwrap().read_to_end(&mut buf).unwrap();
            Cow::Owned(buf)
        }

        #[cfg(not(debug_assertions))]
        #[inline]
        pub fn $fnname() -> Cow<'static, [u8]> {
            use std::borrow::Cow;

            Cow::Borrowed(include_bytes!($compiled_path))
        }
    }
}
