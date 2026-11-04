use super::*;

#[test]
#[should_panic(expected = "ECALL not valid, implemented non privellage instruction set")]
fn ecall_panic() {
    let prg = [0b1110011];
    let mut computer = State::new(&prg);
    computer.run_instruction();

}