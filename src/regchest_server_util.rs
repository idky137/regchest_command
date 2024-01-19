// regchest_server_util.rs
// use: - server util functions for regchest_command
//      - regchest_listen: sets up tcp listener and calls any commands recieved using server_print and server_printout, returning response to client
// authers: idky137
//

use crate::regchest_command::CommandOutput;
use crate::regchest_command_util::{server_print, server_printout};
use std::io;
use std::io::{Read, Write};
use std::net::TcpListener;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingolib::lightclient::LightClient;

// - regchest_listen
// - sets up tcp listener and calls any commands recieved using server_printout, returning response to client
pub fn regchest_listen() {
    let mut regtest_manager_option: Option<RegtestManager> = None;
    let mut _cph: ChildProcessHandler;
    let mut faucet: Option<LightClient> = None;
    let mut recipient: Option<LightClient> = None;
    let mut com_response: String;

    // -setting up tcp listener:
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening at: 127.0.0.1.8080");
    print!(".. ");
    io::stdout().flush().expect("Error: failed to flush stdout");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                while let Ok(size) = stream.read(&mut buffer) {
                    if size == 0 {
                        break;
                    }
                    let command_str = String::from_utf8_lossy(&buffer[..size]);
                    if command_str == "endsession" {
                        println!("Client ended session, closing program");
                        std::process::exit(1);
                    }

                    // --- handling non scenario commands:
                    if let Some(regtest_manager) = &regtest_manager_option {
                        match (&recipient, &faucet) {
                            (Some(recipieent_value), Some(faucet_value)) => {
                                com_response = server_printout(
                                    &command_str
                                        .split_whitespace()
                                        .map(|s| s.to_string())
                                        .collect(),
                                    Some(regtest_manager),
                                    Some(recipieent_value),
                                    Some(faucet_value),
                                );
                            }
                            (Some(recipient_value), None) => {
                                com_response = server_printout(
                                    &command_str
                                        .split_whitespace()
                                        .map(|s| s.to_string())
                                        .collect(),
                                    Some(regtest_manager),
                                    Some(recipient_value),
                                    None,
                                );
                            }
                            (None, Some(faucet_value)) => {
                                com_response = server_printout(
                                    &command_str
                                        .split_whitespace()
                                        .map(|s| s.to_string())
                                        .collect(),
                                    Some(regtest_manager),
                                    None,
                                    Some(faucet_value),
                                );
                            }
                            (None, None) => {
                                panic!("error receiving lightclient");
                            }
                        }

                    // - handling scenario commands:
                    } else {
                        match server_print(
                            &command_str
                                .split_whitespace()
                                .map(|s| s.to_string())
                                .collect(),
                            None,
                            None,
                            None,
                        ) {
                            CommandOutput::FaucetRecipient(
                                regtest_manager_v,
                                cph_v,
                                faucet_v,
                                recipient_v,
                            ) => {
                                regtest_manager_option = Some(regtest_manager_v);
                                _cph = cph_v;
                                faucet = Some(faucet_v);
                                recipient = Some(recipient_v);
                            }
                            _ => {
                                panic!("Error: Incorrect output");
                            }
                        }
                        com_response = "scenario loaded.".to_string();
                    }

                    // - returning response to client:
                    let com_response_trim = com_response.as_str().trim();
                    stream
                        .write_all(com_response_trim.as_bytes())
                        .expect("Errror: Failed to send response");
                    let eor_flag = "\t\t";
                    stream
                        .write_all(eor_flag.as_bytes())
                        .expect("Error: failed to send eor_flag");
                    buffer = [0; 1024];
                }
                print!(".. ");
                io::stdout().flush().expect("Error: failed to flush stdout");
            }
            Err(e) => {
                eprintln!("Error: Failed to accept connection: {}", e);
            }
        }
    }
}
