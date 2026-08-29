mod dns_message;

use std::{error::Error, fs, path::PathBuf};

use bytes::Bytes;
use clap::{Args, Parser, Subcommand, ValueEnum};

use dns_message::DNSMessage;

#[derive(Parser)]
#[command(name = "dns-rs")]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DNSMessageSectionOption {
    Header,
    Question,
    All,
}
#[derive(Args)]
struct ReadArgs {
    #[arg(short, long)]
    section: DNSMessageSectionOption,

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
            let message = DNSMessage::new(Bytes::from_owner(packet_bytes))?;

            match args.section {
                DNSMessageSectionOption::Header => {
                    println!("{:?}", message.header);
                }
                DNSMessageSectionOption::Question => {
                    let questions = message.question_section.get_questions();
                    questions.iter().for_each(|question| {
                        let name: String = message
                            .labeler
                            .get_domain_name(&question.name_address)
                            .expect("Unable to parse domain name address.")
                            .iter()
                            .map(|label| {
                                String::from(*label) + "."
                            })
                            .collect();

                        let qtype = question.qtype;
                        let qclass = question.qclass;

                        println!("Domain Name: {name}\nQuery Type: {:?}\nQuery Class: {:?}", qtype, qclass)
                    });
                }
                DNSMessageSectionOption::All => {
                    todo!();
                }
            }
        }

        _ => todo!(),
    };

    Ok(())
}
