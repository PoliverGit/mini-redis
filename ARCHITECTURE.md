# Architecture

Ce document décrit les parties du projet, une par une, et ce qu'elles
s'échangent.
Il se lit après les contraintes. Il ne parle pas de code.

---

## Le principe

Une demande tapée au clavier traverse le projet **en ligne droite**, et chaque
partie ne fait qu'une seule chose dessus :

```
   clavier
      │
      ▼
  la séance      ── lit une ligne, affiche l'invite et la réponse
      │
      ▼
  la commande    ── comprend la ligne, ou la refuse
      │
      ▼
  l'exécution    ── fait ce qui est demandé
      │
      ▼
   le stockage    ── range et rend les valeurs
      │
      ▼
   la réponse    ── met en forme ce qu'on renverra
      │
      ▼
   écran
```

Le trait important est **le sens des flèches**. Le stockage n'appelle jamais la
séance. La commande n'appelle jamais le stockage. Chaque partie n'appelle que
celles qui sont sous elle.

---

## L'arborescence

```
mini-redis/
├── Cargo.toml            le nom, la version et les dépendances du projet
├── CLAUDE.md             les règles de travail
├── CONTRAINTES.md        ce que le projet doit faire
├── ARCHITECTURE.md       ce document
├── src/
│   ├── main.rs           le point d'entrée
│   ├── session.rs        la séance
│   ├── command.rs        la commande
│   ├── execute.rs        l'exécution
│   ├── store.rs          le stockage
│   └── reply.rs          la réponse
└── tests/
    └── dialogue.rs       le dialogue complet, vu de l'extérieur
```

---

## Les parties, une par une

### `src/store.rs` — le stockage

**Son rôle.** Ranger des valeurs sous des clés, et les rendre quand on les
demande. Il tient aussi la date de péremption de celles qui en ont une.

**Ce qu'il sait faire.** Déposer une valeur. Déposer une valeur avec un délai.
Rendre une valeur. Retirer une clé. Dire si une clé existe. Dire le délai
restant sur une clé. Se vider. Se compter.

Il sait aussi remplacer une valeur **en lui laissant son délai** — c'est ce dont
un compteur a besoin pour que son délai ne soit pas remis à zéro à chaque
augmentation.

**Ce qu'il ignore.** Tout le reste. Il ne fait référence ni au clavier, ni aux
commandes, ni à l'affichage. On pourrait le reprendre tel quel dans un autre
projet.

**Le point délicat.** C'est lui qui décide qu'une valeur est périmée, et il le
décide au moment où on la regarde. Une valeur périmée est retirée à cet
instant-là. Personne d'autre n'a à s'en soucier : pour toutes les parties
au-dessus, elle est simplement absente.

---

### `src/command.rs` — la commande

**Son rôle.** Traduire une ligne de texte en demande claire, ou la refuser en
disant pourquoi.

**Ce qu'il sait faire.** Découper la ligne en mots, en tenant compte des
guillemets. Reconnaître le nom de la demande sans se soucier de la casse.
Vérifier que le nombre de mots correspond. Lire un délai de vie.

**Ce qu'il ignore.** Ce que la demande produira. Il ne touche pas au stockage. Il
ne sait même pas qu'un stockage existe.

**Le point délicat.** C'est la seule partie qui peut refuser une ligne. Toutes
les demandes qui en sortent sont valides : les parties suivantes n'ont plus à
vérifier que la ligne avait un sens.

Il distingue trois issues, pas deux : une demande comprise, un refus motivé, et
**une ligne vide, qui n'est ni l'un ni l'autre**. Une ligne vide n'appelle
aucune réponse.

---

### `src/reply.rs` — la réponse

**Son rôle.** Dire ce que le serveur renvoie, et sous quelle forme cela
s'affiche.

**Ce qu'il sait faire.** Distinguer les cinq natures de réponse : une
confirmation, un texte, une absence, un nombre, un refus. Et donner à chacune
son apparence à l'écran.

**Ce qu'il ignore.** D'où vient la réponse. Il ne décide rien : il met en forme
ce qu'on lui donne.

