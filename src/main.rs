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
                println!("Test entry - in DoUserCommand");
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
                println!("Test entry - in UnfundedClient");
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
                println!("Test entry - in Faucet");
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
                println!("Test entry - in FaucetRecipient");

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
                println!("Test entry - in FaucetFundedRecipient");
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
                println!("Test entry - in GenerateNBlocksReturnNewHeight");
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

// --- run_com
// --- runs command received as &str with input type CommandInput and returns output type CommandOutput
fn run_command(com_nametype: &str, com_inputs: &CommandInput) -> CommandOutput {
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

// --- print_command
// --- takes CommandOutput Enum and prints to console
fn print_command(co: &CommandOutput) {
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
    println!("Starting Tests:");
    let regtest_network = RegtestNetwork::all_upgrades_active();

    // --- Test 1: Scenario:FaucetFundedRecipient
    println!("Test entry 1: Calling run_command::scenario::faucet_funded_recipient:");
    let command_str_1 = "scenarios::faucet_funded_recipient";
    let command_inputs_1 = CommandInput::FaucetFundedRecipient(
        Some(100_000),
        Some(100_000),
        Some(100_000),
        Pool::Orchard,
        regtest_network,
    );
    let command_output_1 = run_command(command_str_1, &command_inputs_1);
    print_command(&command_output_1);
    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let _faucet: LightClient;
    let recipient: LightClient;
    let _opt1: Option<String>;
    let _opt2: Option<String>;
    let _opt3: Option<String>;
    match command_output_1 {
        CommandOutput::FaucetFundedRecipient(
            regtest_manager_v,
            cph_v,
            faucet_v,
            recipient_v,
            opt1_v,
            opt2_v,
            opt3_v,
        ) => {
            regtest_manager = regtest_manager_v;
            _cph = cph_v;
            _faucet = faucet_v;
            recipient = recipient_v;
            _opt1 = opt1_v;
            _opt2 = opt2_v;
            _opt3 = opt3_v;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    // --- Test 2a: DoUserCommand:Balance
    println!("Test entry 2a: Calling run_command::do_user_command::balance:");
    let command_str_2a = "do_user_command";
    let command_inputs_2a =
        CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient));
    let command_output_2a = run_command(command_str_2a, &command_inputs_2a);
    print_command(&command_output_2a);

    // --- Test 2b: DoUserCommand:Balance
    println!("Test entry 2b: Calling run_command::do_user_command::address:");
    let command_str_2b = "do_user_command";
    let command_inputs_2b =
        CommandInput::DoUserCommand(("addresses".to_string(), vec![], &recipient));
    let command_output_2b = run_command(command_str_2b, &command_inputs_2b);
    print_command(&command_output_2b);

    // --- Test 3a: GenerateNBlocksReturnNewHeight
    println!("Test entry 3a: Calling run_command::generate_n_blocks_return_new_height(0):");
    let command_str_3a = "generate_n_blocks_return_new_height";
    let command_inputs_3a = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 0);
    let command_output_3a = run_command(command_str_3a, &command_inputs_3a);
    print_command(&command_output_3a);

    // --- Test 3b: GenerateNBlocksReturnNewHeight
    println!("Test entry 3b: Calling run_command::generate_n_blocks_return_new_height(10):");
    let command_str_3b = "generate_n_blocks_return_new_height";
    let command_inputs_3b = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 10);
    let command_output_3b = run_command(command_str_3b, &command_inputs_3b);
    print_command(&command_output_3b);
}
