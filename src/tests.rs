use super::*;

#[test]
#[should_panic(expected = "ECALL not valid, implemented non privellage instruction set")]
fn ecall_panic() {
    let prg = [0b1110011, 0, 0, 0];
    let mut computer = State::new(&prg);
    computer.run_instruction();

}

#[test]
fn ebreak() {
    let prg = [0b1110011, 0, 0b00010000, 0];

    let mut computer = State::new(&prg);
    computer.run_instruction();
}

#[test]
fn system_instruction_panic() {
    for prg in [
    [0b01110011, 164, 57, 12],
    [0b01110011, 190, 140, 125],
    [0b01110011, 58, 71, 52],
    [0b01110011, 174, 44, 216],
    [0b01110011, 9, 15, 47],
    [0b01110011, 56, 119, 13],
    [0b01110011, 144, 101, 214],
    [0b01110011, 57, 229, 142],
    [0b01110011, 208, 3, 81],
    [0b01110011, 179, 216, 174],
    [0b01110011, 72, 79, 110],
    [0b01110011, 246, 172, 52],
    [0b01110011, 24, 194, 49],
    [0b01110011, 92, 176, 135],
    [0b01110011, 207, 22, 235],
    [0b01110011, 138, 63, 193],
    [0b01110011, 21, 150, 185],
    [0b01110011, 148, 98, 35],
    [0b01110011, 12, 116, 148],
    [0b01110011, 253, 40, 119],
    [0b01110011, 222, 51, 194],
    [0b01110011, 72, 232, 186],
    [0b01110011, 42, 189, 181],
    [0b01110011, 54, 136, 36],
    [0b01110011, 156, 87, 125],
    [0b01110011, 42, 236, 194],
    [0b01110011, 70, 112, 166],
    [0b01110011, 216, 28, 117],
    [0b01110011, 211, 16, 161],
    [0b01110011, 103, 137, 33],
    [0b01110011, 55, 161, 108],
    [0b01110011, 168, 255, 202],
    [0b01110011, 227, 234, 73],
    [0b01110011, 68, 71, 126],
    [0b01110011, 191, 134, 219],
    [0b01110011, 230, 204, 185],
    [0b01110011, 57, 70, 252],
    [0b01110011, 24, 24, 56],
    [0b01110011, 40, 81, 216],
    [0b01110011, 153, 32, 197],
    [0b01110011, 98, 239, 128],
    [0b01110011, 249, 5, 58],
    [0b01110011, 175, 136, 174],
    [0b01110011, 29, 150, 222],
    [0b01110011, 41, 232, 1],
    [0b01110011, 245, 134, 91],
    [0b01110011, 130, 54, 152],
    [0b01110011, 216, 101, 78],
    [0b01110011, 96, 82, 0],
    [0b01110011, 154, 165, 250],
    [0b01110011, 5, 57, 185],
    [0b01110011, 225, 157, 122],
    [0b01110011, 15, 123, 40],
    [0b01110011, 22, 248, 35],
    [0b01110011, 251, 64, 65],
    [0b01110011, 169, 243, 84],
    [0b01110011, 68, 216, 108],
    [0b01110011, 238, 102, 159],
    [0b01110011, 103, 191, 224],
    [0b01110011, 231, 231, 61],
    [0b01110011, 64, 115, 32],
    [0b01110011, 87, 10, 117],
    [0b01110011, 151, 112, 3],
    [0b01110011, 19, 30, 117],
    [0b01110011, 18, 16, 169],
    [0b01110011, 19, 121, 142],
    [0b01110011, 172, 248, 109],
    [0b01110011, 139, 67, 242],
    [0b01110011, 63, 242, 208],
    [0b01110011, 49, 48, 49],
    [0b01110011, 169, 220, 181],
    [0b01110011, 109, 210, 239],
    [0b01110011, 222, 27, 50],
    [0b01110011, 16, 206, 173],
    [0b01110011, 205, 55, 127],
    [0b01110011, 50, 97, 229],
    [0b01110011, 36, 216, 93],
    [0b01110011, 72, 236, 127],
    [0b01110011, 224, 38, 226],
    [0b01110011, 207, 50, 25],
    [0b01110011, 167, 7, 47],
    [0b01110011, 238, 121, 85],
    [0b01110011, 105, 248, 246],
    [0b01110011, 55, 205, 30],
    [0b01110011, 43, 194, 1],
    [0b01110011, 253, 199, 135],
    [0b01110011, 238, 232, 146],
    [0b01110011, 109, 249, 79],
    [0b01110011, 49, 151, 111],
    [0b01110011, 248, 29, 31],
    [0b01110011, 192, 160, 29],
    [0b01110011, 13, 244, 80],
    [0b01110011, 15, 41, 95],
    [0b01110011, 18, 34, 120],
    [0b01110011, 104, 61, 126],
    [0b01110011, 149, 20, 41],
    [0b01110011, 108, 161, 133],
    [0b01110011, 53, 160, 122],
    [0b01110011, 68, 202, 67],
    [0b01110011, 172, 153, 234]
] {

        let mut computer = State::new(&prg);
        let result = std::panic::catch_unwind(move || computer.run_instruction());

        if result.is_ok() {
            panic!("MUST PANIC, didn't on: {:?}", prg);
        }

    }

    
}

