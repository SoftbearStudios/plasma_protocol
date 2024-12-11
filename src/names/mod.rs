// SPDX-FileCopyrightText: 2024 Softbear, Inc.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod fmt_utils;
mod player;
mod referrer;
mod tests;

#[cfg(feature = "server")]
pub use fmt_utils::{
    no_confusable_italics, trim_and_slice_up_to, trim_and_slice_up_to_array_string,
};
pub use fmt_utils::{slice_up_to_array_string, slice_up_to_chars};
pub use player::{NickName, PlayerAlias, TeamName};
pub use referrer::{InvalidRealmName, NexusPath, RealmName, Referrer};
