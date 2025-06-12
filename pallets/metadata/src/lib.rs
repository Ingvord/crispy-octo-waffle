#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Storage: maps CID (Vec<u8>) to (owner AccountId, block number)
    #[pallet::storage]
    #[pallet::getter(fn metadata)]
    pub type MetadataStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>, // CID
        (T::AccountId, T::BlockNumber),
        OptionQuery
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_metadata(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(
                !MetadataStore::<T>::contains_key(&cid),
                Error::<T>::AlreadyRegistered
            );

            let block_number = <frame_system::Pallet<T>>::block_number();
            MetadataStore::<T>::insert(&cid, (sender.clone(), block_number));

            Self::deposit_event(Event::MetadataRegistered { cid, owner: sender });

            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyRegistered,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MetadataRegistered { cid: Vec<u8>, owner: T::AccountId },
    }
}