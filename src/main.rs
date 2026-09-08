//! Le point d'entrée.
//!
//! Il monte les parties et lance la session sur le clavier et l'écran.

mod command;
mod execute;
mod reply;
mod session;
mod store;

use std::io::{self, BufReader};
use std::process::ExitCode;
use store::Store;

fn main() -> ExitCode
{
    // Le clavier donne des octets, BufReader les découpe en lignes
    let mut entree = BufReader::new(io::stdin().lock());
    let mut sortie = io::stdout().lock();
    let mut store = Store::new();
    match session::dialogue(&mut entree, &mut sortie, &mut store)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(erreur) =>
        {
            eprintln!("la session s'est interrompue : {erreur}");
            ExitCode::FAILURE
        }
    }
}
