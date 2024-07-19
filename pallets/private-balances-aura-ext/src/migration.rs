use crate::*;

pub struct UpdateAuthorities<T: Config>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for UpdateAuthorities<T> {
	fn on_runtime_upgrade() -> Weight {
		Pallet::<T>::initialize_authorities(&pallet_aura::Pallet::<T>::authorities());
		T::DbWeight::get().reads_writes(1, 1)
	}
}