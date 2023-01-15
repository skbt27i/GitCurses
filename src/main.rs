extern crate pancurses;
use std::process::Command;

use git2::Repository;
use pancurses::{initscr, endwin, Input};

fn main() {
  let window = initscr();
  window.refresh();
  window.keypad(true);

  window.draw_box('*', '*');
  window.refresh();
  window.mvaddstr(0, 0, "Welcome to Git Curses, a central place for info about Git repositories!");
  window.refresh();

  let y =  window.derwin( 3, 50, 10,10 ).unwrap();
  y.draw_box('*', '*');
  y.keypad(true);


  y.addstr("Enter git link, press delete key to submit.");
  y.mv(1, 1);

  y.refresh();
  let mut url: String = String::new();
  while true {
    match y.getch() {
      Some(Input::KeyDC) => {  break; },
      Some(Input::Character(c)) => { url.push(c); },
      Some(input) => { window.addstr(&format!("{:?}", input)); },
      None => ()
   }

    y.refresh();
    window.refresh();
  }
  let _repo = match Repository::clone(&url, "newRepo") {
    Ok(_repo) => _repo,
    Err(e) => panic!("failed to clone: {}", e),
  };

  while true  {
    y.erase();
    y.refresh();
    window.refresh();

    let mut search_string: String = String::new();

    let x =  window.derwin( 3, 50, 10,10 ).unwrap();

    x.draw_box('*', '*');
    x.addstr("Enter search string, press delete key to submit.");
    x.mv(1, 1);
    while(true) {
        match x.getch() {
          Some(Input::Character(c)) => { search_string.push(c); },
          Some(Input::KeyDC) => break,
          Some(input) => { window.addstr(&format!("{:?}", input)); },
          None => ()
      }
      x.refresh();
      window.refresh();

      }

      let mut owned_string: String = "findstr /s /i /n ".to_owned();
      owned_string.push_str(&search_string);
      owned_string.push_str(" *.*");


      let _output = Command::new("cmd")
      .args(&["/C", &owned_string])
      .spawn()
      .expect("Failed to execute");
  }
  

  endwin();
}
