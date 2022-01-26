mod app;
mod raw_html;
mod text_input;
mod zettel_details;
mod zettel_search;

use app::App;
use raw_html::RawHtml;
use text_input::TextInput;
use zettel_details::ZettelDetails;
use zettel_search::ZettelSearch;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
