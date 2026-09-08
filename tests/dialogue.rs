//! Le dialogue complet, vu de l'extérieur.
//!
//! Ici, on ne connaît aucune partie du projet. On lance le programme comme le
//! ferait n'importe qui, on lui donne des lignes comme si on les tapait, et on
//! regarde tout ce qui s'est affiché à l'écran.

use std::io::Write;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

/// CARGO_BIN_EXE_mini-redis = /chemin/vers/mini-redis
fn session(saisie: &str) -> String
{
    let mut programme = Command::new(env!("CARGO_BIN_EXE_mini-redis"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("le programme devait démarrer");
    programme
        .stdin
        .as_mut()
        .expect("l'entrée devait être ouverte")
        .write_all(saisie.as_bytes())
        .expect("la saisie devait être transmise");
    let resultat = programme
        .wait_with_output()
        .expect("le programme devait se terminer");
    assert!(
        resultat.status.success(),
        "le programme s'est terminé en erreur : {}",
        String::from_utf8_lossy(&resultat.stderr)
    );
    String::from_utf8(resultat.stdout).expect("la sortie devait être lisible")
}

/// Comme `session`, mais on marque une pause au milieu de la saisie pour
/// laisser le temps passer entre deux lignes.
fn session_en_deux_temps(avant: &str, pause: Duration, apres: &str) -> String
{
    let mut programme = Command::new(env!("CARGO_BIN_EXE_mini-redis"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("le programme devait démarrer");
    {
        let entree = programme
            .stdin
            .as_mut()
            .expect("l'entrée devait être ouverte");
        entree
            .write_all(avant.as_bytes())
            .expect("la première saisie devait être transmise");
        entree.flush().expect("la première saisie devait partir");
        sleep(pause);
        entree
            .write_all(apres.as_bytes())
            .expect("la seconde saisie devait être transmise");
    }
    let resultat = programme
        .wait_with_output()
        .expect("le programme devait se terminer");
    assert!(
        resultat.status.success(),
        "le programme s'est terminé en erreur : {}",
        String::from_utf8_lossy(&resultat.stderr)
    );
    String::from_utf8(resultat.stdout).expect("la sortie devait être lisible")
}

#[test]
fn une_session_complete_du_debut_a_la_fin()
{
    let affichage = session(
        "PING\n\
         SET pseudo paul\n\
         GET pseudo\n\
         EXISTS pseudo inconnue\n\
         TTL pseudo\n\
         DEL pseudo\n\
         GET pseudo\n\
         QUIT\n",
    );
    assert_eq!(
        affichage,
        "> \"PONG\"\n\
         > OK\n\
         > \"paul\"\n\
         > (integer) 1\n\
         > (integer) -1\n\
         > (integer) 1\n\
         > (nil)\n\
         > OK\n"
    );
}

#[test]
fn le_stockage_garde_les_valeurs_pendant_toute_la_session()
{
    let affichage = session("SET a 1\nSET b 2\nDBSIZE\nFLUSHALL\nDBSIZE\n");
    assert_eq!(
        affichage,
        "> OK\n> OK\n> (integer) 2\n> OK\n> (integer) 0\n> \n"
    );
}

#[test]
fn une_valeur_avec_espaces_revient_intacte()
{
    let affichage = session("SET titre \"guerre et paix\"\nGET titre\n");
    assert_eq!(affichage, "> OK\n> \"guerre et paix\"\n> \n");
}

#[test]
fn un_compteur_monte_d_une_commande_a_l_autre()
{
    let affichage = session("INCR vues\nINCR vues\nINCR vues\nGET vues\n");
    assert_eq!(
        affichage,
        "> (integer) 1\n> (integer) 2\n> (integer) 3\n> \"3\"\n> \n"
    );
}

#[test]
fn une_ligne_refusee_n_interrompt_pas_la_session()
{
    let affichage = session("BONJOUR\nGET\nSET pseudo paul\nGET pseudo\n");
    assert_eq!(
        affichage,
        "> (error) ERR commande inconnue « BONJOUR »\n\
         > (error) ERR nombre d'arguments incorrect pour la commande « get »\n\
         > OK\n\
         > \"paul\"\n\
         > \n"
    );
}

#[test]
fn une_valeur_a_duree_limitee_disparait_toute_seule()
{
    // On tape les premières lignes, on laisse le temps passer, puis on
    // redemande la valeur : elle n'est plus là.
    let affichage = session_en_deux_temps(
        "SET jeton secret PX 50\nGET jeton\n",
        Duration::from_millis(200),
        "GET jeton\nTTL jeton\nQUIT\n",
    );
    assert_eq!(
        affichage,
        "> OK\n\
         > \"secret\"\n\
         > (nil)\n\
         > (integer) -2\n\
         > OK\n"
    );
}

#[test]
fn la_fin_de_saisie_termine_le_programme_proprement()
{
    assert_eq!(session(""), "> \n");
}
