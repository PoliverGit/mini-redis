#[derive(Debug, PartialEq)]
pub enum Command
{
    Set
    {
        key: String, value: String
    },
    Get
    {
        key: String
    },
    Del
    {
        key: String
    },
}

/// Traduit une ligne de texte en commande.
pub fn parse(texte: &str) -> Result<Command, String>
{
    let mots: Vec<&str> = texte.split_whitespace().collect();
    let (nom, args) = mots.split_first().ok_or("ligne vide".to_string())?;

    match (nom.to_uppercase().as_str(), args)
    {
        ("SET", [key, value]) => Ok(Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }),
        ("GET", [key]) => Ok(Command::Get {
            key: key.to_string(),
        }),
        ("DEL", [key]) => Ok(Command::Del {
            key: key.to_string(),
        }),
        ("SET", _) => Err(format!("SET attend 2 arguments, {} reçus", args.len())),
        ("GET", _) => Err(format!("GET attend 1 argument, {} reçus", args.len())),
        ("DEL", _) => Err(format!("DEL attend 1 argument, {} reçus", args.len())),
        (autre, _) => Err(format!("commande inconnue : {autre}")),
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn set_simple()
    {
        assert_eq!(
            parse("SET pseudo paul"),
            Ok(Command::Set {
                key: "pseudo".to_string(),
                value: "paul".to_string()
            })
        );
    }

    #[test]
    fn get_simple()
    {
        assert_eq!(
            parse("GET pseudo"),
            Ok(Command::Get {
                key: "pseudo".to_string()
            })
        );
    }

    #[test]
    fn del_simple()
    {
        assert_eq!(
            parse("DEL pseudo"),
            Ok(Command::Del {
                key: "pseudo".to_string()
            })
        );
    }

    #[test]
    fn casse_ignoree()
    {
        assert_eq!(
            parse("get pseudo"),
            Ok(Command::Get {
                key: "pseudo".to_string()
            })
        );
    }

    #[test]
    fn espaces_superflus_ignores()
    {
        assert_eq!(
            parse("   SET   pseudo   paul  "),
            Ok(Command::Set {
                key: "pseudo".to_string(),
                value: "paul".to_string()
            })
        );
    }

    #[test]
    fn set_argument_manquant_refuse()
    {
        assert!(parse("SET pseudo").is_err());
    }

    #[test]
    fn set_trop_d_arguments_refuse()
    {
        assert!(parse("SET titre Guerre et Paix").is_err());
    }

    #[test]
    fn get_sans_cle_refuse()
    {
        assert!(parse("GET").is_err());
    }

    #[test]
    fn commande_inconnue_refuse()
    {
        assert!(parse("BONJOUR").is_err());
    }

    #[test]
    fn ligne_vide_refuse()
    {
        assert!(parse("   ").is_err());
    }
}
