# Architecture

Les parties du projet, une par une, et ce qu'elles s'échangent.

---

## Le principe

Une demande tapée au clavier traverse le projet en ligne droite, et chaque
partie ne fait qu'une seule chose dessus :

```
   clavier
      │
      ▼
  la session   ── lit une ligne, affiche l'invite et la réponse
      │
      ▼
  la commande  ── comprend la ligne, ou la refuse
      │
      ▼
  l'exécution  ── fait ce qui est demandé
      │
      ▼
  le stockage  ── range et rend les valeurs
      │
      ▼
  la réponse   ── met en forme ce qu'on renverra
      │
      ▼
   écran
```

Le trait important est le sens des flèches. Le stockage n'appelle jamais la
session. La commande n'appelle jamais le stockage. Chaque partie n'appelle que
celles qui sont sous elle.

---

## L'arborescence

```
mini-redis/
├── Cargo.toml            le nom, la version et les dépendances
├── CLAUDE.md             les règles de travail
├── CONTRAINTES.md        ce que le projet doit faire
├── ARCHITECTURE.md       ce document
├── src/
│   ├── main.rs           le point d'entrée
│   ├── session.rs        la session
│   ├── command.rs        la commande
│   ├── execute.rs        l'exécution
│   ├── store.rs          le stockage
│   └── reply.rs          la réponse
└── tests/
    └── dialogue.rs       le dialogue complet, vu de l'extérieur
```

---

## Les parties

### `src/store.rs` — le stockage

Range des valeurs sous des clés et les rend quand on les demande. Il tient la
date de péremption de celles qui en ont une.

Il sait déposer, déposer avec un délai, rendre, retirer, dire si une clé
existe, dire le délai restant, se vider, se compter. Il sait aussi remplacer
une valeur en lui laissant son délai — ce dont un compteur a besoin pour que
son délai ne soit pas remis à zéro à chaque augmentation.

Il ne fait référence ni au clavier, ni aux commandes, ni à l'affichage.

**Le point délicat.** C'est lui qui décide qu'une valeur est périmée, au moment
où on la regarde, et il la retire à cet instant-là. Pour toutes les parties
au-dessus, elle est simplement absente.

---

### `src/command.rs` — la commande

Traduit une ligne de texte en demande claire, ou la refuse en disant pourquoi.

Il découpe la ligne en mots en tenant compte des guillemets, reconnaît le nom
de la demande sans se soucier de la casse, vérifie que le nombre de mots
correspond, et lit un délai de vie.

Il ignore ce que la demande produira. Il ne sait pas qu'un stockage existe.

**Le point délicat.** C'est la seule partie qui peut refuser une ligne. Les
demandes qui en sortent sont valides : les parties suivantes n'ont plus à
vérifier quoi que ce soit.

Il distingue trois issues, pas deux : une demande comprise, un refus motivé, et
une ligne vide, qui n'appelle aucune réponse.

---

### `src/reply.rs` — la réponse

Dit ce que le serveur renvoie et sous quelle forme cela s'affiche.

Il distingue les cinq natures — confirmation, texte, absence, nombre, refus —
et donne à chacune son apparence.

Il ignore d'où vient la réponse. Il ne décide rien.

**Le point délicat.** C'est lui qui entoure les textes de guillemets et protège
les guillemets contenus dans le texte. Sans ça, on ne distinguerait pas un
texte vide d'une absence de valeur.

---

### `src/execute.rs` — l'exécution

Prend une demande comprise, la fait dans le stockage, et rend la réponse
correspondante.

C'est ici qu'on décide qu'une clé absente vaut `-2` pour le délai restant,
qu'un compteur absent part de zéro, qu'une seconde entamée compte pour une
seconde entière.

Il ignore comment la demande est arrivée et comment la réponse sera affichée.

**Le point délicat.** C'est la seule partie qui modifie le stockage. Quand un
résultat est inattendu, c'est ici qu'on regarde en premier.

---

### `src/session.rs` — la session

Tient le fil du dialogue : afficher l'invite, lire une ligne, la faire
comprendre, la faire exécuter, écrire la réponse, recommencer.

Elle décide quand s'arrêter — quand on le lui demande, ou quand il n'y a plus
rien à lire — et fait en sorte qu'un refus n'interrompe pas la session. Elle ne
juge pas si le programme a réussi : elle rend « terminé » ou « interrompu », et
c'est le point d'entrée qui en tire le code de sortie.

Elle ignore le détail de chaque demande. Une seule l'intéresse, la demande
d'arrêt, parce que c'est elle qui termine la boucle.

**Le point délicat.** Elle ne lit pas directement le clavier et n'écrit pas
directement à l'écran : on lui donne une source de lignes, une destination, et
un stockage. C'est ce qui permet de rejouer une session entière en vérification
avec du texte préparé, et de relire tout ce qu'elle a produit. Sans ça, la
boucle serait la seule partie impossible à vérifier.

---

### `src/main.rs` — le point d'entrée

Monte les parties et lance la session sur le vrai clavier et le vrai écran.

Il crée le stockage vide, branche le clavier et l'écran, lance la session, et
rend au terminal `0` si elle est allée à son terme, `1` si l'écran est devenu
inutilisable.

C'est volontairement la partie la plus courte : elle ne contient aucune
décision, donc aucun test.

---

### `tests/dialogue.rs` — le dialogue vu de l'extérieur

Vérifie le projet comme le ferait quelqu'un qui ne connaît aucune de ses
parties : il lance le programme, lui envoie des lignes comme si on les tapait,
attend qu'il se termine, et compare tout ce qui s'est affiché — invites
comprises.

Les vérifications faites dans chaque fichier regardent une partie isolée. Elles
ne peuvent rien dire de l'enchaînement complet : que le stockage est le même du
début à la fin, qu'un refus ne coupe pas le fil, qu'une valeur périmée
disparaît réellement entre deux demandes séparées par une attente, que le
programme se termine proprement.

Un test y marque une pause au milieu de la saisie : c'est la seule façon de
voir une valeur atteindre son délai dans un vrai dialogue.

---

## Où sont les vérifications

Chaque fichier de `src/` porte les siennes, sauf le point d'entrée qui ne
décide de rien.

| Pièce | Ce qui y est vérifié |
|---|---|
| le stockage | dépôt, relecture, retrait, péremption, comptage |
| la commande | ce qui est compris, ce qui est refusé, et pourquoi |
| la réponse | l'apparence de chacune des cinq natures |
| l'exécution | l'opération et la réponse de chaque demande |
| la session | l'enchaînement, les refus qui ne coupent pas, l'arrêt |
| le dialogue | la session complète, du dehors, programme lancé |
