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
    
    ($tag:expr) => {{
        // CAPACITY incl leading and trailing \0 (+2) 
        const CAPACITY : usize = byte_size_of!($tag) + 2;  
        
        // const data will not be on stack, add leading and trailing null-char \0
        const CONST_DATA : & 'static [u8;  CAPACITY] = concat_bytes!([0u8],$tag,[0u8]);
        
        // local array on stack, must be declared as 'mut', otherwise would
        // be optimized out by compiler
        let mut stacktag : & mut [u8;  CAPACITY] = & mut [0u8; CAPACITY];
        // TODO: find a memcpy expression where compiler would verify matching array sizes. 
        //       Using clone_from_slice will check array-boundaries during runtime. 
        stacktag[..CAPACITY].clone_from_slice(CONST_DATA);
        
        // nop to force linker to preserve the variable on stack
        unsafe { asm!("" : : "r"(&stacktag)) }
    }};
}

#[cfg(test)]
mod tests {
    // just testing syntax from within, please use test/run_test.sh which is 
    // starting a small application. The script will cause the appl. to core-dump with 
    // signal 6 and parse the core file for releasetag strings. If the tags can not be found
    // in the core file the feature did fail.
    #[test]
    fn valid_macro() {
        releasetag!(b"TAG1=123");
        releasetag!(b"TAG2=ABC");
    }
}