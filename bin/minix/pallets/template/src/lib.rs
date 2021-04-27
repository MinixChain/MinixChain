#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use codec::{Encode, Decode};
use sp_runtime::{
	RuntimeDebug,
	traits::StaticLookup,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub type Did = u64;
pub type BondType = u16;

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct BondData {
	pub bond_type: BondType,
	pub data: Vec<u8>
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct DidDetails<AccountId> {
	pub owner: AccountId,
	pub bonds: Vec<BondData>,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn dids)]
	pub type Dids<T: Config> = StorageMap<_, Blake2_128Concat, Did, DidDetails<T::AccountId>>;

	#[pallet::storage]
	pub type NextCommonDid<T> = StorageValue<_, Did, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// receipt, did
		Register(T::AccountId, Did),
		// user, did
		Claim(T::AccountId, Did),
		// owner, receipt, did
		Transfer(T::AccountId, T::AccountId, Did),
		// receipt, did
		ForceTransfer(T::AccountId, Did),
		// owner, did, bond_type
		Bond(T::AccountId, Did, BondType),
		// owner, did, bond_type
		UnBond(T::AccountId, Did, BondType)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		OnlyReservedAndCommunityDid,
		OnlyCommunityAndCommonDid,
		InvalidDid,
		RequireOwner,
		UnassigneDid,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn register(origin: OriginFor<T>, did: Did, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(
				Self::is_reserved(did) || Self::is_community(did),
				Error::<T>::OnlyReservedAndCommunityDid
			);
			let receipt = T::Lookup::lookup(receipt)?;

			Dids::<T>::try_mutate_exists(did, |details|{
				*details = Some(DidDetails{
					owner: receipt.clone(),
					bonds: vec![],
				});

				Self::deposit_event(Event::Register(receipt, did));
				Ok(())
			})

		}

		#[pallet::weight(1000)]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut common_did = Self::get_next_did();

			Dids::<T>::try_mutate_exists(common_did, |details|{
				*details = Some(DidDetails{
					owner: who.clone(),
					bonds: vec![],
				});

				common_did += 1;
				NextCommonDid::<T>::put(&common_did);

				Self::deposit_event(Event::Claim(who, common_did));
				Ok(())
			})
		}

		#[pallet::weight(1000)]
		pub fn transfer(origin: OriginFor<T>, did: Did, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			let is_admin = if Self::is_reserved(did) {
				ensure_root(origin.clone())?;
				true
			} else {
				ensure!(Self::is_community(did) || Self::is_common(did), Error::<T>::OnlyCommunityAndCommonDid);
				false
			};
			let who = ensure_signed(origin).unwrap_or_default();
			let receipt = T::Lookup::lookup(receipt)?;

			Dids::<T>::try_mutate_exists(did, |details|{
				let mut detail = details.as_mut().ok_or(Error::<T>::UnassigneDid)?;

				ensure!(is_admin || detail.owner == who, Error::<T>::RequireOwner);

				detail.owner = receipt.clone();
				detail.bonds = vec![];

				if is_admin {
					Self::deposit_event(Event::ForceTransfer(receipt, did))
				} else {
					Self::deposit_event(Event::Transfer(who, receipt, did));
				}

				Ok(())
			})
		}

		#[pallet::weight(1000)]
		pub fn bond(origin: OriginFor<T>, did: Did, bond_data: BondData) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(did), Error::<T>::InvalidDid);

			Dids::<T>::try_mutate_exists(did, |details|{
				let detail = details.as_mut().ok_or(Error::<T>::UnassigneDid)?;
				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				let bond_type = bond_data.bond_type;

				detail.bonds.push(bond_data);

				Self::deposit_event(Event::Bond(who, did, bond_type));

				Ok(())
			})
		}

		#[pallet::weight(1000)]
		pub fn unbond(origin: OriginFor<T>, did: Did, bond_type: BondType) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(did), Error::<T>::InvalidDid);

			Dids::<T>::try_mutate_exists(did, |details|{
				let detail = details.as_mut().ok_or(Error::<T>::UnassigneDid)?;
				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				detail.bonds.retain(|bond| bond.bond_type == bond_type);

				Self::deposit_event(Event::UnBond(who, did, bond_type));

				Ok(())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	fn is_reserved(did: Did) -> bool {
		if did >= 1 && did < 100_000 {
			return true
		}

		false
	}

	fn is_community(did: Did) -> bool {
		if did >= 100001 && did < 1_000_000 {
			return true
		}

		false
	}

	fn is_common(did: Did) -> bool {
		if did >= 1_000_001 && did <= 1_000_000_000_000 {
			return true
		}

		false
	}

	fn is_valid(did: Did) -> bool {
		if Self::is_reserved(did) ||
			Self::is_community(did) ||
			Self::is_common(did) {
			return true
		}

		false
	}

	fn get_next_did() -> Did {
		let mut common_did = NextCommonDid::<T>::get();

		// Initialize from 1000000
		if common_did == 0 {
			common_did = 1000000
		}
		common_did
	}

	pub fn get_bond(did: Did) -> Option<DidDetails<T::AccountId>> {
		Self::dids(did)
	}
}
