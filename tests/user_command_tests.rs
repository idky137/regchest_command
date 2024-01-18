// user_command_test.rs
// use: holds tests for do_user_command
// authers: idky137
//

use regchest_command::{regchest_command::CommandOutput, regchest_command_util::server_print};
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingolib::lightclient::{LightClient, PoolBalances};

#[test]
fn test_user_command() {
    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
    let recipient: LightClient;

    match server_print(
        &vec!["scenarios::faucet_recipient".to_string()],
        None,
        None,
        None,
    ) {
        CommandOutput::FaucetRecipient(regtest_manager_v, cph_v, faucet_v, recipient_v) => {
            regtest_manager = regtest_manager_v;
            _cph = cph_v;
            faucet = faucet_v;
            recipient = recipient_v;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    match server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    ) {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;
            let balance_check_1 = serde_json::to_string_pretty(&PoolBalances {
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
            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    server_print(
        &vec![
            "do_user_command".to_string(),
            "send".to_string(),
            "faucet".to_string(),
            "recipient".to_string(),
            "100000".to_string(),
            "unified".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "generate_n_blocks_return_new_height".to_string(),
            "10".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "do_user_command".to_string(),
            "sync".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    match server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    ) {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;
            let balance_check_1 = serde_json::to_string_pretty(&PoolBalances {
                sapling_balance: Some(0),
                verified_sapling_balance: Some(0),
                spendable_sapling_balance: Some(0),
                unverified_sapling_balance: Some(0),
                orchard_balance: Some(100000),
                verified_orchard_balance: Some(100000),
                spendable_orchard_balance: Some(100000),
                unverified_orchard_balance: Some(0),
                transparent_balance: Some(0),
            })
            .unwrap();
            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    server_print(
        &vec![
            "do_user_command".to_string(),
            "send".to_string(),
            "faucet".to_string(),
            "recipient".to_string(),
            "100000".to_string(),
            "sapling".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "generate_n_blocks_return_new_height".to_string(),
            "10".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "do_user_command".to_string(),
            "sync".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    match server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    ) {
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
                transparent_balance: Some(0),
            })
            .unwrap();
            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    server_print(
        &vec![
            "do_user_command".to_string(),
            "shield".to_string(),
            "recipient".to_string(),
            "all".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "generate_n_blocks_return_new_height".to_string(),
            "10".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    server_print(
        &vec![
            "do_user_command".to_string(),
            "sync".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

    match server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    ) {
        CommandOutput::DoUserCommand(command_out) => {
            let balance_out = command_out;
            let balance_check_1 = serde_json::to_string_pretty(&PoolBalances {
                sapling_balance: Some(0),
                verified_sapling_balance: Some(0),
                spendable_sapling_balance: Some(0),
                unverified_sapling_balance: Some(0),
                orchard_balance: Some(190000),
                verified_orchard_balance: Some(190000),
                spendable_orchard_balance: Some(190000),
                unverified_orchard_balance: Some(0),
                transparent_balance: Some(0),
            })
            .unwrap();
            assert_eq!(balance_out, balance_check_1);
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }
    server_print(
        &vec![
            "do_user_command".to_string(),
            "notes".to_string(),
            "recipient".to_string(),
            "all".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    server_print(
        &vec![
            "do_user_command".to_string(),
            "summaries".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
}
