# TP : Serveur TCP Rust - Gestion de clients

## Objectif
Créer un serveur TCP simple en Rust qui :
- Accepte plusieurs connexions clients
- Garde une liste des clients connectés (structure Client)
- Applique les notions d'ownership et de membership
- Utilise uniquement les bibliothèques standard demandées

## Notions abordées
- **TCP** : communication réseau bas niveau (std::net::TcpListener, TcpStream)
- **Ownership** : chaque client appartient à la liste, chaque thread possède son client
- **Membership** : la structure Client est membre de la liste des clients
- **Threads** : chaque client est géré dans un thread séparé
- **Mutex/Arc** : pour partager la liste des clients entre threads
- **Lecture/Écriture** : std::io::{Read, Write} pour lire/écrire sur le réseau

## Structure principale
```rust
struct Client {
    id: usize,
    stream: TcpStream,
}
```

## Fonctionnement du serveur
- Le serveur écoute sur 127.0.0.1:7878
- À chaque connexion, il crée un Client, l'ajoute à la liste partagée, et lance un thread pour gérer la communication
- Le thread lit un message du client et le renvoie (echo)
- La connexion se ferme à la fin du thread

## Lancer le serveur
Dans le dossier du projet :
```bash
cargo run
```

## Tester le serveur
Dans un autre terminal :
```bash
telnet 127.0.0.1 7878
# ou
nc 127.0.0.1 7878
```
Tapez un message, il sera renvoyé (echo).
