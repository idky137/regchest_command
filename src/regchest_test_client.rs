// regchest_test_client.rs
// use: testing regchest:regchest_server and usage example.
// authers: idkky137
//

use regchest_command::regchest_client_util::regchest_send;

// - main
// - usage example: regchest_send
fn main() {
    regchest_send("scenarios::faucet_recipient");

    regchest_send("do_user_command balance recipient");

    regchest_send("do_user_command send faucet recipient 100000 unified");

    regchest_send("generate_n_blocks_return_new_height 10");

    regchest_send("do_user_command sync recipient");

    regchest_send("do_user_command balance recipient");

    regchest_send("do_user_command send faucet recipient 100000 sapling");

    regchest_send("generate_n_blocks_return_new_height 10");

    regchest_send("do_user_command sync recipient");

    regchest_send("do_user_command balance recipient");

    regchest_send("do_user_command shield recipient all");

    regchest_send("generate_n_blocks_return_new_height 10");

    regchest_send("do_user_command sync recipient");

    regchest_send("do_user_command balance recipient");

    regchest_send("do_user_command notes recipient all");

    regchest_send("do_user_command summaries recipient");

    println!("Ending connection with server and closing program");
    regchest_send("endsession");
}
