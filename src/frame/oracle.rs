//! Implements support for the pallet_oracle module.

use crate::frame::system::System;
use codec::Encode;
use core::marker::PhantomData;
use std::fmt::Debug;

/// The subset of the `pallet_oracle::Config` that a client must implement.
#[module]
pub trait Oracle: System {}

/// Fetch device ratio
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct DeviceRatioStore<T: Oracle> {
    #[store(returns = (u128, u128))]
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

/// Change device ratio
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SetDeviceRatioCall<T: Oracle> {
    /// amount1
    pub amount1: u128,
    /// amount2
    pub amount2: u128,
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}