// regchest_command:
// Command library for building custom scenarios remotely.
// authers: idky137
//

// --- main:
// --- usage example.
use regchest_command::regchest_command::CommandOutput;
use regchest_command::regchest_command_util::server_print;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingolib::lightclient::LightClient;

fn main() {
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
