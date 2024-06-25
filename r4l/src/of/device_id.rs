// SPDX-License-Identifier: GPL-2.0

//! Devicetree and Open Firmware abstractions.
//!
//! C header: [`include/linux/of_*.h`](../../../../include/linux/of_*.h)

/// An open firmware device id.
#[derive(Clone, Copy)]
pub enum DeviceId {
    /// An open firmware device id where only a compatible string is specified.
    Compatible(&'static str),
}

/// Defines a const open firmware device id table that also carries per-entry data/context/info.
///
/// # Example
///
/// ```
/// # use kernel::{define_of_id_table, module_of_id_table, driver_of_id_table};
/// use kernel::of;
///
/// define_of_id_table! {MY_ID_TABLE, u32, [
///     (of::DeviceId::Compatible(b"test-device1,test-device2"), Some(0xff)),
///     (of::DeviceId::Compatible(b"test-device3"), None),
/// ]};
///
/// module_of_id_table!(MOD_TABLE, ASAHI_ID_TABLE);
///
/// // Within the `Driver` implementation:
/// driver_of_id_table!(MY_ID_TABLE);
/// ```
#[macro_export]
macro_rules! define_of_id_table {
    ($name:ident, $data_type:ty, $($t:tt)*) => {
        $crate::define_id_array!($name, $crate::of::DeviceId, $data_type, $($t)*);
    };
}

/// Convenience macro to declare which device ID table to use for a bus driver.
#[macro_export]
macro_rules! driver_of_id_table {
    ($name:expr) => {
        $crate::driver_id_table!(
            OF_DEVICE_ID_TABLE,
            $crate::of::DeviceId,
            Self::IdInfo,
            $name
        );
    };
}

/// Declare a device ID table as a module-level table. This creates the necessary module alias
/// entries to enable module autoloading.
#[macro_export]
macro_rules! module_of_id_table {
    ($item_name:ident, $table_name:ident) => {
        $crate::module_id_table!($item_name, "of", $crate::of::DeviceId, $table_name);
    };
}
