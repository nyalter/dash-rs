use crate::model::user::{Color, IconType};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Struct modelling the partial user data returned by the `getGJUsers` endpoint.
///
/// Note that no field `diamonds` exists here. This is consistent with Geometry Dash's behavior, as
/// the GD server exhibit a bug where they do not provide diamonds information, although the client
/// has the UI for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchedUser<'a> {
    /// This [`SearchedUser`]'s name
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`
    pub name: Cow<'a, str>,

    /// The [`SearchedUser`]'s unique user ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`
    pub user_id: u64,

    /// This [`SearchedUser`]'s stars
    ///
    /// ## GD Internals:
    /// This value is provided at index `3`
    pub stars: u32,

    /// This [`SearchedUser`]'s beaten demons
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`
    pub demons: u16,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `6`
    pub index_6: Option<Cow<'a, str>>,

    /// This [`SearchedUsers`] creator points
    ///
    /// ## GD Internals:
    /// This value is provided at index `8`
    pub creator_points: u16,

    /// The index of the icon being displayed.
    ///
    /// ## GD Internals:
    /// This value is provided at index `9`
    pub icon_index: u16,

    /// This [`SearchedUser`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    pub primary_color: Color,

    /// This [`SearchedUser`]'s secondary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The amount of secret coins this [`SearchedUser`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    pub secret_coins: u8,

    /// The type of icon being displayed
    ///
    /// ## GD Internals:
    /// This value is provided at index `14`
    pub icon_type: IconType,

    /// Values indicating whether this [`SearchedUser`] has glow activated or not.
    ///
    /// ## GD Internals:
    /// This value is provided at index `15`. A value of `"2"` means `true`, a value of `"0"` means
    /// `false` (yes, this is different from how the field is encoded in [`Profile`])
    pub has_glow: bool,

    /// The [`SearchedUser`]'s unique account ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `16`
    pub account_id: u64,

    /// The amount of user coins this [`SearchedUser`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `17`
    pub user_coins: u16,
}

mod internal {
    use crate::model::user::searched::SearchedUser;

    include!(concat!(env!("OUT_DIR"), "/searched.boilerplate"));
}
