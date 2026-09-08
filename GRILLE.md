# Grille de compréhension

Ce document n'est pas un parcours à suivre. C'est l'état de ce qui est compris.
Il ne contient jamais les questions : celles-ci viennent de l'utilisateur, à
l'oral, et tombent où elles tombent.

**Les trois états.**

| Marque | État | Ce qui le produit |
|---|---|---|
| `·` | non abordé | le point n'a jamais été touché |
| `~` | en cours d'acquisition | l'explication a été donnée, et le contrôle qui la suit est réussi |
| `x` | acquis | une question du même ordre, reposée plus tard, a obtenu une réponse juste du premier coup |

Réussir le contrôle qui suit une explication ne vaut jamais « acquis » :
l'explication venait d'être donnée. L'acquis se prouve à distance.

**Le rôle des couches.** Elles ne dictent pas l'ordre. Elles classent les points
par profondeur, pour repérer ce qui a été couvert, ce qui a été sauté, et ce qui
n'a jamais été touché.

**Quand la main est rendue.** Si l'utilisateur demande une question au lieu d'en
poser une, le point est choisi ici, du plus simple au plus complexe parmi les
non abordés et ceux à revoir. Il est alors introduit par une explication courte
avant toute question : on n'interroge pas sur un domaine encore inconnu.

**Mise à jour.** Après chaque échange : l'état du point touché, les points
acquis au passage, et les points nouveaux nés d'une question, marqués `(Q)`.

---

## Couche 1 — La vue globale

Ce qui se passe, à minima. Aucun fichier n'est nommé, aucune architecture n'est
abordée, aucune commande n'est citée, aucune forme de réponse n'est montrée.

- `~` 1.1 À quoi sert le projet, qui s'en sert, et qui répond
- `~` 1.2 Les étapes d'un échange, du texte tapé jusqu'à l'affichage
- `~` 1.3 Ce qui est gardé, où, et combien de temps
- `~` 1.4 Pourquoi rien n'est transmis : le client et le serveur sont la même
      session (Q) ← 1.1

## Couche 2 — Les fichiers

Le rôle tenu par chacun — superviser, faire une tâche, ranger — jamais
l'implémentation.

- `~` 2.1 Le rôle de chaque fichier, en une phrase ← 1.2
- `·` 2.2 Lequel supervise, et ce que superviser veut dire ici ← 2.1
- `·` 2.3 L'ordre dans lequel une ligne les traverse ← 1.2
- `~` 2.4 Quel fichier détient le stockage et à qui il le prête (Q) ← 1.3
- `~` 2.5 Pourquoi la traduction n'a pas besoin du stockage (Q) ← 2.3
- `·` 2.6 Ce qui est interdit dans le sens remontant ← 2.3

## Couche 3 — Ce que produit chaque échange

- `·` 3.1 Ce qu'est une commande, ce qu'est une réponse ← 1.2
- `·` 3.2 Les grandes catégories de ce qu'on peut demander ← 1.1
- `·` 3.3 Ce que renvoie une commande qui dépose, et une commande qui lit ← 3.2
- `·` 3.4 Ce que renvoient les commandes qui retirent et qui comptent ← 3.2
- `~` 3.5 Ce qui se passe quand la commande porte sur une clé absente ← 1.3
- `·` 3.6 Ce qui se passe quand la ligne tapée n'est pas comprise ← 3.1
- `~` 3.7 Le principe des valeurs qui ont un délai ← 1.3
- `·` 3.8 Les manières dont le programme peut s'arrêter ← 1.2
- `~` 3.9 Les cinq natures de réponse et ce qui produit chacune (Q) ← 3.1
- `~` 3.10 Pourquoi un texte et un nombre ne s'affichent pas pareil, alors que
      les deux peuvent être des chiffres (Q) ← 3.9

## Couche 4 — Les règles précises et leurs raisons

- `·` 4.1 Comment une ligne doit être écrite pour être comprise ← 3.6
- `·` 4.2 Les deux valeurs particulières du délai restant ← 3.7
- `·` 4.3 Le cas de départ d'un compteur ← 3.4
- `·` 4.4 Pourquoi un texte est entouré de guillemets, et ce que cela évite ← 3.9
- `·` 4.5 Pourquoi un délai nul ou négatif est refusé ← 3.7
- `x` 4.6 À quel moment une valeur périmée est retirée, et pourquoi cela ne se
      voit pas de l'extérieur ← 3.7
