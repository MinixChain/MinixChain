#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use codec::{Encode, Decode};
use sp_runtime::{
	RuntimeDebug,
	traits::{
		StaticLookup, Saturating,
	},
};
use frame_support::sp_std::{
	collections::{
		btree_map::BTreeMap,
		vec_deque::VecDeque
	},
	ops::Bound::{Included,Excluded},
};

// TODO: rename pallet-template to pallet-coming-id
// TODO: add admin account

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
	#[pallet::getter(fn distributed)]
	pub type Distributed<T: Config> = StorageMap<_, Blake2_128Concat, Cid, CidDetails<T::AccountId>>;

	#[pallet::storage]
	pub type Distributing<T: Config> = StorageValue<_, BTreeMap<Cid, (T::AccountId, T::BlockNumber)>, ValueQuery>;

	#[pallet::storage]
	pub type WaitDistributing<T> = StorageValue<_, VecDeque<Cid>, ValueQuery>;

	#[pallet::storage]
	pub type NextCommonCid<T> = StorageValue<_, Cid, ValueQuery>;

	/// The `AccountId` of the sudo key.
	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub(super) type Key<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// The `AccountId` of the admin key.
		pub admin_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				admin_key: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Key<T>>::put(&self.admin_key);
		}
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// receipt, cid
		Registered(T::AccountId, Cid),
		// user, cid, expired
		Claiming(T::AccountId, Cid, T::BlockNumber),
		// receipt, cid
		ForceClaimed(T::AccountId, Cid),
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

	#[pallet::error]
	pub enum Error<T> {
		OnlyReservedAndCommunityCid,
		OnlyCommunityAndCommonCid,
		InvalidCid,
		RequireAdmin,
		RequireOwner,
		DistributedCid,
		UndistributedCid,
		InvalidCidEnd,
		OutOfCidsLimit,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumberFor<T>) -> Weight {
			Self::do_initialize(now);
			0
		}
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		// first register(1, alice): alice is the owner of cid 1 and then bond some data,
		// then register(1, bob): alice unbond all data and bob is the new owner of cid 1.
		#[pallet::weight(100_000)]
		pub fn register(origin: OriginFor<T>, cid: Cid, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			ensure!(ensure_signed(origin)? == Self::admin_key(), Error::<T>::RequireAdmin);
			ensure!(
				Self::is_reserved(cid) || Self::is_community(cid),
				Error::<T>::OnlyReservedAndCommunityCid
			);
			if Self::is_community(cid) {
				ensure!(
					Self::is_distributed(cid),
					Error::<T>::DistributedCid
				);
			}
			let receipt = T::Lookup::lookup(receipt)?;

			Distributed::<T>::try_mutate_exists(cid, |details|{
				*details = Some(CidDetails{
					owner: receipt.clone(),
					bonds: vec![],
				});

				if Self::is_distributed(cid) {
					Self::deposit_event(Event::ForceTransfered(receipt, cid))
				} else {
					Self::deposit_event(Event::Registered(receipt, cid));
				}

				Ok(())
			})

		}

		#[pallet::weight(100_000)]
		#[transactional]
		pub fn claim(origin: OriginFor<T>, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let current_height = frame_system::Pallet::<T>::block_number();
			let (common_cid, need_update) = Self::get_common_cid();

			if who == Self::admin_key() {
				let receipt = T::Lookup::lookup(receipt)?;
				Distributed::<T>::try_mutate_exists::<_, _, Error<T>, _>(common_cid, |details|{
					*details = Some(CidDetails{
						owner: receipt.clone(),
						bonds: vec![],
					});

					Self::deposit_event(Event::ForceClaimed(receipt, common_cid));
					Ok(())
				})?;
			} else {
				let expired = current_height.saturating_add(T::ClaimValidatePeriod::get());
				Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs| {
					reqs.insert(common_cid, (who.clone(), expired));

					Self::deposit_event(Event::Claiming(who, common_cid, expired));
					Ok(())
				})?;
			}

			if need_update {
				let next_common_cid = common_cid + 1;
				NextCommonCid::<T>::put(&next_common_cid);
			}

			Ok(())
		}

		// [cid_start,cid_end)
		#[pallet::weight(100_000)]
		#[transactional]
		pub fn approve(origin: OriginFor<T>, cid_start: Cid, cid_end: Cid) -> DispatchResult {
			ensure!(ensure_signed(origin)? == Self::admin_key(), Error::<T>::RequireAdmin);
			ensure!(cid_end >= cid_start, Error::<T>::InvalidCidEnd);
			ensure!(cid_end - cid_start <= (T::CidsLimit::get()) as u64, Error::<T>::OutOfCidsLimit);

			Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs|{
				// 1. Get from Distributing
				let approved = Self::take_distributing_reqs(reqs, cid_start, cid_end);
				for (common_cid, who, _) in approved {
					// 2. Put into Distributed
					Distributed::<T>::try_mutate_exists::<_, _, Error<T>, _>(common_cid, |details|{
						*details = Some(CidDetails{
							owner: who,
							bonds: vec![],
						});

						Ok(())
					})?;

					// 3. Delete from Distributing
					reqs.remove(&common_cid);
				}

				Ok(())
			})?;

			Self::deposit_event(Event::Approved(cid_start, cid_end));

			Ok(())
		}

		#[pallet::weight(100_000)]
		#[transactional]
		pub fn disapprove(origin: OriginFor<T>, cid_start: Cid, cid_end: Cid) -> DispatchResult {
			ensure!(ensure_signed(origin)? == Self::admin_key(), Error::<T>::RequireAdmin);
			ensure!(cid_end >= cid_start, Error::<T>::InvalidCidEnd);
			ensure!(cid_end - cid_start <= (T::CidsLimit::get()) as u64, Error::<T>::OutOfCidsLimit);

			Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs|{
				// 1. Get from Distributing
				let disapproved = Self::take_distributing_reqs(reqs, cid_start, cid_end).into_iter().map(|(cid, _, _)|cid).collect::<Vec<_>>();

				// 2. Put into WaitDistributing
				WaitDistributing::<T>::try_mutate::<_, Error<T>, _>(|deque|{
					deque.extend(disapproved.iter());

					Ok(())
				})?;

				// 3. elete from Distributing
				for common_cid in disapproved {
					reqs.remove(&common_cid);
				}

				Ok(())
			})?;

			Self::deposit_event(Event::DisApproved(cid_start, cid_end));

			Ok(())
		}

		// transfer to self equal unbond all
		#[pallet::weight(100_000)]
		pub fn transfer(origin: OriginFor<T>, cid: Cid, receipt: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			ensure!(Self::is_community(cid) || Self::is_common(cid), Error::<T>::OnlyCommunityAndCommonCid);
			let who = ensure_signed(origin)?;
			let receipt = T::Lookup::lookup(receipt)?;

			Distributed::<T>::try_mutate_exists(cid, |details|{
				let mut detail = details.as_mut().ok_or(Error::<T>::UndistributedCid)?;

				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				detail.owner = receipt.clone();
				detail.bonds = vec![];

				Self::deposit_event(Event::Transfered(who, receipt, cid));

				Ok(())
			})
		}

		#[pallet::weight(100_000)]
		pub fn bond(origin: OriginFor<T>, cid: Cid, bond_data: BondData) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(cid), Error::<T>::InvalidCid);

			Distributed::<T>::try_mutate_exists(cid, |details|{
				let detail = details.as_mut().ok_or(Error::<T>::UndistributedCid)?;
				ensure!(detail.owner == who, Error::<T>::RequireOwner);

				let bond_type = bond_data.bond_type;

				detail.bonds.push(bond_data);

				Self::deposit_event(Event::Bonded(who, cid, bond_type));

				Ok(())
			})
		}

		#[pallet::weight(100_000)]
		pub fn unbond(origin: OriginFor<T>, cid: Cid, bond_type: BondType) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_valid(cid), Error::<T>::InvalidCid);

			Distributed::<T>::try_mutate_exists(cid, |details|{
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
	fn do_initialize(now: T::BlockNumber) {
		// 1. Delete from Distributing
		Distributing::<T>::try_mutate::<_, Error<T>, _>(|reqs|{
			for cid in Self::take_expired_reqs(reqs, now) {
				reqs.remove(&cid);
			}

			Ok(())
		}).unwrap_or_default()
	}

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

	fn is_distributed(cid: Cid) -> bool {
		Distributed::<T>::contains_key(cid)
	}

	fn get_common_cid() -> (Cid, bool) {
		// 1. if take from waitqueue is some(cid), return (cid,false)

		if let Some(cid) = WaitDistributing::<T>::try_mutate::<_, Error<T>, _>(
				|deque|{ Ok(deque.pop_front()) }
			).unwrap_or_default() {
			return (cid, false)
		}

		// 2. if none, get from NextCommonCid

		let cid = NextCommonCid::<T>::get();
		// Initialize from 1_000_000
		if cid == 0 {
			(1_000_000, true)
		} else {
			(cid, true)
		}
	}

	fn take_distributing_reqs(
		reqs: &BTreeMap<Cid, (T::AccountId, T::BlockNumber)>,
		cid_start: Cid,
		cid_end: Cid
	) -> Vec<(Cid, T::AccountId, T::BlockNumber)> {
		reqs.range(
			(Included(cid_start), Excluded(cid_end)))
			.map(|(req,(who, expired))|{
				(req.clone(), who.clone(), expired.clone())
			}).collect()
	}

	fn take_expired_reqs(
		reqs: &BTreeMap<Cid, (T::AccountId, T::BlockNumber)>,
		now: T::BlockNumber
	) -> Vec<Cid> {
		reqs.iter()
			.filter_map(|(cid, (_, expired))|{
				if now > *expired {
					Some(*cid)
				} else {
					None
				}
			}).collect()
	}

	pub fn get_bond(cid: Cid) -> Option<CidDetails<T::AccountId>> {
		Self::distributed(cid)
	}
}
