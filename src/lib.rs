#![feature(asm)] 

#[macro_export]
macro_rules! releasetag {
    
    // The argument of releasetag!() must be a byte-string (array) of 8bit elements, 
    // instead of unicode string. The release-tag may be a byte-string of any size. In case the 
    // the release-tag has byte-length of 126, the occupied stack-size would be 128 bytes, 
    // adding leading and trailing null-characters.
    //   
    // The feature is restricted to byte-strings as unicode strings might contain 
    // non-printable characters causing 'strings' command line tool to print fragments of
    // tag only (causing loss of information)
    
    ($tag:expr) => {
        let _tag = {
            // CAPACITY incl leading and trailing \0 (+2)
            const CAPACITY : usize = byte_size_of!($tag) + 2;

            // Analog to C, declare a type of fixed size, providing a byte-wise copy operator
            struct FixedSize {
               data: [u8;  CAPACITY],
            }

            impl Drop for FixedSize {
                fn drop(&mut self) {
                    // nop to force linker to preserve the variable on stack
                    unsafe { asm!("" : : "r"(self)) }
                }
            }

            // const data will not be on stack. As command line tool 'strings' will search for
            // null-terminated printable chars, add leading and trailing null-char \0 (aka 0u8)
            const CONST_DATA : & 'static [u8;  CAPACITY] = concat_bytes!([0u8],$tag,[0u8]);

            // Create instance of FixedSize and copy chunk byte-wise onto stack,
            // the bounds are verified during compile time.
            let stacktag = FixedSize{data : *CONST_DATA};

            stacktag
        };
        // no-op preventing deadcode compiler warnings
        unsafe { asm!("" : : "r"(&_tag.data)) }
    }
}

#[cfg(test)]
mod tests {
    // just testing syntax from within, please use test/run_test.sh which is 
    // starting a small application. The script will cause the appl. to core-dump with 
    // signal 6 and parse the core file for releasetag strings. If the tags can not be found
    // in the core file the feature did fail.
    #[test]
    fn valid_macro() {
        #[allow(dead_code)]
        releasetag!(b"TAG1=123");
        releasetag!(b"TAG2=ABC");
    }
}