use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct PushshiftSubredditPostsAPILayout {}

#[aorist]
pub enum APILayout {
    PushshiftSubredditPostsAPILayout(PushshiftSubredditPostsAPILayout),
}
