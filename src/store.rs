//! Le stockage.
//!
//! C'est l'endroit où les valeurs sont rangées, chacune sous une clé, avec
//! éventuellement une date au-delà de laquelle elle ne compte plus.
//! Le stockage ne sait rien du clavier, ni des commandes, ni de l'affichage.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Ce qui est réellement rangé sous une clé.
#[derive(Debug, Clone, PartialEq)]
struct Entree
{
    valeur: String,
    perime_le: Option<Instant>,
}

impl Entree
{
    fn est_perimee(&self, maintenant: Instant) -> bool
    {
        match self.perime_le
        {
            Some(limite) => maintenant >= limite,
            None => false,
        }
    }
}

/// Le délai restant sur une clé.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Duree
{
    /// La clé n'existe pas, ou n'existe plus.
    Absente,
    /// La clé existe et n'a pas de délai de péremption.
    Illimitee,
    /// La clé existe et sera retirée une fois ce délai écoulé.
    Restante(Duration),
}

#[derive(Debug, Default)]
pub struct Store
{
    stock: HashMap<String, Entree>,
}

impl Store
{
    pub fn new() -> Self
    {
        Store {
            stock: HashMap::new(),
        }
    }

    /// Range une valeur sous une clé, sans limite de temps.
    /// Ce qui s'y trouvait avant est remplacé, sa limite de temps comprise.
    pub fn set(&mut self, cle: &str, valeur: &str)
    {
        self.stock.insert(
            cle.to_string(),
            Entree {
                valeur: valeur.to_string(),
                perime_le: None,
            },
        );
    }

    /// Range une valeur qui sera périmée une fois `duree` écoulée.
    pub fn set_avec_duree(&mut self, cle: &str, valeur: &str, duree: Duration)
    {
        self.stock.insert(
            cle.to_string(),
            Entree {
                valeur: valeur.to_string(),
                perime_le: Some(Instant::now() + duree),
            },
        );
    }

    /// Lit une valeur. Une valeur périmée est traitée comme absente, et elle
    /// est retirée au passage.
    pub fn get(&mut self, cle: &str) -> Option<&str>
    {
        self.oublier_si_perimee(cle);
        self.stock.get(cle).map(|entree| entree.valeur.as_str())
    }

    /// Retire une clé. Rend vrai si quelque chose a réellement été retiré.
    pub fn del(&mut self, cle: &str) -> bool
    {
        self.oublier_si_perimee(cle);
        self.stock.remove(cle).is_some()
    }

    /// Rend vrai si la clé est présente et pas périmée.
    pub fn existe(&mut self, cle: &str) -> bool
    {
        self.oublier_si_perimee(cle);
        self.stock.contains_key(cle)
    }

    /// Le délai restant sur une clé.
    pub fn duree_restante(&mut self, cle: &str) -> Duree
    {
        self.oublier_si_perimee(cle);
        let maintenant = Instant::now();
        match self.stock.get(cle)
        {
            None => Duree::Absente,
            Some(entree) => match entree.perime_le
            {
                None => Duree::Illimitee,
                Some(limite) => Duree::Restante(limite.saturating_duration_since(maintenant)),
            },
        }
    }

    /// Remplace la valeur d'une clé en lui laissant la limite de temps qu'elle
    /// avait déjà. Si la clé n'existe pas, la valeur est rangée sans limite.
    pub fn set_en_gardant_la_duree(&mut self, cle: &str, valeur: &str)
    {
        self.oublier_si_perimee(cle);
        match self.stock.get_mut(cle)
        {
            Some(entree) => entree.valeur = valeur.to_string(),
            None => self.set(cle, valeur),
        }
    }

    /// Vide entièrement le stockage.
    pub fn vider(&mut self)
    {
        self.stock.clear();
    }

    /// Nombre de clés présentes et non périmées.
    pub fn taille(&mut self) -> usize
    {
        self.oublier_les_perimees();
        self.stock.len()
    }

    fn oublier_si_perimee(&mut self, cle: &str)
    {
        let maintenant = Instant::now();
        if let Some(entree) = self.stock.get(cle)
            && entree.est_perimee(maintenant)
        {
            self.stock.remove(cle);
        }
    }

