//! All kinds of tracks object

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, time::Duration};

use crate::{
    custom_serde::duration_ms, PlayableId, Restriction, SimplifiedAlbum, SimplifiedArtist, TrackId,
};

/// Full track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-trackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    /// Note that a track may not have an ID/URI if it's local
    pub id: Option<TrackId>,
    pub is_local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<TrackLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
}

/// Track link object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-linkedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackLink {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: TrackId,
}

/// Intermediate full track wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks)
#[derive(Deserialize)]
pub struct FullTracks {
    pub tracks: Vec<FullTrack>,
}

/// Simplified track object.
///
/// `is_playable`, `linked_from` and `restrictions` will only be present when
/// relinking is applied.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<TrackId>,
    pub is_local: bool,
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
}

/// Saved track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: FullTrack,
}

/// Track id with specific positions track in a playlist
///
/// This is a short-lived struct for endpoint parameters, so it uses `&dyn
/// PlayableId` instead of `Box<dyn PlayableId>` to avoid the unnecessary
/// allocation. Same goes for the positions slice instead of vector.
pub struct ItemPositions<'a> {
    pub id: &'a dyn PlayableId,
    pub positions: &'a [u32],
}