- `·` 4.7 Ce qu'un dépôt fait au délai déjà posé, et ce qu'une augmentation de
      compteur en fait ← 4.3
- `·` 4.8 Pourquoi la casse est ignorée sur la commande et conservée sur le
      reste ← 4.1
- `·` 4.9 Pourquoi un refus n'arrête pas le programme ← 3.6
- `~` 4.10 Comment la péremption est mesurée sans que rien ne tourne (Q) ← 4.6
- `~` 4.11 Les deux valeurs particulières de la durée restante, et pourquoi
      elles sont négatives (Q) ← 4.2
- `~` 4.12 Pourquoi chaque porte du stockage écarte le périmé elle-même, et ce
      qu'une seule omission produirait (Q) ← 4.6
- `~` 4.13 Ce que le nettoyage en fond garantirait, ce qu'il ne remplacerait
      pas, et pourquoi le projet s'en passe (Q) ← 4.12

## Couche 5 — L'organisation interne

- `·` 5.1 Ce que chaque partie décide, et ce qu'elle ne décide pas ← 2.1
- `·` 5.2 Ce que chaque partie reçoit et ce qu'elle rend ← 2.3
- `·` 5.3 Le sens des appels, et ce qui est interdit dans l'autre sens ← 5.2
- `·` 5.4 La seule partie qui peut refuser une ligne, et ce que cela garantit
      aux suivantes ← 3.6
- `~` 5.5 La seule partie qui modifie le stockage ← 4.7
- `·` 5.6 Ce que le stockage ignore, et ce que cela permet ← 5.3
- `·` 5.7 Pourquoi la boucle de session ne lit pas directement le clavier ← 5.1

## Couche 6 — La vérification

- `·` 6.1 Les deux niveaux de vérification et où ils se trouvent ← 2.1
- `·` 6.2 Ce que le niveau par partie ne peut pas voir ← 5.2
- `·` 6.3 Ce que le niveau complet attrape en plus ← 6.2
- `·` 6.4 Pourquoi une des vérifications marque une pause ← 4.6
- `·` 6.5 La partie qui ne porte aucune vérification, et pourquoi ← 2.1

---

## Journal

Ce que les réponses ont montré, point par point. Pas les questions, pas les
réponses : ce qui a coincé et ce qui est passé.

- **1.1, 1.2, 3.5, 1.4** — Vue d'ensemble reprise au format 2. L'ossature d'un
  échange est juste du premier coup, et la péremption constatée au moment où on
  regarde ressort spontanément (4.6 tient toujours). Deux erreurs : une
  transmission imaginée entre deux programmes distincts, avec redirection
  d'entrée et de sortie, alors qu'il n'y a qu'une seule session — d'où 1.4 ; et
  une étape de vérification des éléments de la commande placée avant
  l'exécution, qui rangeait encore la clé absente du côté du contrôle. Après
  correction, le contrôle sépare juste les deux cas : dans un cas le stockage
  est consulté et ne rend rien, dans l'autre la ligne s'arrête à la traduction.
  3.5 tenu.

- **4.11, 4.12, 4.13** — Nés d'une même suite de questions sur la péremption.
  4.11 : la forme de la réponse n'était pas connue, le fond l'était. 4.12 : la
  conséquence d'une omission a d'abord été comprise comme la disparition de la
  péremption, alors qu'elle produirait une incohérence entre commandes. 4.13 :
  après explication, la hiérarchie entre exactitude et occupation mémoire est
  reformulée juste, et rattachée au client unique. À reposer à distance.
- **4.10** — Travaillé. La supposition de départ plaçait le calcul au moment de
  la demande, avec mémoire de la demande précédente. Corrigé : la limite est
  calculée une seule fois au dépôt et ne bouge plus. Après le code montré en
  entier, la constitution de `perime_le` est reformulée juste. L'absence de
  limite reste à vérifier à distance.
- **1.2** — Reposé à distance. Ossature et ordre des étapes justes. A manqué :
  l'exécution ne se limite pas à vérifier une existence, et la boucle qui
  redemande une ligne après chaque réponse. Reste à revoir.
