// regchest_command main.rs
// use: command library to build custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use zingo_testutils::{
    regtest::{ChildProcessHandler, RegtestManager},
    scenarios,
};
use zingoconfig::RegtestNetwork;
use zingolib::{
    commands::{self, do_user_command},
    lightclient::LightClient,
    wallet::Pool,
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
    UnfundedClient(RegtestNetwork),
    Faucet(Pool, RegtestNetwork),
    FaucetRecipient(Pool, RegtestNetwork),
    FaucetFundedRecipient(Option<u64>, Option<u64>, Option<u64>, Pool, RegtestNetwork),
    GenerateNBlocksReturnNewHeight((String, String)),
}

pub enum CommandOutput {
    DoUserCommand(String),
    UnfundedClient(RegtestManager, ChildProcessHandler, LightClient),
    Faucet(RegtestManager, ChildProcessHandler, LightClient),
    FaucetRecipient(
        RegtestManager,
        ChildProcessHandler,
        LightClient,
        LightClient,
    ),
    FaucetFundedRecipient(
        RegtestManager,
        ChildProcessHandler,
        LightClient,
        LightClient,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
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
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::DoUserCommand(com_out)
    }
}

struct UnfundedClient {}
impl CommandExec<CommandInput, CommandOutput> for UnfundedClient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
        let mut regtest_manager: RegtestManager;
        let mut cph: ChildProcessHandler;
        let mut client: LightClient;
        match com_inputs {
            CommandInput::UnfundedClient(rn) => {
                println!("test entry - UnfundedClient");
                (regtest_manager, cph, client) =
                    RT.block_on(async move { scenarios::unfunded_client(rn).await });
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::UnfundedClient(regtest_manager, cph, client)
    }
}

struct Faucet {}
impl CommandExec<CommandInput, CommandOutput> for Faucet {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
        let mut regtest_manager: RegtestManager;
        let mut cph: ChildProcessHandler;
        let mut client: LightClient;
        match com_inputs {
            CommandInput::Faucet(p, rn) => {
                println!("test entry - Faucet");
                (regtest_manager, cph, client) =
                    RT.block_on(async move { scenarios::faucet(p, rn).await });
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::Faucet(regtest_manager, cph, client)
    }
}

struct FaucetRecipient {}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
        let mut regtest_manager: RegtestManager;
        let mut cph: ChildProcessHandler;
        let mut faucet: LightClient;
        let mut recipient: LightClient;
        match com_inputs {
            CommandInput::FaucetRecipient(p, rn) => {
                println!("test entry - FaucetRecipient");

                (regtest_manager, cph, faucet, recipient) =
                    RT.block_on(async move { scenarios::faucet_recipient(p, rn).await });
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::FaucetRecipient(regtest_manager, cph, faucet, recipient)
    }
}

struct FaucetFundedRecipient {}
impl CommandExec<CommandInput, CommandOutput> for FaucetFundedRecipient {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
        let mut regtest_manager: RegtestManager;
        let mut cph: ChildProcessHandler;
        let mut faucet: LightClient;
        let mut recipient: LightClient;
        let mut opo1: Option<String>;
        let mut opo2: Option<String>;
        let mut opo3: Option<String>;
        match com_inputs {
            CommandInput::FaucetFundedRecipient(op1, op2, op3, p, rn) => {
                println!("test entry - FaucetFundedRecipient");
                (regtest_manager, cph, faucet, recipient, opo1, opo2, opo3) =
                    RT.block_on(async move {
                        scenarios::faucet_funded_recipient(op1, op2, op3, p, rn).await
                    });
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::FaucetFundedRecipient(
            regtest_manager,
            cph,
            faucet,
            recipient,
            opo1,
            opo2,
            opo3,
        )
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
                panic!("Unexpected Command Input variant");
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
            panic!("Command not recognised");
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
    // println!("Output: {:?}", command_output);
}
