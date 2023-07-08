
pub mod io;
pub use io::*;

use crate::*;

pub enum CmdKind {
  Unknown,
  Help,
  Exit,
}

pub enum CmdFlag {
  Single(char), // single hyphen, ex) -h
  Full(DevStr), // double hyphen, ex) --help
  Normal(DevStr), // no hyphen, ex) help
}

pub struct CmdArg {
  flag: CmdFlag,
  value: DevStr,
}

pub struct CmdLine {
  cmd: CmdKind,
  args: Vec<CmdArg>, // ex) -abc "value" >> -a, -b, -c "value"
}

pub struct Cli {
  logs: Vec<CmdLine>
}

impl Cli {
  pub fn new() -> Self {
    Cli {
      logs: Vec::new()
    }
  }
  pub fn run(&mut self) {
    loop {
      let command: CmdLine = Self::get_command();

      match command.cmd {
        CmdKind::Exit => break,
        _ => println!("Command Unknown"),
      }
    }
  }
  fn get_command() -> CmdLine {
    let string = msg_line!("> ");

    if string == "exit" || string == "quit"
    || string == "e" || string == "q" {
      return CmdLine {
        cmd: CmdKind::Exit,
        args: Vec::new(),
      }
    }

    CmdLine {
      cmd: CmdKind::Unknown,
      args: Vec::new(),
    }
  }
}
