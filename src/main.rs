extern crate pancurses;

use pancurses::{initscr, endwin, Input};
use std::io::{self, Write};
fn main() {
  let window = initscr();

  window.draw_box('*', '*');
  window.refresh();
  window.mvaddstr(0, 0, "Welcome to Git Curses, a central place for info about Git repositories!");
  window.refresh();

  let y =  window.derwin( 3, 50, 10,10 ).unwrap();
    
  

  y.draw_box('*', '*');
  y.addstr("Enter your git repository link here");
  y.mv(1, 1);

  y.refresh();
  let mut hello: String = String::new();

  while(true) {
    match y.getch() {
      Some(Input::Character(c)) => { hello.push(c); },
      Some(Input::KeyDC) => break,
      Some(input) => { window.addstr(&format!("{:?}", input)); },
      None => ()
  }
   y.refresh();
   window.refresh();

  }

  endwin();
}