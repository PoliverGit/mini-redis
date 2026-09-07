//! La réponse.
//!
//! Cette partie dit ce que le serveur répond, et sous quelle forme cela
//! s'affiche à l'écran. Elle ne décide de rien : elle met en forme.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Reponse
{
    /// La demande a été faite.
    Ok,
    /// Un texte, rendu entre guillemets pour qu'on voie où il commence et
    /// où il finit.
    Texte(String),
    /// Il n'y avait rien à rendre.
    Rien,
    /// Un nombre : un compte, une durée, un total.
    Entier(i64),
    /// La demande a été refusée, avec sa raison.
    Erreur(String),
}

impl fmt::Display for Reponse
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Reponse::Ok => write!(f, "OK"),
            Reponse::Texte(texte) => write!(f, "\"{}\"", echapper(texte)),
            Reponse::Rien => write!(f, "(nil)"),
            Reponse::Entier(nombre) => write!(f, "(integer) {nombre}"),
            Reponse::Erreur(raison) => write!(f, "(error) {raison}"),
        }
    }
}

/// Protège les guillemets et les barres obliques inverses contenus dans un
/// texte, pour qu'on voie toujours où le texte affiché commence et finit.
fn echapper(texte: &str) -> String
{
    let mut resultat = String::with_capacity(texte.len());
    for caractere in texte.chars()
    {
        match caractere
        {
            '"' => resultat.push_str("\\\""),
            '\\' => resultat.push_str("\\\\"),
            c => resultat.push(c),
        }
    }
    resultat
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn ok_s_affiche_sans_decoration()
    {
        assert_eq!(Reponse::Ok.to_string(), "OK");
    }

    #[test]
    fn un_texte_s_affiche_entre_guillemets()
    {
        assert_eq!(Reponse::Texte("paul".to_string()).to_string(), "\"paul\"");
    }

    #[test]
    fn un_texte_vide_reste_visible()
    {
        assert_eq!(Reponse::Texte(String::new()).to_string(), "\"\"");
    }

    #[test]
    fn un_texte_avec_espaces_garde_ses_bords()
    {
        assert_eq!(
            Reponse::Texte("guerre et paix".to_string()).to_string(),
            "\"guerre et paix\""
        );
    }

    #[test]
    fn les_guillemets_du_texte_sont_proteges()
    {
        assert_eq!(
            Reponse::Texte("il a dit \"oui\"".to_string()).to_string(),
            "\"il a dit \\\"oui\\\"\""
        );
    }

    #[test]
    fn l_absence_de_valeur_s_affiche_nil()
    {
        assert_eq!(Reponse::Rien.to_string(), "(nil)");
    }

    #[test]
    fn un_nombre_est_annonce_comme_tel()
    {
        assert_eq!(Reponse::Entier(1).to_string(), "(integer) 1");
        assert_eq!(Reponse::Entier(0).to_string(), "(integer) 0");
        assert_eq!(Reponse::Entier(-2).to_string(), "(integer) -2");
    }

    #[test]
    fn une_erreur_est_annoncee_comme_telle()
    {
        assert_eq!(
            Reponse::Erreur("ERR commande inconnue « BONJOUR »".to_string()).to_string(),
            "(error) ERR commande inconnue « BONJOUR »"
        );
    }
}
