// TP2 Reseau TCP
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

// Structure représentant un client connecté
#[derive(Debug)]
struct ClientInfo {
    id: usize,
    addr: String,
}

fn handle_client(mut stream: TcpStream, client_id: usize, clients: Arc<Mutex<Vec<ClientInfo>>>) {
    let client_addr = stream.peer_addr().unwrap().to_string();
    println!("Client {} connecté depuis {}", client_id, client_addr);
    
    // Ajouter le client à la liste
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push(ClientInfo {
            id: client_id,
            addr: client_addr.clone(),
        });
        println!("Client {} ajouté à la liste", client_id);
        afficher_clients(&clients);
    }
    
    let mut buffer = [0; 512];
    
    // Boucle de lecture continue pour maintenir la connexion
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Le client a fermé la connexion
                println!("Client {} a fermé la connexion", client_id);
                break;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Client {} dit: {}", client_id, message.trim());
                
                // Echo du message vers le client
                if let Err(_) = stream.write_all(&buffer[..bytes_read]) {
                    println!("Erreur d'écriture vers le client {}", client_id);
                    break;
                }
            }
            Err(_) => {
                println!("Erreur de lecture du client {}", client_id);
                break;
            }
        }
    }
    
    // Retirer le client de la liste à la déconnexion
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.retain(|c| c.id != client_id);
        println!("Client {} retiré de la liste", client_id);
        afficher_clients(&clients);
    }
}

fn afficher_clients(clients: &Arc<Mutex<Vec<ClientInfo>>>) {
    let clients_lock = clients.lock().unwrap();
    println!("=== Liste des clients connectés ({}) ===", clients_lock.len());
    if clients_lock.is_empty() {
        println!("Aucun client connecté");
    } else {
        for client in clients_lock.iter() {
            println!("- Client id: {} depuis {}", client.id, client.addr);
        }
    }
    println!("=======================================");
}

fn main() {
    // Création du serveur TCP sur le port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Impossible de démarrer le serveur");
    println!("Serveur TCP en écoute sur 127.0.0.1:7878");
    println!("Testez avec: telnet 127.0.0.1 7878");

    // Liste partagée des clients connectés (Mutex pour accès concurrent)
    let clients = Arc::new(Mutex::new(Vec::new()));
    let mut id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                id_counter += 1;
                let clients_arc = Arc::clone(&clients);
                
                // Créer un thread pour gérer chaque client
                thread::spawn(move || {
                    handle_client(stream, id_counter, clients_arc);
                });
            }
            Err(e) => {
                println!("Erreur de connexion: {}", e);
            }
        }
    }
}
