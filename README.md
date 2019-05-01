# releasetag

The crate 'releasetag' provides tooling to identify from crash-files the release-tag post-mortem.

Releasetags are placed in context of main() function or on stack of any other thread. In case the application crashes, such tags can be extracted from core-dump file. 

Releasetags can be formed from one or multiple byte-strings, byte-arrays or includes from files. Those must not contain any whitespace as new-lines or line-breaks.

## Use case
Imagine multiple releases/devdrops of your software have been shipped to your
customer. Now the customer is filing multiple crash-reports with attached core-dumps, just the customer is providing unreliable information regarding the corresponding release in question and you would have to guess the software-release.

Now, extracting the releasetag from each core-file it is possible to tell the correspondig
software-release and choosing the correct debug symbols from archive for further
investigation of backtraces.

Example Cargo.toml
```init
...
[dependencies]
releasetag = "^1.1"
```

Example: file test/src/main.rs
```rust
#[macro_use(releasetag)]
extern crate releasetag;

fn main() {
    releasetag!(b"BUILD_TAG=pre");
    releasetag!(b"BUILD_TAG=",     b"MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK");

    // Including data from one or multiple files or compile time env params.
    // Note: the content mustn't contain any newline or linebreak as those would interfer with zero terminated strings!!
    let mut version: [u8; 5] = [0; 5];
    version.copy_from_slice(env!("CARGO_PKG_VERSION").as_bytes());
    releasetag!(b"BUILD_VERSION=", &version);
    releasetag!(b"BUILD_TAG=",     &version, b"/", include_bytes!("../../AUTHOR"));
    
    // or as byte array
    releasetag!(b"BUILD_HOST=",    &[0x30u8, 0x31u8, 0x33u8]);
 
    // your application logic here
}
```
In case the application does crash its state is dumped to a core-file 'core', and for example the following system command 'strings' can be used to extract the tags from core-file:
```
cat core | strings | grep BUILD_
```
Releasetags can be formed from one or multiple byte-strings, byte-arrays or includes from files. Those must not contain any whitespace as new-lines or line-breaks. Regular Rust-strings are not supported as they may contain zero-characters.

The releasetag is a compile-time feature, without any processing during runtime. 

The overhead of each releasetag depends on the underlying architecture and default integer-size. For 32bit architecture the overhead will be 5 bytes, for 64bit architectures the overhead will be 9bytes, caused by leading and trailing zero paddings. 

For example on 32bit arch, a releasetag! of 50 characters, will the occupy 55 bytes on memory stack.

## Demonstrator
Execute the following script to verify the releasetag feature is working:
```
./test/run_test.sh
```
On success, the output should show:
```
aiting until being aborted
...../run_test.sh: line 14: 25494 Aborted                 (core dumped) ./target/release/test-tag
BUILD_HOST=013
BUILD_VERSION=1.1.0
BUILD_TAG=pre
BUILD_TAG=1.1.0/frehberg@gmail.com
BUILD_TAG=MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK
Success: releasetags found in file 'core'
```

### Demonstrator Explained:
The script test/run_test.sh is compiling the application 'test-tag' with 'release' flag. Intention of this test is to verify that the releasetag elements do survive the 'release' optimizer-pass during compilation. The script ./test/run_test.sh is evaluating correct functionality of the releasetag-feature.

The application test-tag contains a number of releasetags BUILD_HOST=.. and BUILD_TAG=..

After a few seconds he script continues sending signal 6 (ABORT) to
the process to cause the application to core-dump with signal 6. The location of the
core file will be 'test/core'.

The resulting core-file is scanned for the releasetag strings 'BUILD_'.

On success the script will exit with return value 0, otherwise the feature is broken and return value will be 1.

