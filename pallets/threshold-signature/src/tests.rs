use crate::{
    mock::{self, *},
    primitive::OpCode,
    Error, Pallet, ScriptHashToAddr,
};
use codec::Decode;
use frame_support::{assert_noop, assert_ok};

#[test]
fn pass_script_should_work() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash.clone()));
        assert_eq!(ScriptHashToAddr::<Test>::get(script_hash), addr);
    });
}

#[test]
fn pass_script_with_invalid_signature() {
    new_test_ext().execute_with(|| {
        let who = 1;
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b")
            .unwrap();
        let tweaked =
            &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719")
                .unwrap();
        let addr =
            <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = vec![1; 64];
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
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
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
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
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
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
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab, ab, control_block, message, script_hash));
        assert_ok!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10)));
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
        assert_noop!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10)), Error::<Test>::NoPassScript);
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
    });
}

#[test]
fn exec_script_no_dump_signature() {
    new_test_ext().execute_with(|| {
        // https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156
        let who = 1;
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <mock::Test as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let message = 666666;
        let script_hash = Pallet::<Test>::compute_script_hash(who, OpCode::Transfer, 10, (0, 10_000_000));
        assert_ok!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab.clone(), ab.clone(), control_block.clone(), message, script_hash.clone()));
        assert_noop!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab.clone(), ab.clone(), control_block.clone(), message, script_hash.clone()), Error::<Test>::ExistedSignature);
        frame_system::Pallet::<Test>::set_block_number(666667);
        assert_ok!(Pallet::<Test>::exec_script(Origin::signed(who), who, OpCode::Transfer, 10, (0, 10_000_000)));
        assert_noop!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab.clone(), ab.clone(), control_block.clone(), message, script_hash.clone()), Error::<Test>::ExpiredSignature);
        assert_noop!(Pallet::<Test>::pass_script(Origin::signed(who), addr, signature_ab.clone(), ab.clone(), control_block.clone(), message+1, script_hash.clone()), Error::<Test>::InvalidSignature);
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
        assert_eq!(pallet_balances::Pallet::<Test>::free_balance(who), 20);
    });
}
