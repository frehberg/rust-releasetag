#[macro_use(releasetag)]
extern crate releasetag;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use std::io::Write;

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
 
    println!("Waiting until being aborted");
    loop {
      thread::sleep(Duration::from_millis(200));
      print!(".");
      stdout().flush().ok();
    }
}
