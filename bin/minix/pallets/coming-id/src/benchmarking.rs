//! Benchmarking setup for pallet-coming-id

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{
    benchmarks, whitelisted_caller, impl_benchmark_test_suite, account,
};
#[allow(unused)]
use crate::Pallet as ComingId;

const SEED: u32 = 0;

benchmarks! {
	register {
	    let c in 1..100000;
	    let admin: T::AccountId = whitelisted_caller();
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(admin.clone());
	}: register(RawOrigin::Signed(admin), c as Cid, recipient_lookup)
	verify {
		assert!(Distributed::<T>::get(c as Cid).is_some());
	}

	claim {
	    let common_user: T::AccountId = account("common_user", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(common_user.clone());
	    let claim_cid: Cid = 1000000;
	}: claim(RawOrigin::Signed(common_user), recipient_lookup)
	verify {
		assert!(Distributing::<T>::get().contains_key(&claim_cid));
	}

	approve {
	    let admin: T::AccountId = whitelisted_caller();
	    let common_user: T::AccountId = account("common_user", 0, SEED);
	    let claim_cid: Cid = 1000000;
	    let expired = T::ClaimValidatePeriod::get();

	    let _ = Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs| {
            reqs.insert(claim_cid, (common_user.clone(), expired));
            Ok(())
	    })?;

	}: approve(RawOrigin::Signed(admin), claim_cid, claim_cid + T::CidsLimit::get() as u64)
	verify {
	    assert!(Distributing::<T>::get().is_empty());
	    assert!(Distributed::<T>::get(claim_cid).is_some());
	}

	disapprove {
	    let admin: T::AccountId = whitelisted_caller();
	    let common_user: T::AccountId = account("common_user", 0, SEED);
	    let claim_cid: Cid = 1000000;
	    let expired = T::ClaimValidatePeriod::get();

	    let _ = Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs| {
            reqs.insert(claim_cid, (common_user.clone(), expired));
            Ok(())
	    })?;

	}: disapprove(RawOrigin::Signed(admin), claim_cid, claim_cid + T::CidsLimit::get() as u64)
	verify {
	    assert!(Distributing::<T>::get().is_empty());
	    assert!(Distributed::<T>::get(claim_cid).is_none());
	    assert!(WaitDistributing::<T>::get().contains(&claim_cid));
	}

	transfer {
	    let common_user: T::AccountId = account("common_user", 0, SEED);
	    let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let claim_cid: Cid = 1000000;

	    let _ = Distributed::<T>::try_mutate_exists::<_,_,Error<T>,_>(claim_cid, |details|{
		    *details = Some(CidDetails{
		        owner: common_user.clone(),
		        bonds: vec![],
		    });

		    Ok(())
		})?;

	}: transfer(RawOrigin::Signed(common_user), claim_cid, recipient_lookup)
	verify {
	    let option = Distributed::<T>::get(claim_cid);
	    assert!(option.is_some());

	    let cid_details = option.unwrap();
	    assert_eq!(cid_details.owner, recipient);
	}

	bond {
	    let common_user: T::AccountId = account("common_user", 0, SEED);
	    let claim_cid: Cid = 1000000;

	    let _ = Distributed::<T>::try_mutate_exists::<_,_,Error<T>,_>(claim_cid, |details|{
		    *details = Some(CidDetails{
		        owner: common_user.clone(),
		        bonds: vec![],
		    });

		    Ok(())
		})?;

		let bond_data = BondData{
			bond_type:1u16,
			data:b"benchmark".to_vec(),
		};

	}: bond(RawOrigin::Signed(common_user.clone()), claim_cid, bond_data.clone())
	verify {
	    let option = Distributed::<T>::get(claim_cid);
	    assert!(option.is_some());

	    let cid_details = option.unwrap();
	    assert_eq!(cid_details.owner, common_user);
	    assert_eq!(cid_details.bonds, vec![bond_data]);
	}

	unbond {
	    let common_user: T::AccountId = account("common_user", 0, SEED);
	    let claim_cid: Cid = 1000000;
        let bond_data = BondData{
			bond_type:1u16,
			data:b"benchmark".to_vec(),
		};

	    let _ = Distributed::<T>::try_mutate_exists::<_,_,Error<T>,_>(claim_cid, |details|{
		    *details = Some(CidDetails{
		        owner: common_user.clone(),
		        bonds: vec![bond_data.clone()],
		    });

		    Ok(())
		})?;

	}: unbond(RawOrigin::Signed(common_user.clone()), claim_cid, 1u16)
	verify {
		let option = Distributed::<T>::get(claim_cid);
	    assert!(option.is_some());

	    let cid_details = option.unwrap();
	    assert_eq!(cid_details.owner, common_user);
	    assert_eq!(cid_details.bonds, vec![]);
	}
}

impl_benchmark_test_suite!(
	ComingId,
	crate::mock::new_test_ext(frame_benchmarking::whitelisted_caller()),
	crate::mock::Test,
);
