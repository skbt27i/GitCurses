extern crate pancurses;
use std::process::Command;

use git2::Repository;
use pancurses::{initscr, Input};

fn main() {
  let window = initscr();
  window.refresh();
  window.keypad(true);

  window.draw_box('*', '*');
  window.refresh();
  window.mvaddstr(0, 0, "Welcome to Git Curses, a central place for info about Git repositories!");
  window.refresh();

  let mut y =  window.derwin( 3, 50, 10,10 ).unwrap();
  y.draw_box('*', '*');
  y.keypad(true);


  y.addstr("Enter git link, press delete key to submit.");
  y.mv(1, 1);

  y.refresh();
  let mut url: String = String::new();
  loop {
    match y.getch() {
      Some(Input::KeyDC) => {  break; },
      Some(Input::Character(c)) => { 
            if c as u32 == 8 {
              // handle backspace
              url.pop();
              y.clear();
              y.draw_box('*', '*');
              y.mvaddstr(0, 0, "Enter git link, press delete key to submit.");
              y.mv(1, 1);
              y.addstr(&url);
          } else {
              // handle other characters
              url.push(c); 
          }      
      },
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
  y.erase();
  y.refresh();

  loop {
    window.refresh();

    let mut search_string: String = String::new();

    y =  window.derwin( 3, 50, 10,10 ).unwrap();

    y.draw_box('*', '*');
    y.addstr("Enter search string, press delete key to submit.");
    y.mv(1, 1);
    loop {
        match y.getch() {
          Some(Input::Character(c)) => { 
              if c as u32 == 8 {
                // handle backspace
                search_string.pop();
                y.clear();
                y.draw_box('*', '*');
                y.addstr("Enter search string, press delete key to submit.");
                y.mv(1, 1);
                y.addstr(&search_string);
            } else {
                // handle other characters
                search_string.push(c); 
            }
          
          },

          Some(Input::KeyDC) => {
            y.clear();
            break;
          },
          Some(input) => { y.addstr(&format!("{:?}", input)); },
          None => ()
        }
        y.refresh();
      }

      let mut owned_string: String = "findstr /s /i /n ".to_owned();
      owned_string.push_str(&search_string);
      owned_string.push_str(" newRepo\\*.*");


      let output = Command::new("cmd")
      .args(&["/C", &owned_string])
      .output()
      .expect("Failed to execute");
      if !output.status.success() {
        println!("Error: String not found");
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}",stdout)

  } 
}