extern crate rustyline;

use ore::repl::rep;
use rustyline::{ DefaultEditor, Result };
use rustyline::error::ReadlineError;

fn main() -> Result<()> {
  let mut rl = DefaultEditor::new()?;

  if rl.load_history("history.txt").is_err() {
    eprintln!("ore< history not found");
  }

  loop {
    let readline = rl.readline("ore> ");
    match readline {
      Ok(line) => {
        let _ = rl.add_history_entry(&line);
        rl.save_history("history.txt").unwrap();
        if line.len() > 0 {
          rep(line);
        }
      },
      Err(ReadlineError::Interrupted) => {
        println!("ore< Interrupted");
        continue
      },
      Err(ReadlineError::Eof) => {
        println!("ore< End Of File");
        break
      },
      Err(err) => {
        println!("ore< Error: {:?}", err);
        break;
      },
    }
  }

  Ok(())
}
