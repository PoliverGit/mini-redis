//! L'exécution.
//!
//! Cette partie prend une demande comprise et va la faire dans le stockage, puis
//! rend la réponse destinée à l'écran. C'est la seule partie qui agit.

use crate::command::Command;
use crate::reply::Reponse;
use crate::store::{Duree, Store};

/// Une clé absente n'a pas de durée de vie : on le dit par ce nombre.
const CLE_ABSENTE: i64 = -2;
/// Une clé sans limite de temps : on le dit par ce nombre.
const SANS_LIMITE: i64 = -1;

pub fn execute(commande: Command, store: &mut Store) -> Reponse
{
    match commande
    {
        Command::Ping(None) => Reponse::Texte("PONG".to_string()),
        Command::Ping(Some(message)) => Reponse::Texte(message),

        Command::Set { cle, valeur, duree } =>
        {
            match duree
            {
                Some(delai) => store.set_avec_duree(&cle, &valeur, delai),
                None => store.set(&cle, &valeur),
            }
            Reponse::Ok
        }

        Command::Get { cle } => match store.get(&cle)
        {
            Some(valeur) => Reponse::Texte(valeur.to_string()),
            None => Reponse::Rien,
        },

        Command::Del { cles } =>
        {
            let retirees = cles.iter().filter(|cle| store.del(cle)).count();
            Reponse::Entier(retirees as i64)
        }

        Command::Exists { cles } =>
        {
            let presentes = cles.iter().filter(|cle| store.existe(cle)).count();
            Reponse::Entier(presentes as i64)
        }

        Command::Ttl { cle } => match store.duree_restante(&cle)
        {
            Duree::Absente => Reponse::Entier(CLE_ABSENTE),
            Duree::Illimitee => Reponse::Entier(SANS_LIMITE),
            Duree::Restante(reste) => Reponse::Entier(secondes_arrondies_au_dessus(reste)),
        },

        Command::Incr { cle } => incrementer(&cle, store),

        Command::Dbsize => Reponse::Entier(store.taille() as i64),

        Command::Flushall =>
        {
            store.vider();
            Reponse::Ok
        }

        Command::Quit => Reponse::Ok,
    }
}

/// Augmente de un la valeur d'une clé. Une clé absente compte pour zéro.
/// La limite de temps déjà posée sur la clé n'est pas touchée.
fn incrementer(cle: &str, store: &mut Store) -> Reponse
{
    let actuelle = match store.get(cle)
    {
        None => 0,
        Some(texte) => match texte.parse::<i64>()
        {
            Ok(nombre) => nombre,
            Err(_) =>
            {
                return Reponse::Erreur("ERR la valeur n'est pas un nombre entier".to_string());
            }
        },
    };

    match actuelle.checked_add(1)
    {
        Some(suivante) =>
        {
            store.set_en_gardant_la_duree(cle, &suivante.to_string());
            Reponse::Entier(suivante)
        }
        None => Reponse::Erreur(
            "ERR la valeur n'est pas un nombre entier ou dépasse les limites".to_string(),
        ),
    }
}

