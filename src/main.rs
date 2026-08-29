mod dns_message;

use std::{error::Error, fs};

use bytes::{Buf, Bytes};

use dns_message::DNSMessage;


fn main() -> Result<(), Box<dyn Error>> {
    let path_arg = std::env::args_os()
        .nth(1)
        .expect("Please provide a valid path");

    let packet_bytes = fs::read(path_arg)?;

    let message = DNSMessage::new(Bytes::from_owner(packet_bytes))?;

    let a = message.question_section.get_questions();
    let question = &a[0];
    let name = message
        .labeler
        .get_domain_name(&question.name_address)
        .unwrap();

    println!("{:?}", name);

    Ok(())
}
