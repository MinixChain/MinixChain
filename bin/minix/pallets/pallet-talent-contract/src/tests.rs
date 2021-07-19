use crate::mock::*;
use frame_support::{assert_noop, assert_ok, Hashable};
use sp_core::H256;
use hex_literal::hex;
use sp_core::hexdisplay::HexDisplay;

const PARTY_A: u64 = 1;
const PARTY_B: u64 = 2;
const NEW_PARTY_B: u64 = 3;
const DIGEST: &[u8] = &hex!["554b8dd03b68100c5d7d370a478e9583b30cb4284183761447c7fd922fd32331"];
const SECRET: &[u8] = b"test_secret";
const SECRET_BLAKE2_256: &[u8] = &hex!["ccd93d849da281fc77dd2c0b812d1d7804a5a849708ac989c07956839251e6f8"];
const _ENCODE_SECRET_BLAKE2_256: &[u8] = &hex!["56997d0ee2a807e463f3f109759450ec7723cc3714fd57f0d88ac2f26ffdbf48"];

#[test]
fn secret_hash() {
    let raw_encode_hash = SECRET.to_vec().blake2_256();
    let secret_hash = HexDisplay::from(&raw_encode_hash);
    println!("{:?}", secret_hash);

    let raw_hash = sp_io::hashing::blake2_256(SECRET);
    let secret_hash = HexDisplay::from(&raw_hash);
    println!("{:?}", secret_hash);
}

#[test]
fn draft_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToSign(secret_hash));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, None);
    })
}

#[test]
fn sign_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToSign(secret_hash));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, None);

        assert_ok!(TalentContract::sign(
            Origin::signed(PARTY_B),
            digest.clone(),
            SECRET.to_vec(),
        ));
        expect_event(TalentContractEvent::Signed(digest.clone(), PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest);
        assert_eq!(status, Status::CanTransfer);
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));
    })
}

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToSign(secret_hash));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, None);

        assert_ok!(TalentContract::sign(
            Origin::signed(PARTY_B),
            digest.clone(),
            SECRET.to_vec(),
        ));
        expect_event(TalentContractEvent::Signed(digest.clone(), PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::CanTransfer);
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));

        assert_ok!(TalentContract::transfer(
            Origin::signed(PARTY_A),
            digest.clone(),
            NEW_PARTY_B,
        ));
        expect_event(TalentContractEvent::Transferring(digest.clone(), NEW_PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToConfirm(Some(PARTY_B), NEW_PARTY_B));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));
    })
}

#[test]
fn confirm_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToSign(secret_hash));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, None);

        assert_ok!(TalentContract::sign(
            Origin::signed(PARTY_B),
            digest.clone(),
            SECRET.to_vec(),
        ));
        expect_event(TalentContractEvent::Signed(digest.clone(), PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::CanTransfer);
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));

        assert_ok!(TalentContract::transfer(
            Origin::signed(PARTY_A),
            digest.clone(),
            NEW_PARTY_B,
        ));
        expect_event(TalentContractEvent::Transferring(digest.clone(), NEW_PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToConfirm(Some(PARTY_B), NEW_PARTY_B));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));

        assert_ok!(TalentContract::confirm(
            Origin::signed(PARTY_B),
            digest.clone(),
        ));
        expect_event(TalentContractEvent::Confirmed(digest.clone()));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::CanTransfer);
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(NEW_PARTY_B));
    })
}

#[test]
fn revoke_draft_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToSign(secret_hash));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, None);

        assert_ok!(TalentContract::revoke(
            Origin::signed(PARTY_A),
            digest.clone(),
        ));
        expect_event(TalentContractEvent::Revoked(digest.clone()));

        assert!(TalentContract::get_contract(digest).is_none());
    })
}

#[test]
fn revoke_transfer_should_work() {
    new_test_ext().execute_with(|| {
        let digest = H256::from_slice(DIGEST);
        let secret_hash = H256::from_slice(SECRET_BLAKE2_256);

        assert_ok!(TalentContract::draft(
            Origin::signed(PARTY_A),
            digest.clone(),
            secret_hash,
            true
        ));
        expect_event(TalentContractEvent::Drafted(digest.clone(), PARTY_A));

        assert_ok!(TalentContract::sign(
            Origin::signed(PARTY_B),
            digest.clone(),
            SECRET.to_vec(),
        ));
        expect_event(TalentContractEvent::Signed(digest.clone(), PARTY_B));

        assert_ok!(TalentContract::transfer(
            Origin::signed(PARTY_A),
            digest.clone(),
            NEW_PARTY_B,
        ));
        expect_event(TalentContractEvent::Transferring(digest.clone(), NEW_PARTY_B));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::WaitingToConfirm(Some(PARTY_B), NEW_PARTY_B));
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));

        assert_ok!(TalentContract::revoke(
            Origin::signed(PARTY_A),
            digest.clone(),
        ));
        expect_event(TalentContractEvent::Revoked(digest.clone()));

        let Contract{
            digest: hash,
            status,
            updated,
            party_a,
            party_b
        } = TalentContract::get_contract(digest.clone()).unwrap();

        assert_eq!(hash, digest.clone());
        assert_eq!(status, Status::CanTransfer);
        assert_eq!(updated, 1);
        assert_eq!(party_a, Some(PARTY_A));
        assert_eq!(party_b, Some(PARTY_B));
    })
}