**Le point délicat.** C'est lui qui entoure les textes de guillemets, et qui
protège les guillemets contenus dans le texte lui-même. Sans ça, on ne
distinguerait pas un texte vide d'une absence de valeur.

---

### `src/execute.rs` — l'exécution

**Son rôle.** Prendre une demande comprise, la faire dans le stockage, et rendre
la réponse correspondante.

**Ce qu'il sait faire.** Pour chaque demande, l'opération qui lui correspond et
la réponse qui va avec. C'est ici qu'on décide qu'une clé absente vaut `-2` pour
le délai restant, qu'un compteur absent part de zéro, qu'une seconde
entamée compte pour une seconde entière.

**Ce qu'il ignore.** Comment la demande est arrivée, et comment la réponse sera
affichée.

**Le point délicat.** C'est **la seule partie qui modifie le stockage**. Toutes
les autres comprennent, rangent ou mettent en forme. Quand un résultat est
inattendu, c'est ici qu'on regarde en premier.

---

### `src/session.rs` — la séance

**Son rôle.** Tenir le fil du dialogue : afficher l'invite, lire une ligne, la
faire comprendre, la faire exécuter, écrire la réponse, recommencer.

**Ce qu'il sait faire.** Boucler. Décider quand s'arrêter : quand on le lui
demande, ou quand il n'y a plus rien à lire. Faire en sorte qu'un refus
n'interrompe pas la séance.

**Ce qu'il ignore.** Le détail de chaque demande. Une seule l'intéresse, la
demande d'arrêt, parce que c'est elle qui termine la boucle.

**Le point délicat.** Elle ne lit pas directement le clavier et n'écrit pas
directement à l'écran : **on lui donne une source de lignes et une destination
pour ce qu'elle écrit.** C'est ce qui permet de rejouer une séance entière en
vérification, en lui donnant du texte préparé et en relisant tout ce qu'elle a
produit. Sans ça, la boucle serait la seule partie impossible à vérifier.

---

### `src/main.rs` — le point d'entrée

**Son rôle.** Monter les parties et lancer la séance sur le vrai clavier et le
vrai écran.

**Ce qu'il sait faire.** Créer le stockage vide, brancher le clavier et l'écran,
lancer la séance, et signaler l'échec si l'écran devient inutilisable.

**Ce qu'il ignore.** Absolument tout le reste. C'est volontairement la partie la
plus courte du projet : elle ne contient aucune décision, donc aucun test.

---

### `tests/dialogue.rs` — le dialogue vu de l'extérieur

**Son rôle.** Vérifier le projet comme le ferait quelqu'un qui ne connaît aucune
de ses parties.

**Ce qu'il fait.** Il lance le programme, lui envoie des lignes comme si on les
tapait, attend qu'il se termine, et compare **tout** ce qui s'est affiché à ce
qui devait s'afficher — invites comprises.

**Pourquoi il existe.** Les vérifications faites dans chaque fichier regardent
une partie isolée. Elles ne peuvent rien dire de l'enchaînement complet : que le
stockage est bien le même du début à la fin de la séance, qu'un refus ne coupe
pas le fil, qu'une valeur périmée disparaît réellement entre deux demandes
séparées par une attente, que le programme se termine proprement.

Un test y marque une pause au milieu de la saisie : c'est la seule façon de
voir une valeur atteindre son délai dans un vrai dialogue.

---

## Où sont les vérifications

Chaque fichier de `src/` porte ses propres vérifications, en dessous de ce
qu'il décrit, à l'exception du point d'entrée qui ne décide de rien.

| Pièce | Ce qui y est vérifié |
|---|---|
| le stockage | dépôt, relecture, retrait, péremption, comptage |
| la commande | ce qui est compris, ce qui est refusé, et pourquoi |
| la réponse | l'apparence de chacune des cinq natures |
| l'exécution | l'opération et la réponse de chaque demande |
| la séance | l'enchaînement, les refus qui ne coupent pas, l'arrêt |
| le dialogue | la séance complète, du dehors, programme lancé |
