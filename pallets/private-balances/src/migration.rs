use crate::*;

pub struct UpdateKeys<T: Config>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for UpdateKeys<T> {
	fn on_runtime_upgrade() -> Weight {
		Weight::zero()
	}
}
