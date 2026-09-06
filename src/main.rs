mod command;
mod execute;
mod store;

use command::parse;
use execute::execute;
use std::io::{self, BufRead, Write};
use store::Store;

fn main()
{
    let mut store = Store::new();
    let entree = io::stdin();
    loop
    {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut ligne = String::new();
        match entree.lock().read_line(&mut ligne)
        {
            Ok(0) => break,          // fin d'entrée (Ctrl-D)
            Ok(_) => {}
            Err(e) =>
            {
                eprintln!("erreur de lecture : {e}");
                break;
            }
        }
        match parse(&ligne)
        {
            Ok(commande) => println!("{}", execute(commande, &mut store)),
            Err(message) => println!("ERR {message}"),
        }
    }
}
