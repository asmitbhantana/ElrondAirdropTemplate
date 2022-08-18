#[test]
fn ping_pong_call_ping_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-ping.scen.json");
}

#[test]
fn ping_pong_call_ping_second_user_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-ping-second-user.scen.json");
}

#[test]
fn ping_pong_call_ping_twice_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-ping-twice.scen.json");
}

#[test]
fn ping_pong_call_ping_wrong_amount_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-ping-wrong-amount.scen.json");
}

#[test]
fn ping_pong_call_pong_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-pong.scen.json");
}

#[test]
fn ping_pong_call_pong_before_deadline_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-pong-before-deadline.scen.json");
}

#[test]
fn ping_pong_call_pong_twice_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-pong-twice.scen.json");
}

#[test]
fn ping_pong_call_pong_without_ping_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-call-pong-without-ping.scen.json");
}

#[test]
fn ping_pong_init_go() {
    elrond_wasm_debug::mandos_go("mandos/airdrop-init.scen.json");
}
