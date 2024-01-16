// regchest_command.rs
// use: - command library to build custom scenarios.
//      - access using regchest_command
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
pub fn command_lib<'a>(
) -> HashMap<&'static str, Box<dyn CommandExec<&'a CommandInput<'a>, CommandOutput>>> {
    #[allow(unused_mut)]
    let mut entries: Vec<(
        &'static str,
        Box<dyn CommandExec<&'a CommandInput, CommandOutput>>,
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

pub enum CommandInput<'a> {
    DoUserCommand((String, Vec<String>, &'a LightClient)),
    UnfundedClient(RegtestNetwork),
    Faucet(Pool, RegtestNetwork),
    FaucetRecipient(Pool, RegtestNetwork),
    FaucetFundedRecipient(Option<u64>, Option<u64>, Option<u64>, Pool, RegtestNetwork),
    GenerateNBlocksReturnNewHeight(&'a RegtestManager, u32),
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
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for DoUserCommand {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::DoUserCommand((command_string, input_vec, lightclient)) => {
                let v_slice: Vec<&str> = input_vec.iter().map(|s| s.as_str()).collect();
                let com_out = do_user_command(command_string, &v_slice, lightclient);
                CommandOutput::DoUserCommand(com_out)
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

struct UnfundedClient {}
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for UnfundedClient {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::UnfundedClient(regtest_network) => {
                let (regtest_manager, cph, client) =
                    RT.block_on(async move { scenarios::unfunded_client(*regtest_network).await });
                CommandOutput::UnfundedClient(regtest_manager, cph, client)
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

struct Faucet {}
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for Faucet {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::Faucet(pool, regtest_network) => {
                let (regtest_manager, cph, client) =
                    RT.block_on(async move { scenarios::faucet(*pool, *regtest_network).await });
                CommandOutput::Faucet(regtest_manager, cph, client)
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

struct FaucetRecipient {}
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for FaucetRecipient {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::FaucetRecipient(pool, regtest_network) => {
                let (regtest_manager, cph, faucet, recipient) = RT.block_on(async move {
                    scenarios::faucet_recipient(*pool, *regtest_network).await
                });
                CommandOutput::FaucetRecipient(regtest_manager, cph, faucet, recipient)
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

struct FaucetFundedRecipient {}
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for FaucetFundedRecipient {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::FaucetFundedRecipient(
                option_1,
                option_2,
                option_3,
                pool,
                regtest_network,
            ) => {
                let (
                    regtest_manager,
                    cph,
                    faucet,
                    recipient,
                    option_out_1,
                    option_out_2,
                    option_out_3,
                ) = RT.block_on(async move {
                    scenarios::faucet_funded_recipient(
                        *option_1,
                        *option_2,
                        *option_3,
                        *pool,
                        *regtest_network,
                    )
                    .await
                });
                CommandOutput::FaucetFundedRecipient(
                    regtest_manager,
                    cph,
                    faucet,
                    recipient,
                    option_out_1,
                    option_out_2,
                    option_out_3,
                )
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

struct GenerateNBlocksReturnNewHeight {}
impl<'a> CommandExec<&'a CommandInput<'a>, CommandOutput> for GenerateNBlocksReturnNewHeight {
    fn exec(&self, com_inputs: &'a CommandInput) -> CommandOutput {
        match com_inputs {
            CommandInput::GenerateNBlocksReturnNewHeight(regtest_manager, blocks) => {
                let com_out = RT
                    .block_on(async move {
                        generate_n_blocks_return_new_height(regtest_manager, *blocks).await
                    })
                    .expect("Invalid response returned");
                CommandOutput::GenerateNBlocksReturnNewHeight(com_out)
            }
            _ => {
                panic!("Unexpected Command Input variant");
            }
        }
    }
}

// --- regchest_command
// --- runs command received as &str with input type CommandInput and returns output type CommandOutput
pub fn regchest_command(com_nametype: &str, com_inputs: &CommandInput) -> CommandOutput {
    let com_lib = command_lib();
    let com_output = match com_lib.get(&com_nametype) {
        Some(value) => value.exec(com_inputs),
        None => {
            panic!("Command not recognised");
        }
    };
    com_output
}
