// regchest_command_util.rs
// use: - util functions for regchest_command
//      - print_command: prints output message from CommandOutput
//      - build_input_data: builds CommandInput from vec of strings
//      - server_command: calls regchest_command, building input with build_input_data.
// authers: idky137
//

use crate::regchest_command::{regchest_command, CommandInput, CommandOutput};
use core::panic;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use zingo_testutils::regtest::RegtestManager;
use zingoconfig::RegtestNetwork;
use zingolib::{get_base_address, lightclient::LightClient, wallet::Pool};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

// --- server_command
// --- calls regchest_command from Vec<String> inputs
pub fn server_command(
    nametype: &String,
    args: &Vec<String>,
    regtest_manager_in: Option<&RegtestManager>,
    recipient_in: Option<&LightClient>,
    faucet_in: Option<&LightClient>,
) -> CommandOutput {
    return regchest_command(
        nametype.as_str(),
        &build_input_data(nametype, args, regtest_manager_in, recipient_in, faucet_in),
    );
}

// --- build_input_data
// --- takes nametype and vec of strings as input and returns correct CommandInput data for command
pub fn build_input_data<'a>(
    nametype: &'a String,
    args: &'a Vec<String>,
    regtest_manager_in: Option<&'a RegtestManager>,
    recipient_in: Option<&'a LightClient>,
    faucet_in: Option<&'a LightClient>,
) -> CommandInput<'a> {
    match nametype.as_str() {
        "do_user_command" => match args[0].as_str() {
            "version" | "defaultfee" => {
                if args.len() != 1 {
                    panic!("do_user_command:{} requires 1 arg: ({})", args[0], args[0]);
                }
                if let Some(recipient) = recipient_in {
                    return CommandInput::DoUserCommand((args[0].to_owned(), vec![], recipient));
                } else if let Some(faucet) = faucet_in {
                    return CommandInput::DoUserCommand((args[0].to_owned(), vec![], faucet));
                } else {
                    panic!("lightclient not provided");
                }
            }
            "sync"
            | "syncstatus"
            | "rescan"
            | "clear"
            | "balance"
            | "addresses"
            | "height"
            | "sendprogress"
            | "summaries"
            | "value_to_address"
            | "sends_to_address"
            | "memobytes_to_address"
            | "exportufvk"
            | "info"
            | "updatecurrentprice"
            | "quit"
            | "seed"
            | "get_birthday"
            | "wallet_kind"
            | "delete" => {
                if args.len() != 2 {
                    panic!(
                        "do_user_command:{} requires 2 args: ({}, target lightclient)",
                        args[0], args[0]
                    );
                }
                if args[1] == "recipient" {
                    if let Some(recipient) = recipient_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![],
                            recipient,
                        ));
                    } else {
                        panic!("recipient lightclient not provided");
                    }
                } else if args[1] == "faucet" {
                    if let Some(faucet) = faucet_in {
                        return CommandInput::DoUserCommand((args[0].to_string(), vec![], faucet));
                    } else {
                        panic!("faucet lightclient not provided");
                    }
                } else {
                    panic!("incorrect target lightclient type (recipient or faucet)");
                }
            }
            "interrupt_sync_after_batch" => {
                if args.len() != 3 {
                    panic!(
                            "do_user_command:{} requires 3 args: ({}, target lightclient, set interrupt bool)",
                            args[0], args[0]
                        );
                }
                if args[2] != "true".to_string() && args[2] != "false".to_string() {
                    panic!("arg[2] must be either be true or false");
                }
                if args[1] == "recipient" {
                    if let Some(recipient) = recipient_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![args[2].to_string()],
                            recipient,
                        ));
                    } else {
                        panic!("recipient lightclient not provided");
                    }
                } else if args[1] == "faucet" {
                    if let Some(faucet) = faucet_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![args[2].to_string()],
                            faucet,
                        ));
                    } else {
                        panic!("faucet lightclient not provided");
                    }
                } else {
                    panic!("incorrect target lightclient type");
                }
            }
            "send" => {
                if args.len() <= 4 || args.len() >= 7 {
                    panic!("do_user_command:{} requires either 5 or 6 args: ({},sending lightclient, target lightclient, amount to send, address type (unified, sapling or transparent), memo(optional))",args[0], args[0]);
                }
                if args[5].as_str() != "unified"
                    || args[5].as_str() != "sapling"
                    || args[5] != "transparent"
                {
                    panic!("incorrect address type (unified, sapling or transparent)");
                }
                let mut memo = String::new();
                if args.len() == 6 {
                    memo = args[6].to_string();
                }
                match (args[1].as_str(), args[2].as_str()) {
                    ("recipient", "recipient") => {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![
                                RT.block_on(async {
                                    get_base_address!(&recipient_in.unwrap(), args[5].as_str())
                                }),
                                args[3].to_string(),
                                memo,
                            ],
                            &recipient_in.unwrap(),
                        ));
                    }
                    ("recipient", "faucet") => {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![
                                RT.block_on(async {
                                    get_base_address!(&faucet_in.unwrap(), args[5].as_str())
                                }),
                                args[3].to_string(),
                                memo,
                            ],
                            &recipient_in.unwrap(),
                        ));
                    }
                    ("faucet", "recipient") => {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![
                                RT.block_on(async {
                                    get_base_address!(&recipient_in.unwrap(), args[5].as_str())
                                }),
                                args[3].to_string(),
                                memo,
                            ],
                            &faucet_in.unwrap(),
                        ));
                    }
                    ("faucet", "faucet") => {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![
                                RT.block_on(async {
                                    get_base_address!(&faucet_in.unwrap(), args[5].as_str())
                                }),
                                args[3].to_string(),
                                memo,
                            ],
                            &faucet_in.unwrap(),
                        ));
                    }
                    _ => {
                        panic!("incorrect target lightclient type");
                    }
                }
            }
            "shield" => {
                if args.len() != 3 {
                    panic!(
                        "do_user_command:{} requires 3 args: ({},lightclient to shield, pool to shield)",
                        args[0], args[0]
                    );
                }
                if args[2].as_str() != "transparent"
                    || args[2].as_str() != "sapling"
                    || args[2] != "all"
                {
                    panic!("incorrect pool type (transparent, sapling or all)");
                }
                if args[1] == "recipient" {
                    if let Some(recipient) = recipient_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![args[2].to_string()],
                            recipient,
                        ));
                    } else {
                        panic!("recipient lightclient not provided");
                    }
                } else if args[1] == "faucet" {
                    if let Some(faucet) = faucet_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![args[2].to_string()],
                            faucet,
                        ));
                    } else {
                        panic!("faucet lightclient not provided");
                    }
                } else {
                    panic!("incorrect target lightclient type");
                }
            }
            "notes" => {
                if args.len() <= 2 || args.len() >= 4 {
                    panic!(
                        "do_user_command:{} requires 2 or 3 args: ({},lightclient to print notes from, include unspent notes [all](optional))",
                        args[0], args[0]
                    );
                }
                let mut unspent: String = String::new();
                if args.len() == 3 {
                    if args[2].as_str() != "all" {
                        panic!("enter [all] as 3rd arg to include unspent notes");
                    }
                    unspent = args[2].to_string();
                }
                if args[1] == "recipient" {
                    if let Some(recipient) = recipient_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![unspent],
                            recipient,
                        ));
                    } else {
                        panic!("recipient lightclient not provided");
                    }
                } else if args[1] == "faucet" {
                    if let Some(faucet) = faucet_in {
                        return CommandInput::DoUserCommand((
                            args[0].to_string(),
                            vec![unspent],
                            faucet,
                        ));
                    } else {
                        panic!("faucet lightclient not provided");
                    }
                } else {
                    panic!("incorrect target lightclient type");
                }
            }
            "encryptmessage" | "decryptmessage" | "parse_address" | "parse_viewkey"
            | "changeserver" | "help" | "setoption" | "getoption" | "save" | "new" => {
                panic!(
                        "do_user_command:{} input data builder not yet implemented [ add to build_input_data() ]",
                        args[0]
                    );
            }
            _ => {
                panic!("incorrect do_user_command variant");
            }
        },
        "scenarios::unfunded_client" => {
            return CommandInput::UnfundedClient(RegtestNetwork::all_upgrades_active());
        }
        "scenarios::faucet" => {
            return CommandInput::Faucet(Pool::Orchard, RegtestNetwork::all_upgrades_active());
        }
        "scenarios::faucent_recipient" => {
            return CommandInput::FaucetRecipient(
                Pool::Orchard,
                RegtestNetwork::all_upgrades_active(),
            );
        }
        "scenarios::faucet_funded_recipient" => {
            if args.len() != 3 {
                panic!("scenario::faucet_funded_recipient requires 3 args");
            }
            return CommandInput::FaucetFundedRecipient(
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
                return CommandInput::GenerateNBlocksReturnNewHeight(
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
