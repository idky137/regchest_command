// main.rs
// use: command library to build custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use regchest_command::regchest_command::{regchest_command, CommandInput, CommandOutput};
use regchest_command::regchest_command_util::{print_command, server_command};
use tokio::runtime::Runtime;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingoconfig::RegtestNetwork;
use zingolib::{get_base_address, lightclient::LightClient, wallet::Pool};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

// --- main
// --- usage example
fn main() {
    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let faucet: LightClient;
    let recipient: LightClient;

    let command_out_1 = server_command(
        &"scenarios::faucet_recipient".to_string(),
        &vec![],
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
        &"do_user_command".to_string(),
        &vec!["balance".to_string(), "recipient".to_string()],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_2);

    let command_out_3 = server_command(
        &"do_user_command".to_string(),
        &vec![
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
        &"generate_n_blocks_return_new_height".to_string(),
        &vec!["1".to_string()],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_4);

    let command_out_5 = server_command(
        &"do_user_command".to_string(),
        &vec!["sync".to_string(), "recipient".to_string()],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_5);

    let command_out_6 = server_command(
        &"do_user_command".to_string(),
        &vec!["balance".to_string(), "recipient".to_string()],
        Some(&regtest_manager),
        Some(&recipient),
        Some(&faucet),
    );
    print_command(&command_out_6);
}
