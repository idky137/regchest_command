// integration_test.rs
// use: holds tests for regchest_command
// authers: idky137
//

use regchest_command::regchest_command::{regchest_command, CommandInput, CommandOutput};
use regchest_command::regchest_command_util::print_command;
use zingo_testutils::regtest::{ChildProcessHandler, RegtestManager};
use zingoconfig::RegtestNetwork;
use zingolib::{lightclient::LightClient, wallet::Pool};

#[test]
#[ignore]
fn test_basic_no_panic() {
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
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);
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
    let command_output_2a = regchest_command(command_str_2a, &command_inputs_2a);
    print_command(&command_output_2a);

    // --- Test 2b: DoUserCommand:Adresses
    println!("Test entry 2b: Calling run_command::do_user_command::addresses:");
    let command_str_2b = "do_user_command";
    let command_inputs_2b =
        CommandInput::DoUserCommand(("addresses".to_string(), vec![], &recipient));
    let command_output_2b = regchest_command(command_str_2b, &command_inputs_2b);
    print_command(&command_output_2b);

    // --- Test 3a: GenerateNBlocksReturnNewHeight(0)
    println!("Test entry 3a: Calling run_command::generate_n_blocks_return_new_height(0):");
    let command_str_3a = "generate_n_blocks_return_new_height";
    let command_inputs_3a = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 0);
    let command_output_3a = regchest_command(command_str_3a, &command_inputs_3a);
    print_command(&command_output_3a);

    // --- Test 3b: GenerateNBlocksReturnNewHeight(10)
    println!("Test entry 3b: Calling run_command::generate_n_blocks_return_new_height(10):");
    let command_str_3b = "generate_n_blocks_return_new_height";
    let command_inputs_3b = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 10);
    let command_output_3b = regchest_command(command_str_3b, &command_inputs_3b);
    print_command(&command_output_3b);
}
