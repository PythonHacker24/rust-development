use std::thread;
use std::time::Duration;

use console::Term;
use console::style;

fn main() {
    let term = Term::stdout();
    term.write_line("Hello World! => {}", style("quite").cyan());
    thread::sleep(Duration::from_millis(2000));
    term.clear_line();    
}

