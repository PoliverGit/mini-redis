//! La commande.
//!
//! Cette partie traduit une ligne tapée au clavier en une demande claire, ou
//! refuse la ligne en expliquant pourquoi. Elle ne touche jamais au stockage :
//! elle comprend, elle n'agit pas.

use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Command
{
    /// Vérifie que le serveur répond. Renvoie le message si on lui en donne un.
    Ping(Option<String>),
    /// Range une valeur, avec ou sans limite de temps.
    Set
    {
        cle: String,
        valeur: String,
        duree: Option<Duration>,
    },
    /// Lit une valeur.
    Get
    {
        cle: String
    },
    /// Retire une ou plusieurs clés.
    Del
    {
        cles: Vec<String>
    },
    /// Compte combien de clés existent parmi celles demandées.
    Exists
    {
        cles: Vec<String>
    },
    /// Demande le délai restant sur une clé.
    Ttl
    {
        cle: String
    },
    /// Augmente de un la valeur d'une clé, qui doit être un nombre entier.
    Incr
    {
        cle: String
    },
    /// Compte les clés du stockage.
    Dbsize,
    /// Vide le stockage.
    Flushall,
    /// Termine la session.
    Quit,
}

/// Traduit une ligne tapée.
///
/// Une ligne vide ne vaut ni commande ni refus : il n'y a rien à faire.
pub fn parse(ligne: &str) -> Result<Option<Command>, String>
{
    let mots = decouper(ligne)?;
    let Some((nom, arguments)) = mots.split_first()
    else
    {
        return Ok(None);
    };

    let nom_normalise = nom.to_uppercase();
    let commande = match (nom_normalise.as_str(), arguments)
    {
        ("PING", []) => Command::Ping(None),
        ("PING", [message]) => Command::Ping(Some(message.clone())),

        ("SET", [cle, valeur]) => Command::Set {
            cle: cle.clone(),
            valeur: valeur.clone(),
            duree: None,
        },
        ("SET", [cle, valeur, unite, quantite]) => Command::Set {
            cle: cle.clone(),
            valeur: valeur.clone(),
            duree: Some(lire_duree(unite, quantite)?),
        },

        ("GET", [cle]) => Command::Get { cle: cle.clone() },

        ("DEL", [_, ..]) => Command::Del {
            cles: arguments.to_vec(),
        },

        ("EXISTS", [_, ..]) => Command::Exists {
            cles: arguments.to_vec(),
        },

        ("TTL", [cle]) => Command::Ttl { cle: cle.clone() },

        ("INCR", [cle]) => Command::Incr { cle: cle.clone() },

        ("DBSIZE", []) => Command::Dbsize,
        ("FLUSHALL", []) => Command::Flushall,
        ("QUIT", []) | ("EXIT", []) => Command::Quit,

        (
            "PING" | "SET" | "GET" | "DEL" | "EXISTS" | "TTL" | "INCR" | "DBSIZE"
            | "FLUSHALL" | "QUIT" | "EXIT",
            _,
        ) =>
        {
            return Err(format!(
                "ERR nombre d'arguments incorrect pour la commande « {} »",
                nom.to_lowercase()
            ));
        }

        _ =>
        {
            return Err(format!("ERR commande inconnue « {nom} »"));
        }
    };

    Ok(Some(commande))
}

/// Lit une limite de temps donnée sous la forme `EX 10` ou `PX 500`.
fn lire_duree(unite: &str, quantite: &str) -> Result<Duration, String>
{
    let nombre: i64 = quantite
        .parse()
        .map_err(|_| "ERR la valeur n'est pas un nombre entier".to_string())?;

    // Un délai nul ou négatif déposerait une valeur déjà périmée : c'est refusé.
    let nombre = u64::try_from(nombre)
        .ok()
        .filter(|delai| *delai > 0)
        .ok_or_else(|| "ERR délai invalide pour la commande « set »".to_string())?;

    match unite.to_uppercase().as_str()
    {
        "EX" => Ok(Duration::from_secs(nombre)),
        "PX" => Ok(Duration::from_millis(nombre)),
        _ => Err("ERR erreur de syntaxe".to_string()),
    }
}

