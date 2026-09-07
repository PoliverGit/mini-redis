# mini-redis

## Le projet

Un stockage de valeurs en mémoire, interrogé au clavier.

On lance le programme, une invite `>` s'affiche, on tape une ligne, le
programme répond, et ainsi de suite jusqu'à ce qu'on lui demande de s'arrêter.
Tout ce qui est rangé ne dure que le temps de la séance : quand le programme se
termine, le stockage disparaît.

Le travail est découpé en cinq rôles :

| Fichier | Rôle |
|---|---|
| `src/main.rs` | Monte les parties et lance la séance sur le clavier et l'écran. |
| `src/session.rs` | Tient le fil du dialogue : invite, lecture, réponse, recommencer. |
| `src/command.rs` | Traduit une ligne tapée en demande claire, ou la refuse en expliquant pourquoi. |
| `src/execute.rs` | Fait ce que la demande réclame, en s'adressant au stockage. |
| `src/store.rs` | Range les valeurs sous leur clé, avec leur éventuelle date de péremption. |
| `src/reply.rs` | Met en forme la réponse affichée. Ne décide de rien. |

Documents de conception : [CONTRAINTES.md](CONTRAINTES.md) et
[ARCHITECTURE.md](ARCHITECTURE.md).

## Les commandes reconnues

| Commande | Effet |
|---|---|
| `PING` | Vérifie que le serveur répond. Réponse `"PONG"`. |
| `PING message` | Vérifie en plus que le message envoyé revient intact. |
| `SET cle valeur` | Range une valeur. |
| `SET cle valeur EX n` | Range une valeur pour `n` secondes. |
| `SET cle valeur PX n` | Range une valeur pour `n` millisecondes. |
| `GET cle` | Lit une valeur. |
| `DEL key1 key2` | Retire une ou plusieurs clés. |
| `EXISTS cle...` | Compte celles qui existent parmi les clés demandées. |
| `TTL cle` | Dit le délai restant sur une clé, en secondes. `-1` : la clé est là sans limite de temps. `-2` : la clé n'existe pas. |
| `INCR cle` | Augmente de un une valeur entière. |
| `DBSIZE` | Compte les clés du stockage. |
| `FLUSHALL` | Vide le stockage. |
| `QUIT` / `EXIT` | Termine la séance. |

## Tester

Lancer toute la batterie de tests :

```bash
cargo test
```

Ne lancer que les tests d'une partie :

```bash
cargo test --lib store
```

Ne lancer que le dialogue complet, vu de l'extérieur :

```bash
cargo test --test dialogue
```

Voir le détail test par test :

```bash
cargo test -- --nocapture --test-threads=1
```

Vérifier que le projet compile sans le lancer :

```bash
cargo check
```

## Essayer à la main

```bash
cargo run
```

Puis taper :

```
PING
SET pseudo paul
GET pseudo
TTL pseudo
SET jeton abc EX 2
DBSIZE
DEL pseudo
EXISTS pseudo jeton
FLUSHALL
QUIT
```

Donner une suite de lignes d'un seul coup, sans rien taper :

```bash
printf 'SET a 1\nINCR a\nGET a\nQUIT\n' | cargo run
```
