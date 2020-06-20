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
    to_the_pastebuffer(&flipped);
  } else {
    let tf: String = "(╯°□°）╯︵ ┻━┻".to_string();
    println!("{} I flip tables if you don't tell me otherwise", &tf);
    to_the_pastebuffer(&tf);
  }
}

fn to_the_pastebuffer(flipped: &str) {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(flipped.to_owned()).unwrap();
  println!("\n\n\"{}\" copied to clipboard!", &flipped);
}