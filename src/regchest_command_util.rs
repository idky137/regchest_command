// regchest_command_util.rs
// use: - util functions for regchest_command
//      - print_command: prints output message
//      - build_input_data: [incomplete]
//      - server_command: [incomplete]
// authers: idky137
//

use crate::regchest_command::{CommandInput, CommandOutput};
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

// --- server_command
// --- calls regchest_command from Vec<String> inputs
pub fn server_command() {
    // inputs: nametype: &String, args: &Vec<String>
    // outputs: CommandOutput

    // command_inputs: CommandInput = build_input_data(nametype, args);
    // command_output: CommandOutput = regchest_command(&nametype.as_str(), CommandInput)
}

// --- build_input_data
// --- takes nametype and vec of strings as input and returns correct CommandInput data for command
pub fn build_input_data(
    nametype: &String,
    args: &Vec<String>,
    regtest_manager_in: Option<&RegtestManager>,
    recipient_in: Option<&LightClient>,
    faucet_in: Option<&LightClient>,
) -> String {
    match nametype.as_str() {
        "do_user_command" => {
            match args[0].as_str() {
                "version" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "sync" => {
                    if args.len() != 2 {
                        panic!("du_user_command:sync requires 2 args");
                    }
                    if args[1] == "recipient" {
                        let command_inputs = CommandInput::DoUserCommand((
                            "sync".to_string(),
                            vec![],
                            recipient_in.unwrap(),
                        ));
                    } else if args[1] == "faucet" {
                        let command_inputs = CommandInput::DoUserCommand((
                            "sync".to_string(),
                            vec![],
                            faucet_in.unwrap(),
                        ));
                    } else {
                        panic!("incorrect lightclient type");
                    }
                }
                "syncstatus" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "encryptmessage" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "decryptmessage" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "parse_address" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "parse_viewkey" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "interrupt_sync_after_batch" => {
                    //interrupt_sync_after_batch
                }
                "changeserver" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "rescan" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "clear" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "help" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "balance" => {
                    //sync
                }
                "addresses" => {
                    //balance
                }
                "height" => {
                    //height
                }
                "sendprogress" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "setoption" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "summaries" => {
                    //summaries
                }
                "value_to_address" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "sends_to_address" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "memobytes_to_address" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "getoption" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "exportufvk" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "info" => {
                    //info
                }
                "updatecurrentprice" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "send" => {
                    //addresses
                }
                "shield" => {
                    //sheild
                }
                "save" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "quit" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "notes" => {
                    //notes
                }
                "new" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "defaultfee" => {
                    panic!("do_user_command variant not yet implemented");
                }
                "seed" => {
                    //seed
                }
                "get_birthday" => {
                    //get_birthday
                }
                "wallet_kind" => {
                    //wallet_kind
                }
                "delete" => {
                    panic!("do_user_command variant not yet implemented");
                }
                _ => {
                    panic!("incorrect do_user_command variant");
                }
            }
        }
        "scenarios::unfunded_client" => {
            let command_inputs =
                CommandInput::UnfundedClient(RegtestNetwork::all_upgrades_active());
        }
        "scenarios::faucet" => {
            let command_inputs =
                CommandInput::Faucet(Pool::Orchard, RegtestNetwork::all_upgrades_active());
        }
        "scenarios::faucent_recipient" => {
            let command_inputs =
                CommandInput::FaucetRecipient(Pool::Orchard, RegtestNetwork::all_upgrades_active());
        }
        "scenarios::faucet_funded_recipient" => {
            if args.len() != 3 {
                panic!("scenario::faucet_funded_recipient requires 3 args");
            }
            let command_inputs = CommandInput::FaucetFundedRecipient(
                Some(args[0].parse::<u64>().unwrap()),
                Some(args[1].parse::<u64>().unwrap()),
                Some(args[2].parse::<u64>().unwrap()),
                Pool::Orchard,
                RegtestNetwork::all_upgrades_active(),
            );
        }
        "generate_n_blocks_return_new_height" => {
            if let Some(regtest_manager) = regtest_manager_in {
                if args.len() != 1 {
                    panic!("generate_n_blocks_return_new_height requires 1 arg")
                }
                let command_inputs = CommandInput::GenerateNBlocksReturnNewHeight(
                    regtest_manager,
                    args[0].parse::<u32>().unwrap(),
                );
            } else {
                panic!("no regtest manager provided");
            }
        }
        _ => {
            panic!("incorrect command name");
        }
    }

    "Change output to CommandInput when done!".to_string()
}

// --- print_command
// --- prints output message
pub fn print_command(co: &CommandOutput) {
    match co {
        CommandOutput::DoUserCommand(user_command) => {
            println!("DoUserCommand Output: {}", user_command);
        }
        CommandOutput::UnfundedClient(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::UnfundedClient: Scenario loaded:");
        }
        CommandOutput::Faucet(_regtest_manager, _child_process_handler, _light_client) => {
            println!("Scenario::Faucet: Scenario loaded:");
        }
        CommandOutput::FaucetRecipient(
            _regtest_manager,
            _child_process_handler,
            _sender_light_client,
            _recipient_light_client,
        ) => {
            println!("Scenario::FaucetRecipient: Scenario loaded:");
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
                "Scenario::FaucetFundedRecipient: scenario loaded: \n- {}\n- {}\n- {}",
                _optional_field1.as_ref().unwrap(),
                _optional_field2.as_ref().unwrap(),
                _optional_field3.as_ref().unwrap()
            );
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(new_height) => {
            println!("GenerateNBlocksReturnNewHeight new height: {}", new_height);
        }
    }
}