/// Découpe une ligne en mots. Les espaces séparent, sauf entre guillemets
/// doubles, ce qui permet de ranger une valeur qui contient des espaces.
fn decouper(ligne: &str) -> Result<Vec<String>, String>
{
    let mut mots = Vec::new();
    let mut mot = String::new();
    let mut mot_commence = false;
    let mut entre_guillemets = false;

    for caractere in ligne.chars()
    {
        match caractere
        {
            '"' =>
            {
                entre_guillemets = !entre_guillemets;
                mot_commence = true;
            }
            c if c.is_whitespace() && !entre_guillemets =>
            {
                if mot_commence
                {
                    mots.push(std::mem::take(&mut mot));
                    mot_commence = false;
                }
            }
            c =>
            {
                mot.push(c);
                mot_commence = true;
            }
        }
    }

    if entre_guillemets
    {
        return Err("ERR guillemet ouvert et jamais refermé".to_string());
    }

    if mot_commence
    {
        mots.push(mot);
    }

    Ok(mots)
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn commande(ligne: &str) -> Command
    {
        parse(ligne)
            .expect("la ligne devait être acceptée")
            .expect("la ligne devait contenir une commande")
    }

    #[test]
    fn ligne_vide_ne_donne_aucune_commande()
    {
        assert_eq!(parse(""), Ok(None));
        assert_eq!(parse("    "), Ok(None));
    }

    #[test]
    fn ping_seul()
    {
        assert_eq!(commande("PING"), Command::Ping(None));
    }

    #[test]
    fn ping_avec_message()
    {
        assert_eq!(
            commande("PING bonjour"),
            Command::Ping(Some("bonjour".to_string()))
        );
    }

    #[test]
    fn set_simple()
    {
        assert_eq!(
            commande("SET pseudo paul"),
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: None,
            }
        );
    }

    #[test]
    fn set_avec_delai_en_secondes()
    {
        assert_eq!(
            commande("SET pseudo paul EX 10"),
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: Some(Duration::from_secs(10)),
            }
        );
    }

    #[test]
    fn set_avec_delai_en_millisecondes()
    {
        assert_eq!(
            commande("SET pseudo paul PX 500"),
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: Some(Duration::from_millis(500)),
            }
        );
    }

    #[test]
    fn set_avec_valeur_entre_guillemets()
    {
        assert_eq!(
            commande("SET titre \"guerre et paix\""),
            Command::Set {
                cle: "titre".to_string(),
                valeur: "guerre et paix".to_string(),
                duree: None,
            }
        );
    }

    #[test]
    fn set_avec_valeur_vide_entre_guillemets()
    {
        assert_eq!(
            commande("SET vide \"\""),
            Command::Set {
                cle: "vide".to_string(),
                valeur: String::new(),
                duree: None,
            }
        );
    }

    #[test]
    fn get_simple()
    {
        assert_eq!(
            commande("GET pseudo"),
            Command::Get {
                cle: "pseudo".to_string()
            }
        );
    }

    #[test]
    fn del_accepte_plusieurs_cles()
    {
        assert_eq!(
            commande("DEL a b c"),
            Command::Del {
                cles: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            }
        );
    }

    #[test]
    fn exists_accepte_plusieurs_cles()
    {
        assert_eq!(
            commande("EXISTS a b"),
            Command::Exists {
                cles: vec!["a".to_string(), "b".to_string()],
            }
        );
    }

    #[test]
    fn ttl_et_incr_prennent_une_cle()
    {
        assert_eq!(
            commande("TTL pseudo"),
            Command::Ttl {
                cle: "pseudo".to_string()
            }
        );
        assert_eq!(
            commande("INCR compteur"),
            Command::Incr {
                cle: "compteur".to_string()
            }
        );
    }

    #[test]
    fn commandes_sans_argument()
    {
        assert_eq!(commande("DBSIZE"), Command::Dbsize);
        assert_eq!(commande("FLUSHALL"), Command::Flushall);
        assert_eq!(commande("QUIT"), Command::Quit);
        assert_eq!(commande("EXIT"), Command::Quit);
    }

    #[test]
    fn le_nom_de_commande_ignore_la_casse()
    {
        assert_eq!(
            commande("get pseudo"),
            Command::Get {
                cle: "pseudo".to_string()
            }
        );
        assert_eq!(
            commande("SeT pseudo paul"),
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: None,
            }
        );
    }

    #[test]
    fn la_casse_des_cles_et_des_valeurs_est_conservee()
    {
        assert_eq!(
            commande("SET Pseudo Paul"),
            Command::Set {
                cle: "Pseudo".to_string(),
                valeur: "Paul".to_string(),
                duree: None,
            }
        );
    }

    #[test]
    fn les_espaces_superflus_sont_ignores()
    {
        assert_eq!(
            commande("   SET   pseudo   paul   "),
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: None,
            }
        );
    }

    #[test]
    fn commande_inconnue_est_refusee()
    {
        assert_eq!(
            parse("BONJOUR"),
            Err("ERR commande inconnue « BONJOUR »".to_string())
        );
    }

    #[test]
    fn nombre_d_arguments_incorrect_est_refuse()
    {
        assert_eq!(
            parse("GET"),
            Err("ERR nombre d'arguments incorrect pour la commande « get »".to_string())
        );
        assert_eq!(
            parse("GET a b"),
            Err("ERR nombre d'arguments incorrect pour la commande « get »".to_string())
        );
        assert_eq!(
            parse("SET pseudo"),
            Err("ERR nombre d'arguments incorrect pour la commande « set »".to_string())
        );
        assert!(parse("DEL").is_err());
        assert!(parse("PING a b").is_err());
        assert!(parse("DBSIZE maintenant").is_err());
    }

    #[test]
    fn set_avec_unite_de_temps_inconnue_est_refuse()
    {
        assert_eq!(parse("SET pseudo paul XX 10"), Err("ERR erreur de syntaxe".to_string()));
    }

    #[test]
    fn set_avec_delai_non_entier_est_refuse()
    {
        assert_eq!(
            parse("SET pseudo paul EX dix"),
            Err("ERR la valeur n'est pas un nombre entier".to_string())
        );
    }

    #[test]
    fn set_avec_delai_nul_est_refuse()
    {
        assert_eq!(
            parse("SET pseudo paul EX 0"),
            Err("ERR délai invalide pour la commande « set »".to_string())
        );
    }

    #[test]
    fn set_avec_delai_negatif_est_refuse()
    {
        assert_eq!(
            parse("SET pseudo paul EX -5"),
            Err("ERR délai invalide pour la commande « set »".to_string())
        );
    }

    #[test]
    fn guillemet_non_referme_est_refuse()
    {
        assert_eq!(
            parse("SET titre \"guerre et paix"),
            Err("ERR guillemet ouvert et jamais refermé".to_string())
        );
    }
}
