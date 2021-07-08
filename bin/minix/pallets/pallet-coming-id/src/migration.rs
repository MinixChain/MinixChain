use super::*;
use frame_support::pallet_prelude::{ValueQuery, StorageValue};

pub struct __OldAdminKey<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> frame_support::traits::StorageInstance for __OldAdminKey<T> {
    fn pallet_prefix() -> &'static str { "ComingId" }
    const STORAGE_PREFIX: &'static str = "Key";
}

pub type OldAdminKey<T> = StorageValue<
    __OldAdminKey<T>, <T as frame_system::Config>::AccountId, ValueQuery
>;

/// A storage migration that remove old admin key and set new admin keys
pub fn update_keys<T: Config>(
    high_key: T::AccountId,
    medium_key: T::AccountId,
    low_key: T::AccountId
) -> Weight {
    let mut writes = 0;
    let mut reads = 0;

    let is_exists = OldAdminKey::<T>::get() != T::AccountId::default();
    log::info!("ComingId: old key is exists? {}", is_exists);

    if is_exists {
        log::info!("ComingId: update high key {:?}", high_key.clone());
        HighKey::<T>::put(high_key);
        log::info!("ComingId: update medium key {:?}", medium_key.clone());
        MediumKey::<T>::put(medium_key);
        log::info!("ComingId: update low key {:?}", low_key.clone());
        LowKey::<T>::put(low_key);

        log::info!("ComingId: kill old key");
        OldAdminKey::<T>::kill();

        writes += 3;
    }

    reads += 1;

    T::DbWeight::get().writes(writes) + T::DbWeight::get().reads(reads)
}

pub fn high_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut &[252, 78, 161, 70, 191, 31, 25, 188, 123, 130, 140, 25, 190, 31, 125, 118, 76, 85, 16, 140, 138, 175, 96, 117, 208, 12, 159, 167, 218, 30, 202, 117][..]
    ).unwrap_or_default()
}

pub fn medium_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut &[116, 9, 45, 229, 24, 198, 57, 77, 94, 194, 216, 145, 92, 34, 130, 45, 13, 98, 204, 105, 156, 232, 217, 23, 124, 56, 232, 18, 163, 237, 53, 101][..]
    ).unwrap_or_default()
}

pub fn low_key<AccountId: Decode + Default>() -> AccountId {
    AccountId::decode(
        &mut &[244, 18, 253, 40, 226, 131, 86, 145, 4, 122, 73, 216, 54, 8, 193, 146, 73, 113, 27, 54, 208, 156, 97, 198, 52, 86, 108, 0, 59, 59, 198, 96][..]
    ).unwrap_or_default()
}