/// Une seconde entamée est une seconde comptée, comme le fait Redis.
fn secondes_arrondies_au_dessus(reste: std::time::Duration) -> i64
{
    let secondes = reste.as_secs() as i64;
    if reste.subsec_nanos() > 0
    {
        secondes + 1
    }
    else
    {
        secondes
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    fn set(store: &mut Store, cle: &str, valeur: &str)
    {
        execute(
            Command::Set {
                cle: cle.to_string(),
                valeur: valeur.to_string(),
                duree: None,
            },
            store,
        );
    }

    #[test]
    fn ping_seul_repond_pong()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(Command::Ping(None), &mut store),
            Reponse::Texte("PONG".to_string())
        );
    }

    #[test]
    fn ping_avec_message_renvoie_le_message()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(Command::Ping(Some("bonjour".to_string())), &mut store),
            Reponse::Texte("bonjour".to_string())
        );
    }

    #[test]
    fn set_repond_ok()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(
                Command::Set {
                    cle: "pseudo".to_string(),
                    valeur: "paul".to_string(),
                    duree: None,
                },
                &mut store
            ),
            Reponse::Ok
        );
    }

    #[test]
    fn get_apres_set_rend_la_valeur()
    {
        let mut store = Store::new();
        set(&mut store, "pseudo", "paul");
        assert_eq!(
            execute(
                Command::Get {
                    cle: "pseudo".to_string()
                },
                &mut store
            ),
            Reponse::Texte("paul".to_string())
        );
    }

    #[test]
    fn get_sur_cle_absente_ne_rend_rien()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(
                Command::Get {
                    cle: "inconnue".to_string()
                },
                &mut store
            ),
            Reponse::Rien
        );
    }

    #[test]
    fn set_avec_delai_puis_attente_rend_la_cle_absente()
    {
        let mut store = Store::new();
        execute(
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: Some(Duration::from_millis(20)),
            },
            &mut store,
        );
        sleep(Duration::from_millis(80));
        assert_eq!(
            execute(
                Command::Get {
                    cle: "pseudo".to_string()
                },
                &mut store
            ),
            Reponse::Rien
        );
    }

    #[test]
    fn del_compte_les_cles_reellement_retirees()
    {
        let mut store = Store::new();
        set(&mut store, "a", "1");
        set(&mut store, "b", "2");
        assert_eq!(
            execute(
                Command::Del {
                    cles: vec!["a".to_string(), "b".to_string(), "inconnue".to_string()],
                },
                &mut store
            ),
            Reponse::Entier(2)
        );
    }

    #[test]
    fn del_sur_cles_absentes_compte_zero()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(
                Command::Del {
                    cles: vec!["inconnue".to_string()],
                },
                &mut store
            ),
            Reponse::Entier(0)
        );
    }

    #[test]
    fn exists_compte_les_cles_presentes()
    {
        let mut store = Store::new();
        set(&mut store, "a", "1");
        assert_eq!(
            execute(
                Command::Exists {
                    cles: vec!["a".to_string(), "inconnue".to_string()],
                },
                &mut store
            ),
            Reponse::Entier(1)
        );
    }

    #[test]
    fn ttl_sur_cle_absente_rend_moins_deux()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(
                Command::Ttl {
                    cle: "inconnue".to_string()
                },
                &mut store
            ),
            Reponse::Entier(-2)
        );
    }

    #[test]
    fn ttl_sur_cle_sans_limite_rend_moins_un()
    {
        let mut store = Store::new();
        set(&mut store, "pseudo", "paul");
        assert_eq!(
            execute(
                Command::Ttl {
                    cle: "pseudo".to_string()
                },
                &mut store
            ),
            Reponse::Entier(-1)
        );
    }

    #[test]
    fn ttl_sur_cle_a_delai_rend_les_secondes_restantes()
    {
        let mut store = Store::new();
        execute(
            Command::Set {
                cle: "pseudo".to_string(),
                valeur: "paul".to_string(),
                duree: Some(Duration::from_secs(10)),
            },
            &mut store,
        );
        assert_eq!(
            execute(
                Command::Ttl {
                    cle: "pseudo".to_string()
                },
                &mut store
            ),
            Reponse::Entier(10)
        );
    }

    #[test]
    fn incr_sur_cle_absente_part_de_zero()
    {
        let mut store = Store::new();
        assert_eq!(
            execute(
                Command::Incr {
                    cle: "compteur".to_string()
                },
                &mut store
            ),
            Reponse::Entier(1)
        );
    }

    #[test]
    fn incr_augmente_de_un_a_chaque_fois()
    {
        let mut store = Store::new();
        set(&mut store, "compteur", "41");
        assert_eq!(
            execute(
                Command::Incr {
                    cle: "compteur".to_string()
                },
                &mut store
            ),
            Reponse::Entier(42)
        );
        assert_eq!(
            execute(
                Command::Get {
                    cle: "compteur".to_string()
                },
                &mut store
            ),
            Reponse::Texte("42".to_string())
        );
    }

    #[test]
    fn incr_sur_valeur_qui_n_est_pas_un_nombre_est_refuse()
    {
        let mut store = Store::new();
        set(&mut store, "pseudo", "paul");
        assert_eq!(
            execute(
                Command::Incr {
                    cle: "pseudo".to_string()
                },
                &mut store
            ),
            Reponse::Erreur("ERR la valeur n'est pas un nombre entier".to_string())
        );
    }

    #[test]
    fn incr_au_dela_de_la_limite_est_refuse()
    {
        let mut store = Store::new();
        set(&mut store, "compteur", &i64::MAX.to_string());
        assert_eq!(
            execute(
                Command::Incr {
                    cle: "compteur".to_string()
                },
                &mut store
            ),
            Reponse::Erreur(
                "ERR la valeur n'est pas un nombre entier ou dépasse les limites".to_string()
            )
        );
    }

    #[test]
    fn incr_ne_reporte_pas_la_limite_de_temps()
    {
        let mut store = Store::new();
        execute(
            Command::Set {
                cle: "compteur".to_string(),
                valeur: "1".to_string(),
                duree: Some(Duration::from_millis(40)),
            },
            &mut store,
        );
        execute(
            Command::Incr {
                cle: "compteur".to_string(),
            },
            &mut store,
        );
        sleep(Duration::from_millis(90));
        assert_eq!(
            execute(
                Command::Get {
                    cle: "compteur".to_string()
                },
                &mut store
            ),
            Reponse::Rien
        );
    }

    #[test]
    fn dbsize_compte_les_cles()
    {
        let mut store = Store::new();
        assert_eq!(execute(Command::Dbsize, &mut store), Reponse::Entier(0));
        set(&mut store, "a", "1");
        set(&mut store, "b", "2");
        assert_eq!(execute(Command::Dbsize, &mut store), Reponse::Entier(2));
    }

    #[test]
    fn flushall_vide_le_stockage()
    {
        let mut store = Store::new();
        set(&mut store, "a", "1");
        assert_eq!(execute(Command::Flushall, &mut store), Reponse::Ok);
        assert_eq!(execute(Command::Dbsize, &mut store), Reponse::Entier(0));
    }

    #[test]
    fn quit_repond_ok()
    {
        let mut store = Store::new();
        assert_eq!(execute(Command::Quit, &mut store), Reponse::Ok);
    }
}
