
use super::*;

pub trait ComingNFT<AccountId> {
    fn mint(
        who: &AccountId,
        cid: Cid,
        card: Vec<u8>
    ) -> DispatchResult;

    fn transfer(
        who: &AccountId,
        cid: Cid,
        recipient: &AccountId
    ) -> DispatchResult;
}
