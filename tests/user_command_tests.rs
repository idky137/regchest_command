// user_command_test.rs
// use: holds tests for do_user_command
// authers: idky137
//

use lazy_static::lazy_static;
use regchest_command::regchest_command::{regchest_command, CommandInput, CommandOutput};
use tokio::runtime::Runtime;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingoconfig::RegtestNetwork;
use zingolib::{
    get_base_address,
    lightclient::{LightClient, PoolBalances},
    wallet::Pool,
};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[test]
fn test_user_command_balance() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::faucet_funded_recipient";
    let command_inputs_1 = CommandInput::FaucetFundedRecipient(
        Some(100_000),
        Some(100_000),
        Some(100_000),
        Pool::Orchard,
        regtest_network,
    );
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let _regtest_manager: RegtestManager;
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
            _regtest_manager = regtest_manager_v;
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

    let command_str_2 = "do_user_command";
    let command_inputs_2 = CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient));
    let command_output_2 = regchest_command(command_str_2, &command_inputs_2);

    match command_output_2 {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;

            let balance_check_1 = serde_json::to_string_pretty(&PoolBalances {
                sapling_balance: Some(100000),
                verified_sapling_balance: Some(100000),
                spendable_sapling_balance: Some(100000),
                unverified_sapling_balance: Some(0),
                orchard_balance: Some(100000),
                verified_orchard_balance: Some(100000),
                spendable_orchard_balance: Some(100000),
                unverified_orchard_balance: Some(0),
                transparent_balance: Some(100000),
            })
            .unwrap();

            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }
}

#[test]
fn test_user_command_send_and_sync() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::faucet_funded_recipient";
    let command_inputs_1 = CommandInput::FaucetFundedRecipient(
        Some(100_000),
        Some(100_000),
        Some(100_000),
        Pool::Orchard,
        regtest_network,
    );
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
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
            faucet = faucet_v;
            recipient = recipient_v;
            _opt1 = opt1_v;
            _opt2 = opt2_v;
            _opt3 = opt3_v;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_str_2 = "do_user_command";
    let command_inputs_2 = CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient));
    let command_output_2 = regchest_command(command_str_2, &command_inputs_2);

    match command_output_2 {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;
            let balance_check_1 = serde_json::to_string_pretty(&PoolBalances {
                sapling_balance: Some(100000),
                verified_sapling_balance: Some(100000),
                spendable_sapling_balance: Some(100000),
                unverified_sapling_balance: Some(0),
                orchard_balance: Some(100000),
                verified_orchard_balance: Some(100000),
                spendable_orchard_balance: Some(100000),
                unverified_orchard_balance: Some(0),
                transparent_balance: Some(100000),
            })
            .unwrap();
            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_str_3 = "do_user_command";
    let recipient_address = RT.block_on(async { get_base_address!(&recipient, "unified") });
    let command_args_3 = vec![recipient_address, "100000".to_string()];
    let command_inputs_3 =
        CommandInput::DoUserCommand(("send".to_string(), command_args_3, &faucet));
    let _command_output_3 = regchest_command(command_str_3, &command_inputs_3);

    let command_str_4 = "generate_n_blocks_return_new_height";
    let command_inputs_4 = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 1);
    let _command_output_4 = regchest_command(command_str_4, &command_inputs_4);

    let command_str_5 = "do_user_command";
    let command_inputs_5 = CommandInput::DoUserCommand(("sync".to_string(), vec![], &recipient));
    let _command_output_5 = regchest_command(command_str_5, &command_inputs_5);

    let command_str_6 = "do_user_command";
    let command_inputs_6 = CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient));
    let command_output_6 = regchest_command(command_str_6, &command_inputs_6);

    match command_output_6 {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;
            let balance_check = serde_json::to_string_pretty(&PoolBalances {
                sapling_balance: Some(100000),
                verified_sapling_balance: Some(100000),
                spendable_sapling_balance: Some(100000),
                unverified_sapling_balance: Some(0),
                orchard_balance: Some(200000),
                verified_orchard_balance: Some(200000),
                spendable_orchard_balance: Some(200000),
                unverified_orchard_balance: Some(0),
                transparent_balance: Some(100000),
            })
            .unwrap();
            assert_eq!(balance_out, balance_check);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }
}
