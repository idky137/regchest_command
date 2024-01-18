// usage_examples.rs
// use: usage examples
// authers: idky137
//

use lazy_static::lazy_static;
use regchest_command::regchest_command::{regchest_command, CommandInput, CommandOutput};
use regchest_command::regchest_command_util::{print_command, server_command, server_print};
use tokio::runtime::Runtime;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingoconfig::RegtestNetwork;
use zingolib::{get_base_address, lightclient::LightClient, wallet::Pool};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[test]
#[ignore]
fn regchest_command_basic_usage() {
    let regtest_network = RegtestNetwork::all_upgrades_active();
    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
    let recipient: LightClient;

    let command_output_1 = regchest_command(
        "scenarios::faucet_funded_recipient",
        &CommandInput::FaucetFundedRecipient(
            Some(100_000),
            Some(100_000),
            Some(100_000),
            Pool::Orchard,
            regtest_network,
        ),
    );
    print_command(&command_output_1);

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
            let _opt1 = opt1_v;
            let _opt2 = opt2_v;
            let _opt3 = opt3_v;
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_output_2 = regchest_command(
        "do_user_command",
        &CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient)),
    );
    print_command(&command_output_2);

    let command_output_3 = regchest_command(
        "do_user_command",
        &CommandInput::DoUserCommand((
            "send".to_string(),
            vec![
                RT.block_on(async { get_base_address!(&recipient, "unified") }),
                "100_000".to_string(),
            ],
            &faucet,
        )),
    );
    print_command(&command_output_3);

    let command_output_4 = regchest_command(
        "generate_n_blocks_return_new_height",
        &CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 1),
    );
    print_command(&command_output_4);

    let command_output_5 = regchest_command(
        "do_user_command",
        &CommandInput::DoUserCommand(("sync".to_string(), vec![], &recipient)),
    );
    print_command(&command_output_5);

    let command_output_6 = regchest_command(
        "do_user_command",
        &CommandInput::DoUserCommand(("balance".to_string(), vec![], &recipient)),
    );
    print_command(&command_output_6);
}

#[test]
#[ignore]
fn server_command_basic_usage() {
    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
    let recipient: LightClient;

    let command_out_1 = server_command(
        &vec!["scenarios::faucet_recipient".to_string()],
        None,
        None,
        None,
    );
    print_command(&command_out_1);

    match command_out_1 {
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

    let command_out_2 = server_command(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_2);

    let command_out_3 = server_command(
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
    print_command(&command_out_3);

    let command_out_4 = server_command(
        &vec![
            "generate_n_blocks_return_new_height".to_string(),
            "1".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_4);

    let command_out_5 = server_command(
        &vec![
            "do_user_command".to_string(),
            "sync".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_5);

    let command_out_6 = server_command(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_6);
}

#[test]
#[ignore]
fn server_print_basic_usage() {
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
            panic!("Error: Incorrect outputs");
        }
    }

    server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );

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
            "1".to_string(),
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

    server_print(
        &vec![
            "do_user_command".to_string(),
            "balance".to_string(),
            "recipient".to_string(),
        ],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
}
