// scenario_test.rs
// use: holds tests for loading basic scenarios
// authers: idky137
//

use regchest_command::regchest_command::{
    print_command, regchest_command, CommandInput, CommandOutput,
};
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingoconfig::RegtestNetwork;
use zingolib::{
    lightclient::{LightClient, PoolBalances},
    wallet::Pool,
};

#[test]
fn test_unfunded_client() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::unfunded_client";
    let command_inputs_1 = CommandInput::UnfundedClient(regtest_network);
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let _regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let recipient: LightClient;
    let balance_out: String;

    match command_output_1 {
        CommandOutput::UnfundedClient(regtest_manager_v, cph_v, recipient_v) => {
            _regtest_manager = regtest_manager_v;
            _cph = cph_v;
            recipient = recipient_v
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
            balance_out = command_out;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let balance_check = serde_json::to_string_pretty(&PoolBalances {
        sapling_balance: Some(0),
        verified_sapling_balance: Some(0),
        spendable_sapling_balance: Some(0),
        unverified_sapling_balance: Some(0),
        orchard_balance: Some(0),
        verified_orchard_balance: Some(0),
        spendable_orchard_balance: Some(0),
        unverified_orchard_balance: Some(0),
        transparent_balance: Some(0),
    })
    .unwrap();

    assert_eq!(balance_out, balance_check);
}

#[test]
fn test_faucet() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::faucet";
    let command_inputs_1 = CommandInput::Faucet(Pool::Orchard, regtest_network);
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let _regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
    let balance_out: String;

    match command_output_1 {
        CommandOutput::Faucet(regtest_manager_v, cph_v, faucet_v) => {
            _regtest_manager = regtest_manager_v;
            _cph = cph_v;
            faucet = faucet_v;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_str_2 = "do_user_command";
    let command_inputs_2 = CommandInput::DoUserCommand(("balance".to_string(), vec![], &faucet));
    let command_output_2 = regchest_command(command_str_2, &command_inputs_2);

    match command_output_2 {
        CommandOutput::DoUserCommand(command_out) => {
            balance_out = command_out;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let balance_check = serde_json::to_string_pretty(&PoolBalances {
        sapling_balance: Some(0),
        verified_sapling_balance: Some(0),
        spendable_sapling_balance: Some(0),
        unverified_sapling_balance: Some(0),
        orchard_balance: Some(1875000000),
        verified_orchard_balance: Some(1875000000),
        spendable_orchard_balance: Some(1875000000),
        unverified_orchard_balance: Some(0),
        transparent_balance: Some(0),
    })
    .unwrap();

    assert_eq!(balance_out, balance_check);
}

#[test]
fn test_faucet_recipient() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::faucet_recipient";
    let command_inputs_1 = CommandInput::FaucetRecipient(Pool::Orchard, regtest_network);
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let _regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let _faucet: LightClient;
    let recipient: LightClient;
    let balance_out: String;

    match command_output_1 {
        CommandOutput::FaucetRecipient(regtest_manager_v, cph_v, faucet_v, recipient_v) => {
            _regtest_manager = regtest_manager_v;
            _cph = cph_v;
            _faucet = faucet_v;
            recipient = recipient_v;
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
            balance_out = command_out;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let balance_check = serde_json::to_string_pretty(&PoolBalances {
        sapling_balance: Some(0),
        verified_sapling_balance: Some(0),
        spendable_sapling_balance: Some(0),
        unverified_sapling_balance: Some(0),
        orchard_balance: Some(0),
        verified_orchard_balance: Some(0),
        spendable_orchard_balance: Some(0),
        unverified_orchard_balance: Some(0),
        transparent_balance: Some(0),
    })
    .unwrap();

    assert_eq!(balance_out, balance_check);
}

#[test]
fn test_faucet_funded_recipient() {
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
    let balance_out: String;

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
            balance_out = command_out;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let balance_check = serde_json::to_string_pretty(&PoolBalances {
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

    assert_eq!(balance_out, balance_check);
}
