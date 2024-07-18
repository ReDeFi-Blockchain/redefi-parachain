use frame_benchmarking::v2::*;

use super::*;

#[benchmarks(
    where
        // TODO: Can we use less ugly way to create Config::AuthorityId?
        for<'a> T::AuthorityId: TryFrom<&'a [u8]>,
        for<'a> <T::AuthorityId as TryFrom<&'a [u8]>>::Error: core::fmt::Debug,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_trusted_authorities() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root.into();

		let public_key = [0u8; 32].as_slice();
		let authority = T::AuthorityId::try_from(public_key).unwrap();
		let authorities = alloc::vec![authority].try_into().unwrap();

		#[block]
		{
			<Pallet<T>>::set_trusted_authorities(origin, authorities)?;
		}

		Ok(())
	}
}
