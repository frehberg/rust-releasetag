# releasetag

The crate 'releasetag' provides tooling for for post-mortem analysis for Rust .

Releasetags are placed in context of main() function or on stack of any other thread. These tags will placed on stack and in case the application is crashing, these tags are embedded into the core dump file.

Postmortem these tags can be extracted from core-dump file, for example release information or application config.

Example: file main.rs
```rust
#![feature(asm)] 
#![feature(plugin)]
#![plugin(bytestool)]
#[macro_use(releasetag)]
extern crate releasetag;

fn main() {
    // The argument must be a byte-string of the form b".." 
    releasetag!(b"BUILD_TAG=MAIN_2016-wk16-05");
    releasetag!(b"BUILD_HOST=host1");
 
    // your application logic here
}
```
In case the application did coredump to file 'core', the following comamnd can be used to extract the tags from core-file:
```
cat core | strings | grep BUILD_
```
The argument of releasetag!() must be a byte-string (array) of 8bit elements, 
instead of unicode string. The release-tag may be a byte-string of any length. In case the 
the release-tag has byte-length 126, the occupied stack-size would be 128 bytes, 
adding leading and trailing null-characters.

The feature is restricted to byte-strings as unicode strings might contain 
non-printable characters causing the command line tool 'strings' to print fragments of
tag only (causing loss of information)