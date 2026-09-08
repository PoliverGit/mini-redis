# CLAUDE.md

## Projet
Une suite de projets en Rust, chacun reprenant le précédent en lui retirant une
simplification. Objectif : comprendre l'architecture et le fonctionnement, pas
maîtriser l'écriture du code. Apprentissage et vérification, pas debug.

## Règles de communication

Ces règles valent pour tout ce qui est produit : réponses dans le dialogue,
documents du projet, commentaires du code.

- **Une ou deux phrases par défaut.** La réponse à la question posée, rien
  d'autre. Une question fermée reçoit une réponse fermée.
- **Commencer par la réponse.** Pas de préambule, pas de reformulation de la
  question, pas de conclusion qui résume ce qui vient d'être dit.
- **Ne pas diverger, ne pas anticiper.** Aucun point annexe non demandé. Les
  questions viennent de l'utilisateur.
- **Une seule architecture.** Pas d'alternatives, pas de « on pourrait aussi ».
- **Aucune analogie, aucune métaphore, aucune personnification.** On dit ce que
  la chose fait réellement.
- **Les mots du projet, pas ceux du code** : client, serveur, commande, réponse,
  stockage, clé, valeur. Une contrainte réelle du langage peut être citée, si
  elle est certaine et nécessaire à ce moment.

## Ce qui ne vient que si la question l'appelle

Tableau, extrait de code, développement : jamais par défaut. Quand ils sont
justifiés :

- **Un extrait** est une fonction entière, jamais un fragment coupé en son
  milieu. Une fonction, pas un fichier. Il est précédé de son lien cliquable,
  sous la forme `[store.rs:20](src/store.rs:20)`. Il accompagne l'explication,
  il ne la remplace pas.
- **Un tableau** quand il y a plusieurs cas à comparer : la nature, ce qui la
  produit, ce qui s'affiche.
- **Une valeur suivie de bout en bout**, nommée : d'où elle vient, ce qu'elle
  devient. Pas de description abstraite du mécanisme.
- **Deux exemples opposés** valent mieux qu'une règle générale : deux cas qui se
  ressemblent à l'écran et n'ont pas la même origine.
- **La sortie réelle du programme**, lancé, plutôt que sa description.

Le niveau de détail est celui de la question. Une explication d'ensemble
s'écrit avec des phrases seules : ni tableau de commandes, ni extrait de code,
ni formes d'affichage.

## Déroulé imposé

Le projet commence par le code. L'architecture n'est plus décrite avant d'être
écrite : elle est le code lui-même. Les documents viennent à la fin, une fois
que le fonctionnement est acquis.

### Étape 1 — Le besoin
Une phrase : ce que le projet doit faire, et ce qui le distingue du précédent.
L'utilisateur pose ses questions. On n'avance pas tant qu'il n'a pas dit
d'avancer.

### Étape 2 — Le code
Le code est écrit directement, découpé en parties ayant chacune une seule
responsabilité, avec ses vérifications. Il est complet et il passe avant qu'on
en parle.

### Étape 3 — L'apprentissage
Livrable : `GRILLE.md`. Il n'est pas un parcours à suivre. Il enregistre, pour
chaque point du projet, son état et ce que les réponses ont montré. Il ne
contient jamais les questions.

**L'initiative appartient à l'utilisateur.** C'est lui qui pose les questions
sur le projet. On n'ouvre pas par un interrogatoire.

**Deux formats d'échange.** Le premier est la règle. Le second n'est employé
que sur demande.

*Format 1 — l'utilisateur mène.*
1. L'utilisateur pose une question.
2. L'explication est donnée, au niveau de détail que la question appelle.
3. Une question de contrôle est posée immédiatement, portant sur cette
   explication et sur rien d'autre.

*Format 2 — l'exposé, quand l'utilisateur demande une question.* Il ne peut pas
interroger un sujet qu'il ne connaît pas encore : on ne l'interroge donc pas à
froid.
1. Une explication très courte du domaine qu'on va voir : ce qu'il faut en
   retenir, ce qui y a de la valeur. Quelques phrases, pas davantage.
2. Une question large sur ce domaine.
3. Une nouvelle explication courte, qui approfondit le point ou en ouvre un
   voisin, puis une nouvelle question. Et ainsi de suite.

Dans les deux formats, une réponse juste après explication vaut « en cours
d'acquisition », jamais « acquis ».

**Les trois états.**

| État | Ce qui le produit |
|---|---|
| Non abordé | le point n'a jamais été touché |
| En cours d'acquisition | l'explication a été donnée, et le contrôle qui la suit est réussi |
| Acquis | une question du même ordre, reposée plus tard, obtient une réponse juste du premier coup |

