//! La séance.
//!
//! C'est le fil du dialogue : afficher l'invite, lire une ligne, la faire
//! comprendre, la faire exécuter, écrire la réponse, recommencer.
//! Elle s'arrête quand on le lui demande, ou quand il n'y a plus rien à lire.

use crate::command::{Command, parse};
use crate::execute::execute;
use crate::reply::Reponse;
use crate::store::Store;
use std::io::{self, BufRead, Write};

/// Ce qui est affiché avant chaque ligne attendue.
pub const INVITE: &str = "> ";

/// Tient la séance jusqu'à son terme.
pub fn dialogue(
    entree: &mut impl BufRead,
    sortie: &mut impl Write,
    store: &mut Store,
) -> io::Result<()>
{
    loop
    {
        write!(sortie, "{INVITE}")?;
        sortie.flush()?;

        let mut ligne = String::new();
        if entree.read_line(&mut ligne)? == 0
        {
            // Plus rien à lire : on termine la ligne en cours et on s'arrête.
            writeln!(sortie)?;
            sortie.flush()?;
            return Ok(());
        }

        match parse(&ligne)
        {
            // Une ligne vide n'appelle aucune réponse.
            Ok(None) =>
            {}
            Ok(Some(Command::Quit)) =>
            {
                writeln!(sortie, "{}", execute(Command::Quit, store))?;
                sortie.flush()?;
                return Ok(());
            }
            Ok(Some(commande)) =>
            {
                writeln!(sortie, "{}", execute(commande, store))?;
            }
            Err(raison) =>
            {
                writeln!(sortie, "{}", Reponse::Erreur(raison))?;
            }
        }

        sortie.flush()?;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::io::Cursor;

    /// Joue une suite de lignes tapées et rend tout ce qui s'est affiché.
    fn jouer(saisie: &str) -> String
    {
        let mut entree = Cursor::new(saisie.as_bytes().to_vec());
        let mut sortie = Vec::new();
        let mut store = Store::new();
        dialogue(&mut entree, &mut sortie, &mut store).expect("le dialogue devait aboutir");
        String::from_utf8(sortie).expect("la sortie devait être lisible")
    }

    #[test]
    fn saisie_vide_affiche_une_invite_puis_s_arrete()
    {
        assert_eq!(jouer(""), "> \n");
    }

    #[test]
    fn une_commande_donne_une_reponse_puis_une_nouvelle_invite()
    {
        assert_eq!(jouer("PING\n"), "> \"PONG\"\n> \n");
    }

    #[test]
    fn ranger_puis_relire_dans_la_meme_seance()
    {
        assert_eq!(
            jouer("SET pseudo paul\nGET pseudo\n"),
            "> OK\n> \"paul\"\n> \n"
        );
    }

    #[test]
    fn le_stockage_est_le_meme_du_debut_a_la_fin()
    {
        assert_eq!(
            jouer("SET a 1\nSET b 2\nDBSIZE\nDEL a\nDBSIZE\n"),
            "> OK\n> OK\n> (integer) 2\n> (integer) 1\n> (integer) 1\n> \n"
        );
    }

    #[test]
    fn une_ligne_vide_ne_donne_aucune_reponse()
    {
        assert_eq!(jouer("\n   \nPING\n"), "> > > \"PONG\"\n> \n");
    }

    #[test]
    fn une_ligne_refusee_affiche_la_raison_et_la_seance_continue()
    {
        assert_eq!(
            jouer("BONJOUR\nPING\n"),
            "> (error) ERR commande inconnue « BONJOUR »\n> \"PONG\"\n> \n"
        );
    }

    #[test]
    fn quit_termine_la_seance_et_ignore_la_suite()
    {
        assert_eq!(jouer("QUIT\nPING\n"), "> OK\n");
    }

    #[test]
    fn exit_termine_la_seance_aussi()
    {
        assert_eq!(jouer("EXIT\n"), "> OK\n");
    }

    #[test]
    fn une_derniere_ligne_sans_retour_chariot_est_traitee()
    {
        assert_eq!(jouer("PING"), "> \"PONG\"\n> \n");
    }
}
