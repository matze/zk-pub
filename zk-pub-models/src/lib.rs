//! Zettel models shared by the offline generator and the web app.

use serde::{Deserialize, Serialize};

/// A zettel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zettel {
    /// Link anchor to the zettel.
    pub anchor: String,
    /// Title of the zettel and derived from the top-level h1 header.
    pub title: String,
    /// HTML of the entire zettel content.
    pub inner_html: String,
}
