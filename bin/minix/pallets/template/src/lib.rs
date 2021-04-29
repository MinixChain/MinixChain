#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use codec::{Encode, Decode};
use sp_runtime::{
	RuntimeDebug,
	traits::{
		StaticLookup, Saturating,
	},
};

// TODO: rename pallet-template to pallet-coming-id

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub type Cid = u64;
pub type BondType = u16;

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct BondData {
	pub bond_type: BondType,
	pub data: Vec<u8>
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct CidDetails<AccountId> {
	pub owner: AccountId,
	pub bonds: Vec<BondData>,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{transactional, dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The number of blocks over which a claim request period.
		type ClaimValidatePeriod: Get<Self::BlockNumber>;
		/// Max number of cids to approve/disapprove per extrinsic call.
		type CidsLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn cids)]
	pub type Cids<T: Config> = StorageMap<_, Blake2_128Concat, Cid, CidDetails<T::AccountId>>;

	#[pallet::storage]
	pub type NextCommonCid<T> = StorageValue<_, Cid, ValueQuery>;

	#[pallet::storage]
	pub type ClaimQueue<T: Config> = StorageValue<_, Vec<(Cid, T::AccountId, T::BlockNumber)>, ValueQuery>;

	#[pallet::storage]
	pub type WaitQueue<T> = StorageValue<_, Vec<Cid>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// receipt, cid
		Registered(T::AccountId, Cid),
		// user, cid, expired
		Claiming(T::AccountId, Cid, T::BlockNumber),
		// cid_start, cid_end
		Approved(Cid, Cid),
		// cid_start, cid_end
		DisApproved(Cid, Cid),
		// owner, receipt, cid
		Transfered(T::AccountId, T::AccountId, Cid),
		// receipt, cid
		ForceTransfered(T::AccountId, Cid),
		// owner, cid, bond_type
		Bonded(T::AccountId, Cid, BondType),
		// owner, cid, bond_type
		UnBonded(T::AccountId, Cid, BondType)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		OnlyReservedAndCommunityCid,
		OnlyCommunityAndCommonCid,
		InvalidCid,
		RequireOwner,
		UndistributedCid,
		InvalidCidEnd,
		OutOfCidsLimit,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		// first register(1, alice): alice is the owner of cid 1 and then bond some data,
		// then register(1, bob): alice unbond all data and bob is the new owner of cid 1.
		#[pallet::weight(1000)]
		pub fn register(origin: OriginFor<T>, cid: Cid, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(
				Self::is_reserved(cid) || Self::is_community(cid),
				Error::<T>::OnlyReservedAndCommunityCid
			);
			let receipt = T::Lookup::lookup(receipt)?;

			Cids::<T>::try_mutate_exists(cid, |details|{
				*details = Some(CidDetails{
					owner: receipt.clone(),
					bonds: vec![],
				});

				Self::deposit_event(Event::Registered(receipt, cid));

				Ok(())
			})

		}

		#[pallet::weight(1000)]
		#[transactional]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let (common_cid, need_update) = Self::get_common_cid();
			let expired = frame_system::Pallet::<T>::block_number().saturating_add(T::ClaimValidatePeriod::get());

			ClaimQueue::<T>::try_mutate::<_, (Cid, T::AccountId, T::BlockNumber), _>(|reqs| {
				reqs.push((common_cid, who.clone(), expired));

				Ok(())
			});

			// Cids::<T>::try_mutate_exists(common_cid, |details|{
			// 	*details = Some(CidDetails{
			// 		owner: who.clone(),
			// 		bonds: vec![],
			// 	});
			//
			//
			// 	Ok(())
			// });


			if need_update {
				let next_common_cid = common_cid + 1;
				NextCommonCid::<T>::put(&next_common_cid);
			}

			Self::deposit_event(Event::Claiming(who, common_cid, expired));

			Ok(())
		}

		// [cid_start,cid_end)
		#[pallet::weight(1000)]
		pub fn approve(origin: OriginFor<T>, cid_start: Cid, cid_end: Cid) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(cid_end >= cid_start, Error::<T>::InvalidCidEnd);
			ensure!(cid_end - cid_start <= (T::CidsLimit::get()) as u64, Error::<T>::OutOfCidsLimit);


			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn disapprove(origin: OriginFor<T>, cid_start: Cid, cid_end: Cid) -> DispatchResult {
			Ok(())
		}

		// transfer to self equal unbond all
		#[pallet::weight(1000)]
		pub fn transfer(origin: OriginFor<T>, cid: Cid, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			let is_admin = if Self::is_reserved(cid) {
				ensure_root(origin.clone())?;
				true
			} else {
				ensure!(Self::is_community(cid) || Self::is_common(cid), Error::<T>::OnlyCommunityAndCommonCid);
				false
			};
			let who = ensure_signed(origin).unwrap_or_default();
			let receipt = T::Lookup::lookup(receipt)?;

			Cids::<T>::try_mutate_exists(cid, |details|{
				let mut detail = details.as_mut().ok_or(Error::<T>::UndistributedCid)?;

				ensure!(is_admin || detail.owner == who, Error::<T>::RequireOwner);

				detail.owner = receipt.clone();
				detail.bonds = vec![];

				if is_admin {
					Self::deposit_event(Event::ForceTransfered(receipt, cid))
				} else {
					Self::deposit_event(Event::Transfered(who, receipt, cid));
				}

				Ok(())
			})
		}

		#[pallet::weight(1000)]
		pub fn bond(origin: OriginFor<T>, cid: Cid, bond_data: BondData) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(cid), Error::<T>::InvalidCid);

			Cids::<T>::try_mutate_exists(cid, |details|{
				let detail = details.as_mut().ok_or(Error::<T>::UndistributedCid)?;
				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				let bond_type = bond_data.bond_type;

				detail.bonds.push(bond_data);

				Self::deposit_event(Event::Bonded(who, cid, bond_type));

				Ok(())
			})
		}

		#[pallet::weight(1000)]
		pub fn unbond(origin: OriginFor<T>, cid: Cid, bond_type: BondType) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(cid), Error::<T>::InvalidCid);

			Cids::<T>::try_mutate_exists(cid, |details|{
				let detail = details.as_mut().ok_or(Error::<T>::UndistributedCid)?;
				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				detail.bonds.retain(|bond| bond.bond_type == bond_type);

				Self::deposit_event(Event::UnBonded(who, cid, bond_type));

				Ok(())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	fn is_reserved(cid: Cid) -> bool {
		if cid >= 1 && cid < 100_000 {
			return true
		}

		false
	}

	fn is_community(cid: Cid) -> bool {
		if cid >= 100_000 && cid < 1_000_000 {
			return true
		}

		false
	}

	fn is_common(cid: Cid) -> bool {
		if cid >= 1_000_000 && cid < 1_000_000_000_000 {
			return true
		}

		false
	}

	fn is_valid(cid: Cid) -> bool {
		if Self::is_reserved(cid) || Self::is_community(cid) || Self::is_common(cid) {
			return true
		}

		false
	}

	fn get_common_cid() -> (Cid, bool) {
		let cid = NextCommonCid::<T>::get();

		// Initialize from 1_000_000
		if cid == 0 {
			(1_000_000, true)
		} else {
			(cid, true)
		}
	}

	pub fn get_bond(cid: Cid) -> Option<CidDetails<T::AccountId>> {
		Self::cids(cid)
	}
}
