use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct PushshiftSubredditPostsAPILayout {}

#[aorist]
pub enum APILayout {
    PushshiftSubredditPostsAPILayout(AoristRef<PushshiftSubredditPostsAPILayout>),
}
