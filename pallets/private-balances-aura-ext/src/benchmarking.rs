use frame_benchmarking::v2::*;

use super::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_authorities(b: Linear<1, 100_000>) -> Result<(), BenchmarkError> {
		let authority = T::AuthorityId::decode(&mut [0u8; 32].as_slice())
			.expect("T::AuthorityId decode should be successful");

		let authorities = alloc::vec![authority; b as usize]
			.try_into()
			.expect("authorities length should be smaller or equal T::MaxAuthorities. Please check values in Linear<.., ..>");

		#[extrinsic_call]
		_(RawOrigin::Root, authorities);

		Ok(())
	}

	#[benchmark]
	fn set_trusted_authorities(b: Linear<0, 100_000>) -> Result<(), BenchmarkError> {
		let authority = T::AuthorityId::decode(&mut [0u8; 32].as_slice())
			.expect("T::AuthorityId decode should be successful");

		let authorities = alloc::vec![authority; b as usize]
			.try_into()
			.expect("authorities length should be smaller or equal T::MaxAuthorities. Please check values in Linear<.., ..>");

		#[extrinsic_call]
		_(RawOrigin::Root, authorities);

		Ok(())
	}
}
