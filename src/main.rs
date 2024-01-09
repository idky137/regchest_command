// regchest_command main.rs
// use: command library to build custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use zingo_testutils::{
    generate_n_blocks_return_new_height,
    regtest::{ChildProcessHandler, RegtestManager},
    scenarios,
};
use zingoconfig::RegtestNetwork;
use zingolib::{commands::do_user_command, lightclient::LightClient, wallet::Pool};

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
    GenerateNBlocksReturnNewHeight(RegtestManager, u32),
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
    GenerateNBlocksReturnNewHeight(u32),
}

struct DoUserCommand {}
impl CommandExec<CommandInput, CommandOutput> for DoUserCommand {
    fn exec(&self, com_inputs: CommandInput) -> CommandOutput {
        let com_out: String;
        match com_inputs {
            CommandInput::DoUserCommand((s, v, lc)) => {
                println!("Test entry - in DoUserCommand");
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
        let regtest_manager: RegtestManager;
        let cph: ChildProcessHandler;
        let client: LightClient;
        match com_inputs {
            CommandInput::UnfundedClient(rn) => {
                println!("Test entry - in UnfundedClient");
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
        let regtest_manager: RegtestManager;
        let cph: ChildProcessHandler;
        let client: LightClient;
        match com_inputs {
            CommandInput::Faucet(p, rn) => {
                println!("Test entry - in Faucet");
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
        let regtest_manager: RegtestManager;
        let cph: ChildProcessHandler;
        let faucet: LightClient;
        let recipient: LightClient;
        match com_inputs {
            CommandInput::FaucetRecipient(p, rn) => {
                println!("Test entry - in FaucetRecipient");

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
        let regtest_manager: RegtestManager;
        let cph: ChildProcessHandler;
        let faucet: LightClient;
        let recipient: LightClient;
        let opo1: Option<String>;
        let opo2: Option<String>;
        let opo3: Option<String>;
        match com_inputs {
            CommandInput::FaucetFundedRecipient(op1, op2, op3, p, rn) => {
                println!("Test entry - in FaucetFundedRecipient");
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
        let com_out: u32;
        match com_inputs {
            CommandInput::GenerateNBlocksReturnNewHeight(rm, n) => {
                println!("Test entry - in GenerateNBlocksReturnNewHeight");
                com_out = RT
                    .block_on(async move { generate_n_blocks_return_new_height(&rm, n).await })
                    .expect("Invalid response returned");
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(com_out)
    }
}

// --- run_com
// --- runs command received in "com_nametype"and returns output
fn run_command(com_nametype: &str, com_inputs: CommandInput) -> CommandOutput {
    let com_lib = command_lib();

    println!("Test entry - in run_command:");

    let com_output = match com_lib.get(&com_nametype) {
        Some(value) => value.exec(com_inputs),
        None => {
            panic!("Command not recognised");
        }
    };
    com_output
}

// --- print_co
// --- takes CommandOutput Enum and prints to console
fn print_command(co: CommandOutput) {
    println!("Test entry - in print_command:");
    match co {
        CommandOutput::DoUserCommand(user_command) => {
            println!("DoUserCommand Output: {}", user_command);
        }
        CommandOutput::UnfundedClient(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::UnfundedClient");
        }
        CommandOutput::Faucet(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::Faucet");
        }
        CommandOutput::FaucetRecipient(
            _regtest_manager,
            _child_process_handler,
            _sender_light_client,
            _recipient_light_client,
        ) => {
            println!("Scenario::FaucetRecipient");
        }
        CommandOutput::FaucetFundedRecipient(
            _regtest_manager,
            _child_process_handler,
            _sender_light_client,
            _recipient_light_client,
            _optional_field1,
            _optional_field2,
            _optional_field3,
        ) => {
            println!("Scenario::FaucetFundedRecipient");
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(new_height) => {
            println!("GenerateNBlocksReturnNewHeight Output: {}", new_height);
        }
    }
}

// --- main
// --- used for testing..
fn main() {
    println!("Test entry - Starting main:");
    println!("Test entry - Loading test scenario:");
    let (_regtest_manager, _cph, _faucet, recipient, _txid) =
        RT.block_on(async move { scenarios::faucet_funded_recipient_default(100_000).await });

    let command_str = "do_user_command";
    let command_inputs = CommandInput::DoUserCommand(("balance".to_string(), vec![], recipient));

    println!("Test entry - Calling run_command:");
    let command_output = run_command(command_str, command_inputs);
    print_command(command_output);
}