#[test]
fn fence() {
    let prg = [0b0001111, 0, 0, 0];

    let mut computer = State::new(&prg);
    computer.run_instruction();
}

#[test]
#[should_panic(expected = "Inavlid/not implemented memory ordering instruction")]
fn invalid_fence1() {
    let prg = [0b0001111, 0b0010_0000, 0, 0];

    let mut computer = State::new(&prg);
    computer.run_instruction();
}

#[test]
#[should_panic(expected = "Inavlid/not implemented memory ordering instruction")]
fn invalid_fence2() {
    let prg = [0b0001111, 0b0100_0000, 0, 0];

    let mut computer = State::new(&prg);
    computer.run_instruction();
}

#[test]
#[should_panic(expected = "Inavlid/not implemented memory ordering instruction")]
fn invalid_fence3() {
    let prg = [0b0001111, 0b0001_0000, 0, 0];

    let mut computer = State::new(&prg);
    computer.run_instruction();
}

#[test]
fn lui() {
    let prg = [0xb7, 0x77, 0x32, 0x1e];

    let mut computer = State::new(&prg);
    computer.run_instruction();
    dbg!(&computer.registers);

    assert_eq!(computer.registers.get(15), 0x1e327 << 12);
}

#[test]
fn lui_x32() {
    let prg = [0xb7, 0x1f, 0xbf, 0x9f];
    let mut computer = State::new(&prg);
    computer.run_instruction();
    dbg!(&computer.registers);

    assert_eq!(computer.registers.get(31), 654321 << 12);
}

#[test]
fn lui_write_x0() {
    let prg = [0b00110111, 0b0000_0000, 1, 1];

    let mut computer = State::new(&prg);
    computer.run_instruction();
    dbg!(&computer.registers);

    assert_eq!(computer.registers.get(0), 0);
}

#[test]
fn test_add12i() {
    assert_eq!(add_12i(5, 6), 11);
    assert_eq!(add_12i(0b1111_1111_1111, 6), 5);
    assert_eq!(add_12i(0b1111_1111_1111, 213487012), 213487011);
    assert_eq!(add_12i(0b1111_1111_1110, 213487012), 213487010);
}

#[test]
fn basic_addi() {
    let prg = [0x13, 0x01, 0x50, 0x76];

    let mut computer = State::new(&prg);
    computer.run_instruction();

    assert_eq!(computer.registers.get(2), 0x765);
}

#[test]
fn store() {
    /*

        0:	000010b7          	lui	ra,0x1
        4:	cba98137          	lui	sp,0xcba98
        8:	76510113          	addi	sp,sp,1893
        c:	0020a023          	sw	sp,0(ra) 
        10:	0020a223          	sw	sp,4(ra)
        14:	00209423          	sh	sp,8(ra)
        18:	00208523          	sb	sp,10(ra)
        1c:	002085a3          	sb	sp,11(ra)
     */
    let prg = [
        0xB7, 0x10, 0x00, 0x00, 
        0x37, 0x81, 0xa9, 0xcb,
        0x13, 0x01, 0x51, 0x76,
        0x23, 0xA0, 0x20, 0x00, 
        0x23, 0xA2, 0x20, 0x00, 
        0x23, 0x94, 0x20, 0x00, 
        0x23, 0x85, 0x20, 0x00,
        0xA3, 0x85, 0x20, 0x00];

    let mut computer = State::new(&prg);
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();

    let to_compare = [0, 101, 135, 169, 203, 101, 135, 169, 203, 101, 135, 101, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let slice = &computer.ram[4095..4120];
    assert!(slice.eq(&to_compare));
    
    let res = std::panic::catch_unwind(move || computer.run_instruction());
    assert!(res.is_err());
}