- **4.6** — Reposé à distance, réponse juste et complète du premier coup :
  la valeur périmée survit à l'attente et n'est retirée qu'au moment où une
  demande la regarde. **Acquis.**
- **1.1** — Le rôle du projet et les acteurs sont posés. A manqué : le serveur
  n'est pas qu'un validateur, il exécute la demande, et sa réponse rapporte un
  résultat, pas seulement un accord ou un refus. Second passage : la clé absente
  est encore décrite comme une erreur non fatale. À revoir.
- **1.2** — Les étapes sont vues. A coincé deux fois : le serveur contient le
  stockage, il ne le contacte pas ; et le « où » de la valeur.
- **1.3** — Vu. La durée jusqu'à l'arrêt du programme est acquise, l'endroit
  l'est moins.
- **2.1, 2.4, 2.5** — Nés de la demande de voir les fichiers et la circulation
  du stockage dans le programme. Tableau des fichiers donné, circulation
  expliquée, contrôles réussis.
- **3.9, 3.10** — Nés de la question sur les formes de réponse observées. Les
  cinq natures ont été montrées sur une sortie réelle.
- **3.5, 3.7, 4.6, 4.10, 5.5** — Vus au passage : clé absente contre erreur,
  retrait de la valeur périmée au moment où on la consulte, seule partie qui
  modifie le stockage.
- **Changement de méthode.** L'initiative passe à l'utilisateur : il pose les
  questions, l'explication suit, un contrôle immédiat suit l'explication. Les
  couches ne dictent plus l'ordre. Les états remplacent les cases cochées.

---

## Comptes rendus quotidiens

Écrit en fin de journée, quand l'utilisateur annonce qu'il arrête. Un bloc par
jour, le plus récent en haut. Il est destiné à la session du lendemain : il doit
suffire à savoir où en est la compréhension réelle sans relire le journal.

Chaque bloc dit, en quelques phrases :
- ce qui a été travaillé et ce qui a changé d'état ;
- ce qui tient réellement, avec ce qui le prouve ;
- ce qui résiste, et sous quelle forme l'erreur revient ;
- ce qu'il faut reprendre en priorité à la session suivante.

C'est une appréciation, pas un relevé. On y dit franchement si la progression
avance ou si un point stagne malgré plusieurs passages.

---

### 2026-09-07

**Travaillé.** La péremption, de bout en bout : ce qui est enregistré au dépôt,
quand le retrait a lieu, ce que répond une demande de durée restante, et
pourquoi aucun nettoyage ne tourne en fond. La méthode de travail a aussi été
refondue en cours de session — c'est l'utilisateur qui pose les questions
désormais.

**Ce qui tient.** Le moment du retrait est acquis, prouvé à distance : reposée
sans préparation, la question a reçu une réponse juste et complète du premier
coup — la valeur survit à l'attente et n'est retirée qu'au moment où une demande
la regarde. C'est le seul point acquis à ce jour, mais il est solide, et il a
servi d'appui à tout le reste de la session.

**Ce qui résiste.** Une même erreur revient sous deux formes. La clé absente est
encore décrite comme une erreur, alors que c'est une réponse normale à une
demande parfaitement exécutée. Et le rôle du serveur est spontanément réduit à
la validation d'une ligne, l'exécution passant au second plan. Les deux relèvent
de la couche 1 : le plan d'ensemble n'est pas encore net, alors que le détail de
la péremption, lui, passe bien.

**Une bonne surprise.** L'incohérence produite par une seule porte du stockage
qui omettrait d'écarter le périmé a été comprise vite, et la hiérarchie entre
exactitude des réponses et occupation mémoire a été reformulée correctement.
C'est un raisonnement d'architecture, plus profond que ce que la couche 1
demande.

**Priorité demain.** Reprendre la vue d'ensemble, pas le détail. Faire redire
les étapes d'un échange, et vérifier que la clé absente n'est plus rangée du
côté des refus. Ne pas replonger dans la péremption : elle est en avance sur le
reste.

**Appréciation.** La progression est réelle mais déséquilibrée. Les questions
posées descendent volontiers dans le mécanisme, et c'est là que la compréhension
est la meilleure. Ce qui manque est en amont : la phrase simple qui dit ce que
fait le programme. C'est classique, et cela se corrige en reposant les questions
larges, pas en expliquant davantage.
