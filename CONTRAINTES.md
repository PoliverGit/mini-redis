# Contraintes

Ce que le projet fait, ce qu'il ne fait pas, et pourquoi.

---

## 1. Ce qu'est le projet

Un serveur qui garde des valeurs en mémoire, chacune rangée sous une clé.

On dépose une valeur en donnant une clé. On la relit en redonnant cette clé.

## 2. Comment on lui parle

Au clavier, dans le terminal. Il affiche une invite, on tape une demande, il
répond, il réaffiche l'invite.

Pas de réseau. Un seul interlocuteur, celui qui est devant le clavier. C'est un
choix : on veut voir le dialogue complet sans rien d'autre autour.

## 3. Ce que le stockage retient

Une clé est un texte, une valeur est un texte. Rien d'autre : pas de listes,
pas d'ensembles, pas de structures imbriquées.

## 4. Ce que le stockage oublie

Tout, à la fin de la session. Rien n'est écrit sur le disque.

On veut apprendre le dialogue et le rangement, pas la conservation.

## 5. Les valeurs qui expirent

On peut déposer une valeur avec un délai, en secondes ou en millisecondes.
Passé ce délai, elle n'existe plus.

Le délai doit être un entier strictement positif. Zéro ou négatif est refusé :
déposer une valeur déjà périmée est presque toujours une faute de frappe.

**Le nettoyage se fait au moment où on regarde.** Rien ne tourne en fond. Une
valeur périmée n'est constatée périmée que lorsqu'une demande vient la
chercher, et elle est retirée à cet instant-là. Vu du dehors, une valeur
périmée est simplement absente.

Déposer une nouvelle valeur sous une clé efface le délai posé dessus.
Augmenter un compteur ne le repousse pas. Dans un cas on remplace, dans l'autre
on modifie.

## 6. Les demandes reconnues

| Demande | Ce qu'elle fait | Ce qu'elle rend |
|---|---|---|
| `PING` | vérifie que le serveur répond | `PONG` |
| `PING message` | idem, en renvoyant le message | le message |
| `SET cle valeur` | dépose une valeur | `OK` |
| `SET cle valeur EX n` | dépose pour `n` secondes | `OK` |
| `SET cle valeur PX n` | dépose pour `n` millisecondes | `OK` |
| `GET cle` | reprend une valeur | la valeur, ou rien |
| `DEL cle...` | retire une ou plusieurs clés | combien ont été retirées |
| `EXISTS cle...` | compte celles qui existent | combien existent |
| `TTL cle` | demande le délai restant | secondes, `-1`, ou `-2` |
| `INCR cle` | augmente un compteur de un | la nouvelle valeur |
| `DBSIZE` | compte les clés | le total |
| `FLUSHALL` | vide le stockage | `OK` |
| `QUIT` ou `EXIT` | termine la session | `OK` |

`TTL` : `-1` veut dire « la clé existe et ne périmera jamais », `-2` veut dire
« la clé n'existe pas ». Une seconde entamée compte pour une seconde entière.

`INCR` : une clé absente compte pour zéro, la première augmentation donne donc
un. Si la valeur rangée n'est pas un entier, la demande est refusée — on ne
devine pas ce que l'utilisateur voulait.

## 7. Comment une demande est écrite

Le nom de la demande s'écrit en majuscules ou en minuscules indifféremment.

Les clés et les valeurs gardent leur casse exacte : `Paul` et `paul` sont deux
valeurs différentes. Le nom de la demande appartient au serveur, la valeur
appartient à l'utilisateur.

Les espaces en trop sont ignorés. Une valeur qui contient des espaces s'écrit
entre guillemets doubles ; un guillemet jamais refermé est refusé.

Une ligne vide n'appelle aucune réponse.

## 8. Ce que le serveur répond

| Nature | Affichage |
|---|---|
| confirmation | `OK` |
| texte | entre guillemets, `"paul"` |
| absence | `(nil)` |
| nombre | `(integer) 42` |
| refus | `(error) ` suivi de la raison |

Un nombre est calculé par le serveur ; un texte est ce que l'utilisateur a
rangé. C'est pourquoi ils ne s'affichent pas pareil : on ne confond jamais le
texte `"42"` avec le nombre `42`, ni le texte vide avec l'absence de valeur.

Ces formes sont reprises de Redis. Les raisons de refus sont en français : le
projet sert à apprendre, pas à imiter.

## 9. Comment le serveur traite une erreur

Une demande refusée n'interrompt jamais la session. Le serveur dit pourquoi il
refuse et réaffiche l'invite.

Le programme ne s'arrête que dans trois cas : on lui demande, il n'y a plus rien
à lire, ou l'écran devient inutilisable.

## 10. Ce qu'on s'autorise et ce qu'on s'interdit

Une bibliothèque extérieure est acceptée si elle fait exactement ce qu'on avait
décidé de faire. Ce qui doit être compris, c'est le fonctionnement, pas la
manière dont il est écrit. En l'état le projet n'en utilise aucune : c'est un
constat, pas une règle.

Aucun raccourci entre les parties. Le stockage ne fait référence ni au clavier
ni aux commandes. La traduction ne touche jamais au stockage. Chaque partie
n'appelle que celles qui sont sous elle.

## 11. Comment on vérifie

Deux niveaux, qui ne se remplacent pas.

Chaque partie est vérifiée dans son propre fichier, isolée : ce qui entre, ce
qui sort.

Le dialogue complet est vérifié de l'extérieur, dans un dossier à part : on
lance le programme, on lui donne des lignes comme si on les tapait, on compare
tout ce qui s'est affiché.

Le second attrape ce que le premier ne peut pas voir : l'enchaînement des
demandes, le stockage identique d'un bout à l'autre, l'invite, et la fin propre
du programme.

Un test vérifie une chose et une seule. Son nom dit laquelle.
