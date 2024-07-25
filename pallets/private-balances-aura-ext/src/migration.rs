use crate::*;

pub struct UpdateAuthorities<T: Config>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for UpdateAuthorities<T> {
	fn on_runtime_upgrade() -> Weight {
		if <Authorities<T>>::exists() {
			return Weight::zero();
		}

		let authorities = pallet_aura::Pallet::<T>::authorities();
		Pallet::<T>::initialize_authorities(authorities);

		T::DbWeight::get().reads_writes(1, 1)
	}
}
