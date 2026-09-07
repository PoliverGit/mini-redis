# Contraintes

Ce document dit ce que le projet doit faire, ce qu'il ne fait pas, et pourquoi.
Il ne parle pas de code. Il se lit avant l'architecture.

---

## 1. Ce qu'est le projet

Un serveur qui garde des valeurs en mémoire, chacune rangée sous une clé, et qui
répond à des demandes tapées au clavier.

On dépose une valeur en donnant une clé. On la relit plus tard en redonnant
cette clé. Rien d'autre.

## 2. Comment on lui parle

Au clavier, dans le terminal.

On lance le programme. Il affiche une invite et attend. On tape une demande, on
valide, il répond, il réaffiche l'invite. On recommence autant qu'on veut.

Il n'y a **pas de réseau**. Un seul interlocuteur à la fois, celui qui est
devant le clavier. C'est un choix, pas une limite technique : on veut voir le
dialogue complet sans rien d'autre autour.

## 3. Ce que le stockage retient

Une valeur est un texte. Une clé est un texte.

Le stockage ne retient rien d'autre : pas de listes, pas d'ensembles, pas de
structures imbriquées. Une clé, un texte.

## 4. Ce que le stockage oublie

Tout, à la fin de la séance.

Rien n'est écrit sur le disque. Quand le programme s'arrête, le stockage
disparaît. C'est assumé : on veut apprendre le dialogue et le rangement, pas la
conservation à long terme.

## 5. Les valeurs qui expirent

On peut déposer une valeur en donnant un délai, en secondes ou en
millisecondes. Passé ce délai, elle n'existe plus.

Le délai doit être un nombre entier strictement positif. Un délai nul ou négatif
est refusé, parce qu'il n'a pas de sens : déposer une valeur déjà périmée est
presque toujours une erreur de frappe.

**Le nettoyage se fait au moment où on regarde.** Rien ne tourne en fond pour
faire le ménage. Une valeur périmée n'est constatée périmée que lorsqu'on vient
la chercher — et elle est alors retirée. Vu du dehors, la différence ne se voit
pas : une valeur périmée est toujours traitée comme absente.

Conséquence à connaître : déposer une nouvelle valeur sous une clé efface le
délai posé sur cette clé. En revanche, augmenter un compteur ne repousse pas
son délai. Dans un cas on remplace, dans l'autre on modifie.

## 6. Les demandes reconnues

| Demande | Ce qu'elle fait | Ce qu'elle rend |
|---|---|---|
| `PING` | vérifie que le serveur répond | `PONG` |
| `PING message` | idem, en renvoyant le message | le message |
| `SET cle valeur` | dépose une valeur | `OK` |
| `SET cle valeur EX n` | dépose une valeur pour `n` secondes | `OK` |
| `SET cle valeur PX n` | dépose une valeur pour `n` millisecondes | `OK` |
| `GET cle` | reprend une valeur | la valeur, ou rien |
| `DEL cle...` | retire une ou plusieurs clés | combien ont été retirées |
| `EXISTS cle...` | compte celles qui existent | combien existent |
| `TTL cle` | demande le délai restant | secondes, `-1`, ou `-2` |
| `INCR cle` | augmente un compteur de un | la nouvelle valeur |
| `DBSIZE` | compte les clés du stockage | le total |
| `FLUSHALL` | vide le stockage | `OK` |
| `QUIT` ou `EXIT` | termine la séance | `OK` |

Pour `TTL` : `-1` veut dire « cette clé existe et ne périmera jamais », `-2`
veut dire « cette clé n'existe pas ». Une seconde entamée compte
pour une seconde entière.

Pour `INCR` : une clé absente compte pour zéro, la première augmentation
donne donc un. Si la valeur déposée n'est pas un nombre entier, la demande est
refusée — on ne devine pas ce que l'utilisateur voulait.

## 7. Comment une demande est écrite

Le nom de la demande s'écrit indifféremment en majuscules ou en minuscules.
`GET`, `get` et `GeT` sont la même demande.

**Les clés et les valeurs, elles, gardent leur casse exacte.** `Paul` et
`paul` sont deux valeurs différentes. C'est un choix : le nom de la demande
appartient au serveur, la valeur appartient à l'utilisateur, et on ne touche
pas à ce qui appartient à l'utilisateur.

Les espaces en trop, avant, après, entre les mots, sont ignorés.

Une valeur qui contient des espaces s'écrit entre guillemets doubles. Un
guillemet ouvert et jamais refermé est une erreur signalée.

Une ligne vide n'appelle aucune réponse. On réaffiche simplement l'invite.

## 8. Ce que le serveur répond

Chaque réponse est reconnaissable à sa forme :

- une confirmation s'affiche `OK` ;
- un texte s'affiche entre guillemets, pour qu'on voie où il commence et où il
  finit — un texte vide reste donc visible ;
- une absence de valeur s'affiche `(nil)` ;
- un nombre s'affiche `(integer) 42` ;
- un refus s'affiche `(error)` suivi de la raison.

Ces formes sont reprises de Redis. Elles ont un mérite : on ne confond jamais le
texte `"42"` avec le nombre `42`, ni le texte vide avec l'absence de valeur.

Les raisons de refus sont écrites en français. Le projet sert à apprendre, pas à
imiter.

## 9. Comment le serveur traite une erreur

**Une demande refusée n'interrompt jamais la séance.** Le serveur explique
pourquoi il refuse, puis réaffiche l'invite. On corrige et on continue.

Le programme ne s'arrête que dans trois cas : on lui demande de s'arrêter, il
n'y a plus rien à lire au clavier, ou l'écran devient inutilisable.

## 10. Ce qu'on s'autorise et ce qu'on s'interdit

**Aucune obligation de tout écrire soi-même.** Une bibliothèque extérieure est
acceptée dès lors qu'elle fait exactement ce qu'on avait décidé de faire.
L'implémentation va au plus simple. Ce qui doit être compris, c'est le
fonctionnement — pourquoi on découpe une ligne de cette manière, pourquoi on
mesure un délai ainsi — pas la manière dont ces gestes sont écrits.

En l'état, le projet n'utilise aucune bibliothèque extérieure : il n'en a pas eu
besoin. C'est un constat, pas une règle.

**Aucun raccourci entre les parties.** Le stockage ne fait référence ni au
clavier, ni aux commandes. La partie qui comprend les commandes ne touche
jamais au stockage. Chaque partie fait une seule chose et n'appelle que celles
qui sont sous elle.

## 11. Comment on vérifie

Deux niveaux, et ils ne se remplacent pas.

**Chaque partie est vérifiée dans son propre fichier**, isolée du reste. On y
vérifie son comportement seule : ce qui entre, ce qui sort.

**Le dialogue complet est vérifié de l'extérieur**, dans un dossier à part. Là,
on ne connaît aucune partie : on lance le programme comme n'importe qui, on lui
donne des lignes comme si on les tapait, on compare tout ce qui s'est affiché.

Le second niveau attrape ce que le premier ne peut pas voir : l'enchaînement des
demandes, la persistance du stockage d'un bout à l'autre de la séance, l'invite,
et la fin propre du programme.

Un test vérifie une chose et une seule. Son nom dit ce qu'il vérifie.
