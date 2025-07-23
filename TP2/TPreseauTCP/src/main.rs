use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

// Structure représentant un client connecté
struct Client {
    id: usize,
    stream: TcpStream,
}

fn handle_client(mut client: Client) {
    let mut buffer = [0; 512];
    // Lecture simple (bloquante)
    match client.stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                // On renvoie ce qu'on a reçu (echo)
                let _ = client.stream.write(&buffer[..bytes_read]);
            }
        }
        Err(_) => {
            // Erreur de lecture, on ignore
        }
    }
    // La connexion se ferme à la fin de la fonction
}

fn main() {
    // Création du serveur TCP sur le port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Impossible de démarrer le serveur");
    println!("Serveur TCP en écoute sur 127.0.0.1:7878");

    // Liste partagée des clients connectés (Mutex pour accès concurrent)
    let clients = Arc::new(Mutex::new(Vec::new()));
    let mut id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                id_counter += 1;
                let client = Client {
                    id: id_counter,
                    stream: stream.try_clone().expect("Clone stream"),
                };

                // Ajout du client à la liste (ownership/membership)
                let clients_arc = Arc::clone(&clients);
                {
                    let mut clients_lock = clients_arc.lock().unwrap();
                    clients_lock.push(Client {
                        id: client.id,
                        stream: stream.try_clone().unwrap(),
                    });
                }

                // On gère le client dans un thread séparé
                thread::spawn(move || {
                    handle_client(client);
                });
            }
            Err(_) => {
                // Erreur de connexion, on ignore
            }
        }
    }
}
