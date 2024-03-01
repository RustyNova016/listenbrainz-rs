//! Contains the implementations of functions for the [`crate::raw::response`] models.
//! While those models are meant to be low level, there can be still some useful function
//! that can be implemented

use super::response::UserListensListen;

impl UserListensListen {
    /// Returns whether the listen is linked to a Musicbrainz recording.
    pub fn is_unlinked(&self) -> bool {
        self.track_metadata.mbid_mapping.is_some()
    }
}
