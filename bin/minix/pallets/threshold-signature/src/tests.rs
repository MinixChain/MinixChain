use crate::{
    mast::{Mast, XOnly},
    mock::{self, *},
    primitive::OpCode,
    Error, Pallet, ScriptHashToAddr,
};
use codec::Decode;
use core::convert::TryFrom;
use frame_support::{assert_noop, assert_ok};

fn generate_control_block() -> Vec<Vec<u8>> {
    let abc =
        hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768").unwrap();
    let ab =
        hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
    let ac =
        hex::decode("b69af178463918a181a8549d2cfbe77884852ace9d8b299bddf69bedc33f6356").unwrap();
    let bc =
        hex::decode("a20c839d955cb10e58c6cbc75812684ad3a1a8f24a503e1c07f5e4944d974d3b").unwrap();
    let pubkeys = vec![ab.clone(), ac, bc]
        .iter()
        .map(|pubkey| XOnly::try_from(pubkey.clone()).unwrap())
        .collect::<Vec<XOnly>>();
    let mast = Mast::new(pubkeys);
    let proofs = mast
        .generate_merkle_proof(&XOnly::try_from(ab).unwrap())
        .unwrap()
        .iter()
        .map(|n| n.to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut control_block = vec![abc];
    control_block.extend(proofs);
    assert_eq!(
        vec![
            "881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768",
            "e17a23050f6f6db2f4218ce9f7c14edd21c5f24818157103c5a8524d7014c0dd",
            "0bac21362eecf9223bc477d6dfbbe02066a911eba752faedb26d881c466ea80f",
        ],
        control_block.iter().map(hex::encode).collect::<Vec<_>>()
    );
    control_block
}

#[test]
fn pass_script_should_work() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let control_block = generate_control_block();
        let message = b"We are legion!".to_vec();
        assert_eq!("576520617265206c6567696f6e21", hex::encode(&message));
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash.clone()));
        assert_eq!(ScriptHashToAddr::<Test>::get(script_hash), addr);
    });
}

#[test]
fn pass_script_with_invalid_signature() {
    new_test_ext().execute_with(|| {
        let who = 1;
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861")
            .unwrap();
        let tweaked =
            &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049")
                .unwrap();
        let addr =
            <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = vec![1; 64];
        let control_block = generate_control_block();
        let message = b"We are legion!".to_vec();
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_noop!(
            Pallet::<Test>::pass_script(
                Origin::signed(who),
                addr,
                signature_ab,
                ab,
                control_block,
                message,
                script_hash
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn exec_script_should_work() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let control_block = generate_control_block();
        let message = b"We are legion!".to_vec();
        assert_eq!("576520617265206c6567696f6e21", hex::encode(&message));
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash));
        assert_ok!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10)));
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
    });
}

#[test]
fn exec_script_mismatch_time_lock() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let control_block = generate_control_block();
        let message = b"We are legion!".to_vec();
        assert_eq!("576520617265206c6567696f6e21", hex::encode(&message));
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (2, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash));
        assert_noop!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (2, 10)), Error::<Test>::MisMatchTimeLock);
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 10);
    });
}

#[test]
fn exec_script_no_pass_script() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let control_block = generate_control_block();
        let message = b"We are legion!".to_vec();
        assert_eq!("576520617265206c6567696f6e21", hex::encode(&message));
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash));
        assert_ok!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10)));
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
        assert_noop!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10)), Error::<Test>::NoPassScript);
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
    });
}
