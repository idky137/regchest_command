// testutil_test.rs
// use: holds tests for generate_n_blocks_return_new_height
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
fn test_generate_n_blocks_return_new_height() {
    let regtest_network = RegtestNetwork::all_upgrades_active();

    let command_str_1 = "scenarios::unfunded_client";
    let command_inputs_1 = CommandInput::UnfundedClient(regtest_network);
    let command_output_1 = regchest_command(command_str_1, &command_inputs_1);

    let regtest_manager: RegtestManager;
    let _cph: ChildProcessHandler;
    let _recipient: LightClient;
    let height_out_1: u32;
    let height_out_2: u32;

    match command_output_1 {
        CommandOutput::UnfundedClient(regtest_manager_v, cph_v, recipient_v) => {
            regtest_manager = regtest_manager_v;
            _cph = cph_v;
            _recipient = recipient_v
        }
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_str_2 = "generate_n_blocks_return_new_height";
    let command_inputs_2 = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 0);
    let command_output_2 = regchest_command(command_str_2, &command_inputs_2);

    match command_output_2 {
        CommandOutput::GenerateNBlocksReturnNewHeight(height_out_v) => height_out_1 = height_out_v,
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    let command_str_3 = "generate_n_blocks_return_new_height";
    let command_inputs_3 = CommandInput::GenerateNBlocksReturnNewHeight(&regtest_manager, 10);
    let command_output_3 = regchest_command(command_str_3, &command_inputs_3);

    match command_output_3 {
        CommandOutput::GenerateNBlocksReturnNewHeight(height_out_v) => height_out_2 = height_out_v,
        _ => {
            panic!("Error: Incorrect output");
        }
    }

    assert_eq!(height_out_1 + 10, height_out_2);
}
