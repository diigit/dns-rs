mod message;

use std::{error::Error, fs, path::PathBuf};

use bytes::Bytes;
use clap::{Args, Parser, Subcommand, ValueEnum};

use message::Message;

#[derive(Parser)]
#[command(name = "dns-rs")]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MessageSectionOption {
    Header,
    Question,
    All,
}
#[derive(Args)]
struct ReadArgs {
    #[arg(short, long)]
    section: MessageSectionOption,

    file: PathBuf,
}

// TODO
#[derive(Args)]
struct WriteArgs {}

#[derive(Subcommand)]
enum Commands {
    Read(ReadArgs),
    Write(WriteArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read(args) => {
            let packet_bytes = fs::read(args.file)?;
            let message = Message::new(Bytes::from_owner(packet_bytes))?;

            match args.section {
                MessageSectionOption::Header => {
                    println!("{:?}", message.header);
                }
                MessageSectionOption::Question => {
                    let questions = message.question_section.get_iter(&message.labeler);
                    questions.for_each(|question| {
                        let name: String = question
                            .name
                            .iter()
                            .map(|label| String::from(*label) + ".")
                            .collect();

                        let qtype = question.qtype;
                        let qclass = question.qclass;

                        println!(
                            "Domain Name: {name}\nQuery Type: {:?}\nQuery Class: {:?}",
                            qtype, qclass
                        )
                    });
                }
                MessageSectionOption::All => {
                    todo!();
                }
            }
        }

        _ => todo!(),
    };

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_query_msg() -> message::Message {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/query_packet.txt");

        Message::new(Bytes::from_owner(fs::read(path).unwrap())).unwrap()
    }

    pub fn get_response_msg() -> message::Message {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/response_packet.txt");

        Message::new(Bytes::from_owner(fs::read(path).unwrap())).unwrap()
    }
}
