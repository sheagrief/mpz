// This example demonstrates how to securely and privately transfer data using OT extension.
// In practical situations data would be communicated over a channel such as TCP.
// For simplicity, this example shows how to use OT components in memory.

use mpz_core::Block;
use mpz_ot_core::chou_orlandi::{Receiver, Sender};

pub fn main() {
    // Receiver choice bits
    let choices = vec![false, true, false, false, true, true, false, true];

    println!("Receiver choices: {:?}", &choices);

    // Sender messages the receiver chooses from
    let inputs = [
        [Block::new(0), Block::new(1)],
        [Block::new(2), Block::new(3)],
        [Block::new(4), Block::new(5)],
        [Block::new(6), Block::new(7)],
        [Block::new(8), Block::new(9)],
        [Block::new(10), Block::new(11)],
        [Block::new(12), Block::new(13)],
        [Block::new(14), Block::new(15)],
    ];

    println!("Sender inputs: {:?}", &inputs);

    // First the sender creates a setup message and passes it to receiver
    let (sender_setup, sender) = Sender::default().setup();

    // Receiver takes sender's setup and creates its own setup message, and
    // generates the receiver payload
    let (receiver_setup, mut receiver) = Receiver::default().setup(sender_setup);
    let receiver_payload = receiver.receive_random(&choices);

    // Finally, sender encrypts their inputs and sends them to receiver
    let mut sender = sender.receive_setup(receiver_setup).unwrap();
    let sender_payload = sender.send(&inputs, receiver_payload).unwrap();

    // Receiver takes the encrypted inputs and is able to decrypt according to their choice bits
    let received = receiver.receive(sender_payload).unwrap();

    println!("Transferred messages: {:?}", received);
}