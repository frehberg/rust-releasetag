/// Placing a formatted byte-string on process-stack eventually propagating into crash core-dumps.
///
/// This macro takes a number of comma-separated byte string literals, byte-arrays or 
/// content of includes forming a single byte-sequence on stack with leading and trailing 
/// zero-bytes, terminating the strings.
///
/// These arrays and included files must not cotain and zero-string. The code would compile, and the 
/// data would reside in core files, but tools like 'strings' would extract not a single string but
/// multiple strings.
///
/// That said: if you do not depend on tools like 'strings' extracting zero-terminated strings from binaries, 
/// your tag and files may be formed by any kind of byte-sequence.
///
/// # Example
///
/// ```rust
/// #[macro_use(releasetag)]
/// extern crate releasetag;
/// 
/// fn main() {
///     releasetag!(b"BUILD_TAG=pre");
///     releasetag!(b"BUILD_TAG=",     b"MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK");
/// 
///     // Including data from one or multiple files or compile time env params.
///     // Note: the content mustn't contain any newline or linebreak as those would interfer with zero terminated strings!!
///     let mut version: [u8; 5] = [0; 5];
///     version.copy_from_slice(env!("CARGO_PKG_VERSION").as_bytes());
///     releasetag!(b"BUILD_VERSION=", &version);
///     releasetag!(b"BUILD_TAG=",     &version, b"/", include_bytes!("../AUTHOR"));
///    
///     // or as byte array
///     releasetag!(b"BUILD_HOST=",    &[0x30u8, 0x31u8, 0x33u8]);
///  
///     // your application logic here
/// }
/// ```
#[macro_export]
macro_rules! releasetag {
    // Thanks to Japaric's help, this crate no longer depends on nightly features.
    ($tag:expr) => {
        let _tag = {
            // Prevent reordering of struct-members
            #[repr(C)]
            struct EmbeddedOctetBuf<T> {
                pad0 : usize, // Leading '\0' using default integer alignment
                txt0 : T,     // User defined array or static string
                pad1 : u8,    // Trailing '\0' consecutive to array
            }

            // Define stacktag, with leading (and aligned) 0 and trailing \0, fencing the string.
            let stacktag = EmbeddedOctetBuf{pad0: 0, txt0 : *$tag, pad1: 0x0u8};

            stacktag
        };
        // "Volatile" will prevent compiler from
        // * optimizing the unused value out
        // * turning the stack value into a static value
        let _myref = unsafe { std::ptr::read_volatile(&&_tag); };
    };

    ($tag:expr, $val1:expr) => {
        let _tag = {
            // Prevent reordering of struct-members
            #[repr(C)]
            struct EmbeddedOctetBuf<T, V> {
                pad0 : usize, // Leading '\0' using default integer alignment
                txt0 : T,     // User defined array or static byte string (byte array [u8])
                txt1 : V,     // User defined array or static byte string (byte array [u8])
                pad1 : u8,    // Trailing '\0' consecutive to array
            }

            // Define stacktag, with leading (and aligned) 0 and trailing \0, fencing the string.
            let stacktag = EmbeddedOctetBuf{pad0: 0, txt0 : *$tag, txt1 : *$val1, pad1: 0x0u8};

            stacktag
        };
        // "Volatile" will prevent compiler from
        // * optimizing the unused value out
        // * turning the stack value into a static value
        let _myref = unsafe { std::ptr::read_volatile(&&_tag); };
    };

    ($tag:expr, $val1:expr, $val2:expr) => {
        let _tag = {
            // Prevent reordering of struct-members
            #[repr(C)]
            struct EmbeddedOctetBuf<T, V, W> {
                pad0 : usize, // Leading '\0' using default integer alignment
                txt0 : T,     // User defined array or static byte string (byte array [u8])
                txt1 : V,     // User defined array or static byte string (byte array [u8])                
                txt2 : W,     // User defined array or static byte string (byte array [u8])
                pad1 : u8,    // Trailing '\0' consecutive to array
            }

            // Define stacktag, with leading (and aligned) 0 and trailing \0, fencing the string.
            let stacktag = EmbeddedOctetBuf{pad0: 0, txt0 : *$tag, txt1 : *$val1, txt2 : *$val2, pad1: 0x0u8};

            stacktag
        };
        // "Volatile" will prevent compiler from
        // * optimizing the unused value out
        // * turning the stack value into a static value
        let _myref = unsafe { std::ptr::read_volatile(&&_tag); };
    };

    ($tag:expr, $val1:expr, $val2:expr, $val3:expr) => {
        let _tag = {
            // Prevent reordering of struct-members
            #[repr(C)]
            struct EmbeddedOctetBuf<T, V, W, X> {
                pad0 : usize, // Leading '\0' using default integer alignment
                txt0 : T,     // User defined array or static byte string (byte array [u8])
                txt1 : V,     // User defined array or static byte string (byte array [u8])                
                txt2 : W,     // User defined array or static byte string (byte array [u8])
                txt3 : X,     // User defined array or static byte string (byte array [u8])
                pad1 : u8,    // Trailing '\0' consecutive to array
            }

            // Define stacktag, with leading (and aligned) 0 and trailing \0, fencing the string.
            let stacktag = EmbeddedOctetBuf{pad0: 0, txt0 : *$tag, txt1 : *$val1, txt2 : *$val2, txt3 : *$val3, pad1: 0x0u8};

            stacktag
        };
        // "Volatile" will prevent compiler from
        // * optimizing the unused value out
        // * turning the stack value into a static value
        let _myref = unsafe { std::ptr::read_volatile(&&_tag); };
    };
}


#[cfg(test)]
mod tests {
    /// Testing syntax only, please use test/run_test.sh testing the successfull extraction from core-file.
    #[test]
    fn valid_macro() {
        #[allow(dead_code)]
        releasetag!(b"TAG1=123");
        releasetag!(b"TAG2", b"ABC");
        releasetag!(b"TAG3", b"=", b"ABC" );
        releasetag!(b"TAG4", b"[", &[0x30u8, 0x31u8, 0x33u8], b"]");
   	let mut version: [u8; 5] = [0; 5];
    	version.copy_from_slice(env!("CARGO_PKG_VERSION").as_bytes());
    	releasetag!(b"TAG4=", &version);
        releasetag!(b"TAG4", b"[", include_bytes!("../AUTHOR"), b"]");
    }
}
