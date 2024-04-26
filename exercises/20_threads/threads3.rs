// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
        // Création d'une instance d'Arc pour partager la file d'attente entre les threads
    let qc = Arc::new(q);
        // Clonage de l'Arc pour obtenir deux références à la file d'attente
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
        // Clonage du canal d'envoi pour permettre à chaque thread d'envoyer des valeurs
    let tx1 = tx.clone();

    thread::spawn(move || {
                // Création du premier thread pour envoyer les valeurs de first_half
        for val in &qc1.first_half {
            println!("sending {:?}", val);
                // Envoi de la valeur via le canal d'envoi
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
        // Création du deuxième thread pour envoyer les valeurs de second_half
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
                        // Envoi de la valeur via le canal d'envoi
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
