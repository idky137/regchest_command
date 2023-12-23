// regchest_command_server main.rs
// use: gives access to zingo lib functions over tcp to create custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use tokio::{runtime::Runtime, task};
use zingo_testutils::scenarios;
use zingolib::{
    commands::{self, do_user_command},
    lightclient::LightClient,
};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[derive(Debug)]
pub struct CommandData {
    com_name: &'static str,
    com_type: &'static str,
    com_req_inputs: &'static usize,
    com_opt_inputs: &'static usize,
}

pub trait CommandExec<I, O> {
    fn exec(&self, com_nametype: &str, com_inputs: I, com_options: Vec<&str>) -> O;
    fn get_command_data(&self) -> &CommandData;
}

// --- command_lib
// --- library of available functions

pub fn command_lib() -> HashMap<&'static str, Box<dyn CommandExec<CommandInput, CommandOutput>>> {
    #[allow(unused_mut)]
    let mut entries: Vec<(
        &'static str,
        Box<dyn CommandExec<CommandInput, CommandOutput>>,
    )> = vec![
        (
            "do_user_command",
            Box::new(DoUserCommand {
                command_data: CommandData {
                    com_name: "do_user_command",
                    com_type: "do_user_command",
                    com_req_inputs: &3,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::unfunded_client",
            Box::new(UnfundedClient {
                command_data: CommandData {
                    com_name: "unfunded_client",
                    com_type: "scenarios",
                    com_req_inputs: &1,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet",
            Box::new(Faucet {
                command_data: CommandData {
                    com_name: "faucet",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_recipient",
            Box::new(FaucetRecipient {
                command_data: CommandData {
                    com_name: "faucet_recipient",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_funded_recipient",
            Box::new(FaucetFundedRecipient {
                command_data: CommandData {
                    com_name: "faucet_funded_recipient",
                    com_type: "scenarios",
                    com_req_inputs: &5,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "generate_n_blocks_return_new_height",
            Box::new(GenerateNBlocksReturnNewHeight {
                command_data: CommandData {
                    com_name: "generate_n_blocks_return_new_height",
                    com_type: "generate_n_blocks_return_new_height",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_recipient_default",
            Box::new(FaucetRecipientDefault {
                command_data: CommandData {
                    com_name: "faucet_recipient_default",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &2,
                },
            }),
        ),
    ];
    entries.into_iter().collect()
}

pub enum CommandInput {
    DoUserCommand((String, Vec<String>, LightClient)),
    UnfundedClient((String, String)),
    Faucet((String, String)),
    FaucetRecipient((String, String)),
    FaucetFundedRecipient((String, String)),
    GenerateNBlocksReturnNewHeight((String, String)),
    FaucetRecipientDefault((String, String)),
}

pub enum CommandOutput {
    DoUserCommand(String),
    UnfundedClient(String),
    Faucet(String),
    FaucetRecipient(String),
    FaucetFundedRecipient(String),
    GenerateNBlocksReturnNewHeight(String),
    FaucetRecipientDefault(String),
}

struct DoUserCommand {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for DoUserCommand {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::DoUserCommand((s, v, lc)) => {
                println!("test entry - DoUserCommand");
                let v_slice: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
                com_out = do_user_command(&s, &v_slice, &lc);
                //com_out = "test entry - DoUserCommand".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::DoUserCommand(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct UnfundedClient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for UnfundedClient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::UnfundedClient((s_1, s_2)) => {
                println!("test entry - UnfundedClient");
                com_out = "test entry - UnfundedClient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::UnfundedClient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct Faucet {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for Faucet {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::Faucet((s_1, s_2)) => {
                println!("test entry - Faucet");
                com_out = "test entry - Faucet".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::Faucet(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetRecipient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetRecipient((s_1, s_2)) => {
                println!("test entry - FaucetRecipient");
                com_out = "test entry - FaucetRecipient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetRecipient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetFundedRecipient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetFundedRecipient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetFundedRecipient((s_1, s_2)) => {
                println!("test entry - FaucetFundedRecipient");
                com_out = "test entry - FaucetFundedRecipient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetFundedRecipient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct GenerateNBlocksReturnNewHeight {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for GenerateNBlocksReturnNewHeight {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::GenerateNBlocksReturnNewHeight((s_1, s_2)) => {
                println!("test entry - GenerateNBlocksReturnNewHeight");
                com_out = "test entry - GenerateNBlocksReturnNewHeight".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetRecipientDefault {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipientDefault {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetRecipientDefault((s_1, s_2)) => {
                println!("test entry - FaucetRecipientDefault");
                com_out = "test entry - FaucetRecipientDefault".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetRecipientDefault(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

// --- run_com
// --- runs command received and returns output if exists
fn run_command(
    com_str: &str,
    com_nametype: &str,
    com_type: &str,
    com_name: &str,
    com_inputs_vec: Vec<&str>,
    com_options_vec: Vec<&str>,
) -> String {
    let com_lib = command_lib();
    let mut com_out = String::new();
    let num_com_inputs = com_inputs_vec.len();
    let num_com_options = com_options_vec.len();

    println!("In run_com:");

    // --- change input type to enum, add match to find input type and put code below in main??? also change output type to enum??? then have seperate function that calls run_command and hold state of outputs (lightclients..) (or put in main?)
    // --- THIS - change input_vec, output_vec to enum, create second function to call run_command from string_input and return string_output(possible call seperate function to unwrap input_string) (move input and output match statements here)(this is where actual LightClient.. are held in scope and new calls to run_command are made) (once called, program never leaves this function until all commands have been executed)
    println!("Loading test scenario");
    let (_regtest_manager, _cph, _faucet, recipient, _txid) =
        RT.block_on(async move { scenarios::faucet_funded_recipient_default(100_000).await });
    let command_inputs = CommandInput::DoUserCommand(("balance".to_string(), vec![], recipient));
    // ---

    for (key, value) in &com_lib {
        if com_nametype == *key {
            let com_out_com = value.exec(&com_nametype, command_inputs, com_options_vec);
            match com_out_com {
                CommandOutput::DoUserCommand(output) => {
                    println!("DoUserCommand with value: {}", output);
                    com_out = output;
                }
                CommandOutput::UnfundedClient(output) => {
                    println!("UnfundedClient with value: {}", output);
                    com_out = output;
                }
                CommandOutput::Faucet(output) => {
                    println!("Faucet with value: {}", output);
                    com_out = output;
                }
                CommandOutput::FaucetRecipient(output) => {
                    println!("FaucetRecipient with value: {}", output);
                    com_out = output;
                }
                CommandOutput::FaucetFundedRecipient(output) => {
                    println!("FaucetFundedRecipient with value: {}", output);
                    com_out = output;
                }
                CommandOutput::GenerateNBlocksReturnNewHeight(output) => {
                    println!("DoUserCommand with value: {}", output);
                    com_out = output;
                }
                CommandOutput::FaucetRecipientDefault(output) => {
                    println!("FaucetRecipientDefault with value: {}", output);
                    com_out = output;
                }
            }
            break;
        } else {
            println!("Command not recognised");
            com_out = "Command not recognised".to_string();
        }
    }

    println!("Command complete");

    com_out
}

// --- main
// ---
fn main() {
    let out_string = run_command(
        "do_user_command(1, 2, 3)",
        "do_user_command",
        "do_user_command",
        "do_user_command",
        vec!["1", "2", "3"],
        vec![],
    );
    println!("Output: {}", out_string);
}
