// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

//! # Primitives crate.
//!
//! This crate contains types, traits and constants.

#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub mod budget;

/// This type works like [`PhantomData`] but supports generating _scale-info_ descriptions to generate node metadata.
#[derive(Encode, Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub struct PhantomType<T>(core::marker::PhantomData<T>);

impl<T: TypeInfo + 'static> TypeInfo for PhantomType<T> {
	type Identity = PhantomType<T>;

	fn type_info() -> scale_info::Type {
		use scale_info::{
			build::{FieldsBuilder, UnnamedFields},
			form::MetaForm,
			type_params, Path, Type,
		};
		Type::builder()
			.path(Path::new("up_data_structs", "PhantomType"))
			.type_params(type_params!(T))
			.composite(
				<FieldsBuilder<MetaForm, UnnamedFields>>::default().field(|b| b.ty::<[T; 0]>()),
			)
	}
}
impl<T> MaxEncodedLen for PhantomType<T> {
	fn max_encoded_len() -> usize {
		0
	}
}
