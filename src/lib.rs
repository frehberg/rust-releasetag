#![feature(asm)] 


#[macro_export]
macro_rules! releasetag {
    ($tag:expr) => {{
        // CAPACITY incl leading and trailing \0 (+2)
        const CAPACITY : usize = 64;
        let mut stackmem : &mut[u8;  CAPACITY] = &mut[0; CAPACITY];

        stackmem[0] = '\0' as u8; // delimit from preceding string
        
        // copy tag into stack mem
        for (i,c) in $tag.iter().enumerate() {
            stackmem[i+1] = *c;
        }
        // nop to force linker to preserve the variable on stack
        unsafe { asm!("" : : "r"(&stackmem)) }
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn valid_macro() {
        releasetag!(b"TAG1=123");
        releasetag!(b"TAG2=ABC");
    }
}
