// regchest_command main.rs
// use: command library to build custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use zingo_testutils::scenarios;
use zingolib::{
    commands::{self, do_user_command},
    lightclient::LightClient,
};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

pub trait CommandExec<I, O> {
    fn exec(&self, com_inputs: I) -> O;
}

// --- command_lib
// --- library of available functions
pub fn command_lib() -> HashMap<&'static str, Box<dyn CommandExec<CommandInput, CommandOutput>>> {
    #[allow(unused_mut)]
    let mut entries: Vec<(
        &'static str,
        Box<dyn CommandExec<CommandInput, CommandOutput>>,
    )> = vec![
        ("do_user_command", Box::new(DoUserCommand {})),
        ("scenarios::unfunded_client", Box::new(UnfundedClient {})),
        ("scenarios::faucet", Box::new(Faucet {})),
        ("scenarios::faucet_recipient", Box::new(FaucetRecipient {})),
        (
            "scenarios::faucet_funded_recipient",
            Box::new(FaucetFundedRecipient {}),
        ),
        (
            "generate_n_blocks_return_new_height",
            Box::new(GenerateNBlocksReturnNewHeight {}),
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
}

#[derive(Debug)]
pub enum CommandOutput {
    DoUserCommand(String),
    UnfundedClient(String),
    Faucet(String),
    FaucetRecipient(String),
    FaucetFundedRecipient(String),
    GenerateNBlocksReturnNewHeight(String),
}

struct DoUserCommand {}
impl CommandExec<CommandInput, CommandOutput> for DoUserCommand {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

struct UnfundedClient {}
impl CommandExec<CommandInput, CommandOutput> for UnfundedClient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

struct Faucet {}
impl CommandExec<CommandInput, CommandOutput> for Faucet {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

struct FaucetRecipient {}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

struct FaucetFundedRecipient {}
impl CommandExec<CommandInput, CommandOutput> for FaucetFundedRecipient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

struct GenerateNBlocksReturnNewHeight {}
impl CommandExec<CommandInput, CommandOutput> for GenerateNBlocksReturnNewHeight {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
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
}

// --- run_com
// --- runs command received and returns output if exists
fn run_command(com_nametype: &str, com_inputs: CommandInput) -> CommandOutput {
    let com_lib = command_lib();

    println!("In run_com:");

    let com_output = match com_lib.get(&com_nametype) {
        Some(value) => value.exec(com_inputs),
        None => {
            println!("Command not recognised");
            CommandOutput::DoUserCommand("command not recognised".to_string())
        }
    };
    println!("Command complete");
    com_output
}

// --- main
// ---
fn main() {
    println!("Loading test scenario");
    let (_regtest_manager, _cph, _faucet, recipient, _txid) =
        RT.block_on(async move { scenarios::faucet_funded_recipient_default(100_000).await });

    let command_str = "do_user_command";
    let command_inputs = CommandInput::DoUserCommand(("balance".to_string(), vec![], recipient));

    println!("Calling run_command");
    let command_output = run_command(command_str, command_inputs);
    println!("Output: {:?}", command_output);
}
