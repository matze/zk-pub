//! Zettel models shared by the offline generator and the web app.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A zettel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zettel {
    /// Title of the zettel and derived from the top-level h1 header.
    pub title: String,
    /// HTML of the entire zettel content.
    pub inner_html: String,
}

/// Map mapping anchors to Zettel.
pub type ZettelMap = HashMap<String, Zettel>;
