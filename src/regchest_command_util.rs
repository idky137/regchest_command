// regchest_command_util.rs
// use: - util functions for regchest_command
//      - print_command: prints output message
//      - build_input_data: [incomplete]
//      - server_command: [incomplete]
// authers: idky137
//

use crate::regchest_command::CommandOutput;
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

// --- print_command
// --- prints output message
pub fn print_command(co: &CommandOutput) {
    match co {
        CommandOutput::DoUserCommand(user_command) => {
            println!("DoUserCommand Output: {}", user_command);
        }
        CommandOutput::UnfundedClient(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::UnfundedClient: Scenario loaded");
        }
        CommandOutput::Faucet(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::Faucet: Scenario loaded");
        }
        CommandOutput::FaucetRecipient(
            _regtest_manager,
            _child_process_handler,
            _sender_light_client,
            _recipient_light_client,
        ) => {
            println!("Scenario::FaucetRecipient: Scenario loaded");
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
            println!(
                "Scenario::FaucetFundedRecipient: Scenario loaded - {} {} {}",
                _optional_field1.as_ref().unwrap(),
                _optional_field2.as_ref().unwrap(),
                _optional_field3.as_ref().unwrap()
            );
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(new_height) => {
            println!("GenerateNBlocksReturnNewHeight Output: {}", new_height);
        }
    }
}
