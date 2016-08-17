#![feature(asm)] 


#[macro_export]
macro_rules! releasetag {
    ($tag:expr) => {{
        const LEN : usize = 64; //($tag).len();    
        let mut stackmem : &mut[u8; LEN] = &mut[0; LEN];
        stackmem[0] = '\0' as u8; // delimit from preceding string
        
        // copy tag into stack mem
        for (i,c) in $tag.chars().enumerate() {
            stackmem[i+1] = c as u8;
        }
        // nop to force linker to preserve the variable on stack
        unsafe { asm!("" : : "r"(&stackmem)) }
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn valid_macro() {
        releasetag!("TAG1=123");
        releasetag!("TAG2=ABC");
    }
}
