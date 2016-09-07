# releasetag

The crate 'releasetag' provides tooling for for post-mortem analysis of rust-apps.

Releasetags are placed in context of main() function or on stack of any other thread. These tags will placed on stack and in case the application is crashing, these tags are embedded into the core dump file.

Postmortem these tags can be extracted from core-dump file, for example release information or application config.

Use case: Imagine multiple releases/devdrops of your software have been shipped to your
customer. Now the customer is filing a crash-report with attached core-file(s). Extracting
the releasetag from each core-file the developers will be able to tell the correspondig
software revision and which debug symbols should be used from archive for further
investigation of backtraces.

Example Cargo.toml
```init
...
[dependencies]
bytestool = "0.2.0"
releasetag = "0.4.0"
```

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

## Demonstrator
Execute the following script to verify the releasetag feature is working:
```
./test/run_test.sh
```
On success, the output should show:
```
BUILD_HOST=host1
BUILD_TAG=MAIN_2016-wk16-05
Success: releasetags found in file 'core'
```

###Explained:
The script test/run_test.sh is compiling the application test-tag using release mode
to verify the optimizer is not eliminating the stacktag variables.
The application test-tag contains two releasetags BUILD_HOST=.. and BUILD_TAG=..

The script will start the application and after a few seconds sending signal 6 (ABORT) to
the process to cause the application to core-dump with signal 6. The location of the
core file will be 'test/core'.

The resulting core-file is searched for the releasetag strings 'BUILD_'.  On success the script will return with return value 0, otherwise the feature is broken and return value will be 1.

The feature requires that the optimizer of compiler rustc is not eliminating the 'unused' stacktag variables. The inline assembler expression should prevent the elimination. If upgrading to new release of rustc, one should execute the script to verify the feature.
