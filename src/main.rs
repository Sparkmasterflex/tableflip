extern crate clipboard;

use std::env;
use clipboard::{ClipboardProvider, ClipboardContext};

mod upside_down;

use crate::upside_down::upside_down;

fn main() {
  let args = env::args().skip(1);
  let mut to_flip = String::new();
  let mut i = 0;

  if args.len() != 0 {
    for s in env::args().skip(1) {
      to_flip += &s;
      if i < args.len() - 1 { to_flip += " " }
      i += 1;
    }
    let flipped: String = format!(
      "(╯°□°）╯︵ {flip}",
      flip = upside_down(&to_flip)
    );
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(flipped.to_owned()).unwrap();
    println!("\"{}\" copied to clipboard!", &flipped);
  } else {
      println!("(╯°□°）╯︵ ┻━┻ I flip tables if you don't tell me otherwise");
  }
}
