//! Le point d'entrée.
//!
//! Il monte les parties et lance la séance sur le clavier et l'écran.

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
    let mut entree = BufReader::new(io::stdin().lock());
    let mut sortie = io::stdout().lock();
    let mut store = Store::new();

    match session::dialogue(&mut entree, &mut sortie, &mut store)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(erreur) =>
        {
            eprintln!("la séance s'est interrompue : {erreur}");
            ExitCode::FAILURE
        }
    }
}
