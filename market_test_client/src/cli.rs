use anyhow::Result;
use rustyline::error::ReadlineError;
use std::net::Ipv4Addr;

use clap::Parser;
use rustyline::DefaultEditor;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    actor::{Command, Message},
    ActorMarketState, ActorMarketStateLockCond,
};
pub const LOOPBACK_ADDR: &str = "127.0.0.1";
pub const DEFAULT_MARKET_SERVER_PORT: &str = "8080";

pub type Port = u16;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Port where the local market server is listening on and the client should connect to
    // NOTE: Essentially, this is just a port to communicate with the market server over an IPC
    // mechanism like TCP sockets between two processes.
    #[arg(short, long, default_value = DEFAULT_MARKET_SERVER_PORT)]
    pub market_port: Port,
    /// Username of the user registering the file
    // NOTE: probably to be removed since we already have unique SHA256 peerIDs?
    #[arg(short, long)]
    pub username: String,
    /// The price of the file per MB
    // NOTE: protobuf writers set this to be i64
    #[arg(short, long)]
    pub price: u64,
    /// The ID of the peer. If not provided, then it is automatically generated
    #[arg(short, long)]
    pub id: Option<String>,
    /// Port where other consumer peer clients should connect to retrieve files
    #[arg(long)]
    pub client_port: Port,
    /// IP where other consumer peer clients should connect to retrieve files
    #[arg(long)]
    pub client_ip: Ipv4Addr,
}

const PROMPT: &str = ">> ";

pub fn start_main_loop(
    tx: UnboundedSender<Command>,
    lock_cond: ActorMarketStateLockCond,
) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let (lock, cvar) = &*lock_cond;
    let mut state = lock.lock().unwrap();
    loop {
        match *state {
            ActorMarketState::NotConnected => {
                state = cvar.wait(state).unwrap();
                continue;
            }
            ActorMarketState::FailedToConnect => {
                drop(state);
                return Err(anyhow::anyhow!("Failed to find a command executor"));
            }
            ActorMarketState::Connected => {
                drop(state);
                break;
            }
        };
    }
    loop {
        let line = rl.readline(PROMPT);
        match line {
            Ok(line) => {
                let msg = Message::new(line);
                match msg.into_command() {
                    Ok(cmd) => {
                        // bails when the receiver is dropped
                        if let Command::Quit = cmd {
                            println!("Quitting...");
                            tx.send(cmd)?;
                            break;
                        } else {
                            tx.send(cmd)?;
                        }
                    }
                    Err(err) => {
                        eprintln!("Error parsing command: {}", err);
                    }
                }
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                let _ = tx.send(Command::Quit);
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                let _ = tx.send(Command::Quit);
                break;
            }
        }
    }
    Ok(())
}
