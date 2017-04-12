#![feature(asm)]

#[macro_export]
macro_rules! releasetag {
    
    // The argument of releasetag!() must be a byte-string (array) of 8bit elements,
    // instead of unicode string. The release-tag may be a byte-string of any size. In case the
    // the release-tag has byte-length of 120, the occupied stack-size would be 125 bytes on
    // 32bit arch (or 129 bytes on 64bit architecture) adding leading and trailing null-characters.
    //   
    // The feature is restricted to byte-strings as unicode strings might contain 
    // non-printable characters causing 'strings' command line tool to print fragments of
    // tag only (causing loss of information)
    
    ($tag:expr) => {
        let _tag = {
            struct EmbeddedOctetBuf<T> {
                pad0 : usize, // leading '\0' using default integer alignment
                data : T,  // user defined array or static string
                pad1 : u8, // trailing '\0' consecutive to array
            }

            // prevent optimizer removing the object from stack
            impl<T> Drop for EmbeddedOctetBuf<T> {
                fn drop(&mut self) {
                    // nop to force linker to preserve the variable on stack
                    unsafe { asm!("" : : "r"(self)) }
                }
            }

            // define stacktag with aligned leading and trailing \0
            let stacktag = EmbeddedOctetBuf{pad0: 0, data : *$tag, pad1: 0x0u8};

            stacktag
        };
        // no-op preventing deadcode compiler warnings
        unsafe { asm!("" : : "r"(&_tag.pad0)) }
        unsafe { asm!("" : : "r"(&_tag.data)) }
        unsafe { asm!("" : : "r"(&_tag.pad1)) }
    }
}

#[cfg(test)]
mod tests {
    // just testing syntax from within lib, please use test/run_test.sh which is
    // starting a small application. The script will cause the appl. to core-dump with 
    // signal 6 and parse the core file for releasetag strings. If the tags can not be found
    // in the core file the feature did fail.
    #[test]
    fn valid_macro() {
        #[allow(dead_code)]
        releasetag!(b"TAG1=123");
        releasetag!(b"TAG2=ABC");
        releasetag!(&[0x42u8, 0x55u8, 0x49u8, 0x4cu8, 0x44u8, 0x5fu8]); // "BUILD_"
    }
}