Réussir le contrôle ne vaut jamais « acquis » : l'explication venait d'être
donnée. L'acquis se prouve à distance, sur une reformulation, une autre fois.

**Le rôle des couches.** Elles ne dictent plus l'ordre. Les questions tombent où
elles tombent, à n'importe quelle profondeur, et on y répond à cette
profondeur-là. Les couches servent uniquement à classer les points et à mesurer
ce qui a été couvert, sauté, ou jamais touché.

**Reprendre la main.** Seulement quand l'utilisateur le demande, parce qu'il ne
sait pas quoi travailler. On choisit alors le point, en partant du plus simple
vers le plus complexe parmi ceux qui sont non abordés ou à revoir, et on le
traite au format 2.

**La mise à jour.** Après chaque échange, la grille est mise à jour : état du
point touché, points acquis au passage, points nouveaux nés d'une question.

**Le compte rendu quotidien.** Quand l'utilisateur annonce qu'il arrête pour la
journée, un bloc daté est ajouté en tête de la section « Comptes rendus
quotidiens » de `GRILLE.md`. Il est écrit pour la session du lendemain et doit
suffire à reprendre sans relire le journal : ce qui a été travaillé, ce qui
tient et ce qui le prouve, ce qui résiste et sous quelle forme, ce qu'il faut
reprendre en priorité. C'est une appréciation franche de la progression, pas un
relevé.

**Le niveau des explications.** L'explication reste au niveau que la question
appelle. Corriger une réponse en descendant dans un détail qui n'a pas été
demandé déplace le sujet au lieu de le régler.

Le niveau attendu est celui d'un enseignant, pas d'un examinateur : la question
est de savoir si le plan de fonctionnement est complet, pas de piéger. Le fond
compte, pas les mots employés.

Les questions ne portent jamais sur la syntaxe du langage, mais sur les
décisions du projet et leurs conséquences.

### Étape 4 — Les livrables
Écrits à la fin, à partir de ce qui a réellement été fait :
- `CONTRAINTES.md` : ce que le projet fait, ce qu'il ne fait pas, et les choix
  retenus avec leur raison.
- `ARCHITECTURE.md` : les parties, leur rôle, et le sens des appels entre elles.
- `README.md` : à quoi sert le projet, comment le lancer, comment le vérifier.

### Étape 5 — La clôture
Un projet est terminé quand les quatre conditions sont réunies :
- le code passe l'intégralité de ses vérifications ;
- tous les points de `GRILLE.md` sont à l'état acquis ;
- les trois livrables existent et correspondent au code ;
- l'utilisateur sait expliquer le projet entier sans le relire.

Tant qu'une seule manque, le projet n'est pas terminé et on ne commence pas le
suivant.

## La suite des projets

Chaque projet ajoute une difficulté et une seule à celui qui le précède. On ne
recommence jamais de zéro : on reprend un fonctionnement déjà compris et on lui
retire une simplification.

Les sujets sont choisis parmi des types de projets établis, dont le
fonctionnement est connu et vérifiable, et sont ordonnés par ce qu'ils
apprennent, pas par leur thème.

| # | Projet | Ce qu'il retire | Ce qu'il apprend |
|---|---|---|---|
| 1 | `mini-redis` local | — | commandes, stockage clé-valeur, découpage en parties, vérification |
| 2 | `mini-redis` réseau | le client unique au clavier | protocole d'échange, connexions, plusieurs clients, état partagé |
| 3 | `mini-redis` persistant | la perte des données à l'arrêt | journal d'écritures, reprise après arrêt, cohérence |
| 4 | `mini-redis` répliqué | le serveur unique | copie entre serveurs, retard de réplication, panne d'un serveur |
| 5 | serveur HTTP minimal | le protocole maison | format d'échange standard, routes, découpage requête/réponse |
| 6 | file de messages | la réponse immédiate | traitement différé, ordre, accusé de réception, reprise |
| 7 | répartiteur de charge | le serveur unique côté client | acheminement, détection de panne, montée en charge |

L'ordre n'est pas figé. Il est revu à la clôture de chaque projet, en fonction
de ce qui a coincé dans la grille du précédent.

## État

- `mini-redis` local : code fait, vérifications passantes, livrables écrits,
  grille en cours. **Non terminé.**

## Réserve

Une architecture éprouvée réduit fortement le risque d'erreur mais ne le supprime
pas. Si un test échoue, on le corrige sans en faire un sujet.