    fn oublier_les_perimees(&mut self)
    {
        let maintenant = Instant::now();
        self.stock.retain(|_, entree| !entree.est_perimee(maintenant));
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::thread::sleep;

    #[test]
    fn get_sur_stockage_vide_ne_rend_rien()
    {
        let mut store = Store::new();
        assert_eq!(store.get("pseudo"), None);
    }

    #[test]
    fn set_puis_get_rend_la_valeur()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        assert_eq!(store.get("pseudo"), Some("paul"));
    }

    #[test]
    fn set_ecrase_la_valeur_precedente()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        store.set("pseudo", "paula");
        assert_eq!(store.get("pseudo"), Some("paula"));
    }

    #[test]
    fn deux_cles_ne_se_melangent_pas()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        store.set("ville", "lyon");
        assert_eq!(store.get("pseudo"), Some("paul"));
        assert_eq!(store.get("ville"), Some("lyon"));
    }

    #[test]
    fn del_retire_la_cle()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        assert!(store.del("pseudo"));
        assert_eq!(store.get("pseudo"), None);
    }

    #[test]
    fn del_sur_cle_absente_ne_retire_rien()
    {
        let mut store = Store::new();
        assert!(!store.del("inconnue"));
    }

    #[test]
    fn existe_suit_les_ajouts_et_les_retraits()
    {
        let mut store = Store::new();
        assert!(!store.existe("pseudo"));
        store.set("pseudo", "paul");
        assert!(store.existe("pseudo"));
        store.del("pseudo");
        assert!(!store.existe("pseudo"));
    }

    #[test]
    fn valeur_a_duree_limitee_disparait_apres_le_delai()
    {
        let mut store = Store::new();
        store.set_avec_duree("pseudo", "paul", Duration::from_millis(20));
        assert_eq!(store.get("pseudo"), Some("paul"));
        sleep(Duration::from_millis(80));
        assert_eq!(store.get("pseudo"), None);
        assert!(!store.existe("pseudo"));
    }

    #[test]
    fn set_sans_duree_efface_la_duree_precedente()
    {
        let mut store = Store::new();
        store.set_avec_duree("pseudo", "paul", Duration::from_millis(20));
        store.set("pseudo", "paula");
        sleep(Duration::from_millis(80));
        assert_eq!(store.get("pseudo"), Some("paula"));
    }

    #[test]
    fn duree_restante_dit_absente_pour_une_cle_inconnue()
    {
        let mut store = Store::new();
        assert_eq!(store.duree_restante("pseudo"), Duree::Absente);
    }

    #[test]
    fn duree_restante_dit_illimitee_pour_une_valeur_sans_delai()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        assert_eq!(store.duree_restante("pseudo"), Duree::Illimitee);
    }

    #[test]
    fn duree_restante_compte_a_rebours_pour_une_valeur_a_delai()
    {
        let mut store = Store::new();
        store.set_avec_duree("pseudo", "paul", Duration::from_secs(10));
        match store.duree_restante("pseudo")
        {
            Duree::Restante(reste) =>
            {
                assert!(reste <= Duration::from_secs(10));
                assert!(reste > Duration::from_secs(8));
            }
            autre => panic!("durée inattendue : {autre:?}"),
        }
    }

    #[test]
    fn remplacer_la_valeur_conserve_la_limite_de_temps()
    {
        let mut store = Store::new();
        store.set_avec_duree("compteur", "1", Duration::from_millis(40));
        store.set_en_gardant_la_duree("compteur", "2");
        assert_eq!(store.get("compteur"), Some("2"));
        sleep(Duration::from_millis(90));
        assert_eq!(store.get("compteur"), None);
    }

    #[test]
    fn remplacer_la_valeur_d_une_cle_absente_la_cree_sans_limite()
    {
        let mut store = Store::new();
        store.set_en_gardant_la_duree("compteur", "1");
        assert_eq!(store.get("compteur"), Some("1"));
        assert_eq!(store.duree_restante("compteur"), Duree::Illimitee);
    }

    #[test]
    fn vider_supprime_tout()
    {
        let mut store = Store::new();
        store.set("a", "1");
        store.set("b", "2");
        store.vider();
        assert_eq!(store.taille(), 0);
        assert_eq!(store.get("a"), None);
    }

    #[test]
    fn taille_ne_compte_pas_les_valeurs_perimees()
    {
        let mut store = Store::new();
        store.set("permanent", "1");
        store.set_avec_duree("ephemere", "2", Duration::from_millis(20));
        assert_eq!(store.taille(), 2);
        sleep(Duration::from_millis(80));
        assert_eq!(store.taille(), 1);
    }
}
