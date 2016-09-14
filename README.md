# releasetag

The crate 'releasetag' provides tooling for post-mortem analysis of rust-apps.

Releasetags are placed in context of main() function or on stack of any other thread. These tags will placed on stack. In case of an application crash the tag can be extracted from core dump file. 

## Use case
Imagine multiple releases/devdrops of your software have been shipped to your
customer. Now the customer is filing multiple crash-reports with attached core-file(s), but customer is providing unreliable information regarding the the corresponding release
in question.

Now, extracting the releasetag from each core-file the developers will be able to tell the correspondig
software revision and knows which debug symbols should be used from archive for further
investigation of backtraces.

Example Cargo.toml
```init
...
[dependencies]
bytestool = "0.2.0"
releasetag = "0.5.0"
```

Example: file main.rs
```rust
#![feature(plugin)]
#![plugin(bytestool)]
#[macro_use(releasetag)]
extern crate releasetag;
use releasetag::tag;

fn main() {
    // The argument must be a byte-string of the form b".." 
    releasetag!(b"BUILD_TAG=MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK");
    releasetag!(b"BUILD_HOST=host1");
 
    // your application logic here
}
```
In case the application did coredump to file 'core', the following comamnd can be used to extract the tags from core-file:
```
cat core | strings | grep BUILD_
```
The argument of releasetag!() must be a byte-string (array) of 8bit elements, with unlimited length. Regular UTF-Strings are not supported due to non-printable chars.  

The releasetag is a compile-time feature, without any processing during runtime. The overhead is 2 bytes. For example, if the releasetag! defines a byte string of 126 characters, the occupied stack-size would be 128 bytes, adding leading and trailing null-characters.

## Demonstrator
Execute the following script to verify the releasetag feature is working:
```
./test/run_test.sh
```
On success, the output should show:
```
BUILD_HOST=host1
BUILD_TAG=MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK
Success: releasetags found in file 'core'
```

###Explained:
The script test/run_test.sh is compiling the application 'test-tag' with release mode,
verifying the optimizer is not eliminating the 'unused' stacktag variables.
The application test-tag contains two releasetags BUILD_HOST=.. and BUILD_TAG=..

After a few seconds he script continues sending signal 6 (ABORT) to
the process to cause the application to core-dump with signal 6. The location of the
core file will be 'test/core'.

The resulting core-file is scanned for the releasetag strings 'BUILD_'.  

On success the script will return with return value 0, otherwise the feature is broken and return value will be 1.

The feature requires that the optimizer of compiler rustc is not eliminating the 'unused' stacktag variables. Former versions did use an inlined assembler NOP-expression for this purpose, but since release rustc-1.12 the stacktag mus be referenced using an function of dynamic library libreleasetag.so. 

The script ./test/run_test.sh is evaluating correct functionality of the feature.
