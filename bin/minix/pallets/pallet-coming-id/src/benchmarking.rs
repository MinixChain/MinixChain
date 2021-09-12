//! Benchmarking setup for pallet-coming-id

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{whitelisted_caller, benchmarks};
use frame_system::RawOrigin;
use sp_std::vec;

fn bond_data(t: u16, b: u32) -> BondData {
    BondData{
        bond_type: t,
        data: vec![1; b as usize].into(),
    }
}

fn update_bond<T: Config>(cid: Cid, t: u16, b: u32) -> Result<(), &'static str> {
    for i in 0..b {
        Distributed::<T>::mutate_exists(cid, |details|{
            if let Some(detail) = details {
                detail.bonds.push(bond_data(t, i));
            }
        })
    }

    Ok(())
}

benchmarks! {
    register {
        let admin: T::AccountId = whitelisted_caller();
        let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(admin.clone());
        let claim_cid: Cid = 1000000;
    }: register(RawOrigin::Signed(admin), claim_cid, recipient_lookup)
    verify {
        assert!(Distributed::<T>::get(claim_cid).is_some());
    }

    bond {
        let common_user: T::AccountId = whitelisted_caller();
        let claim_cid: Cid = 1000000;

        let _ = Distributed::<T>::try_mutate_exists::<_,_,Error<T>,_>(claim_cid, |details|{
            *details = Some(CidDetails{
                owner: common_user.clone(),
                bonds: Vec::new(),
                card: Bytes::from(Vec::new())
            });

            Ok(())
        })?;

        let b in 0 .. (T::MaxCardSize::get() / 1024) => {
            update_bond::<T>(claim_cid, 1u16, b)?;
        };

    }: bond(RawOrigin::Signed(common_user.clone()), claim_cid, bond_data(1u16, b))
    verify {
        let option = Distributed::<T>::get(claim_cid);
        assert!(option.is_some());

        let cid_details = option.unwrap();
        assert_eq!(cid_details.owner, common_user);
    }

    unbond {
        let common_user: T::AccountId = whitelisted_caller();
        let claim_cid: Cid = 1000000;
        let bond_data = BondData{
            bond_type: 1u16,
            data: Bytes::from(b"benchmark".to_vec()),
        };

        let mut bonds: Vec<BondData> = Vec::new();
        bonds.push(bond_data);

        let _ = Distributed::<T>::try_mutate_exists::<_,_,Error<T>,_>(claim_cid, |details|{
            *details = Some(CidDetails{
                owner: common_user.clone(),
                bonds: bonds,
                card: Bytes::from(Vec::new())
            });

            Ok(())
        })?;

    }: unbond(RawOrigin::Signed(common_user.clone()), claim_cid, 1u16)
    verify {
        let option = Distributed::<T>::get(claim_cid);
        assert!(option.is_some());

        let cid_details = option.unwrap();
        assert_eq!(cid_details.owner, common_user);
    }
}
