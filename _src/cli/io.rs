
use std::io::Write;

// print! macro
//  * does not change line
//  * no arg not allowed
//  * does not flush immediately (fix: printfl)

// println! macro
//  * change line
//  * no arg allowed (flushln)
//  * flush immdediately

// flush immdediately
#[macro_export]
macro_rules! printfl {
  () => {
    flush();
  };
  ($($arg:tt)*) => {{
    print!($($arg)*);
    flush();
  }};
}

// simplified version
//  * does not need to write 'use std::io::Write;'
//  * no need to write unwrap (already written, potential risk)
//  * less verbose
pub fn flush() {
  std::io::stdout().flush().unwrap();
}

pub fn flushln() {
  println!();
}

// simplified version
//  * does not need to make string (instead, this returns string)
//  * no worry about appending (read_line appends input to string, when string is reused)
//    this just doesn't reuse string. this might be less efficient, btw.
//  * does not need to trim (without trimming, line break will remain at the end)
//  * less verbose
pub fn get_line() -> String {
  let mut string = String::new();
  std::io::stdin().read_line(&mut string).unwrap();
  string.trim().to_string()
}

// print message and get line
#[macro_export]
macro_rules! msg_line {
  () => {
    get_line()
  };
  ($($arg:tt)*) => {{
    printfl!($($arg)*);
    get_line()
  }};
}
