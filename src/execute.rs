use crate::command::Command;
use crate::store::Store;

/// Exécute une commande sur le store et rend la réponse destinée au client.
///
///     Set          -> "OK"          (l'ancienne valeur est jetée)
///     Get trouvé   -> la valeur
///     Get absent   -> "(nil)"
///     Del supprimé -> "1"
///     Del absent   -> "0"
pub fn execute(commande: Command, store: &mut Store) -> String
{
    match commande
    {
        Command::Set { key, value } =>
        {
            store.set(&key, &value);
            "OK".to_string()
        }
        Command::Get { key } => match store.get(&key)
        {
            Some(valeur) => valeur.to_string(),
            None => "(nil)".to_string(),
        },
        Command::Del { key } => match store.del(&key)
        {
            Some(_) => "1".to_string(),
            None => "0".to_string(),
        },
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn set_rend_ok()
    {
        let mut store = Store::new();
        let c = Command::Set { key: "pseudo".to_string(), value: "paul".to_string() };
        assert_eq!(execute(c, &mut store), "OK");
    }

    #[test]
    fn get_trouve_rend_la_valeur()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        let c = Command::Get { key: "pseudo".to_string() };
        assert_eq!(execute(c, &mut store), "paul");
    }

    #[test]
    fn get_absent_rend_nil()
    {
        let mut store = Store::new();
        let c = Command::Get { key: "inconnue".to_string() };
        assert_eq!(execute(c, &mut store), "(nil)");
    }

    #[test]
    fn del_supprime_rend_1()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        let c = Command::Del { key: "pseudo".to_string() };
        assert_eq!(execute(c, &mut store), "1");
    }

    #[test]
    fn del_absent_rend_0()
    {
        let mut store = Store::new();
        let c = Command::Del { key: "inconnue".to_string() };
        assert_eq!(execute(c, &mut store), "0");
    }

    #[test]
    fn set_ecrase_la_valeur_precedente()
    {
        let mut store = Store::new();
        execute(Command::Set { key: "k".to_string(), value: "v1".to_string() }, &mut store);
        execute(Command::Set { key: "k".to_string(), value: "v2".to_string() }, &mut store);
        assert_eq!(execute(Command::Get { key: "k".to_string() }, &mut store), "v2");
    }
}
