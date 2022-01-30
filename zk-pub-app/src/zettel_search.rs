use crate::app::Route;
use crate::{text_input, TextInput};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;
use yew_router::prelude::*;
use zk_pub_models::{Zettel, ZettelMap};

/// Return Zettel from `map` sorted by score matching `term`.
fn sorted_zettels(map: ZettelMap, term: &str) -> Vec<(i64, (String, Zettel))> {
    let matcher = SkimMatcherV2::default();

    let mut scored = map
        .into_iter()
        .filter_map(|item| match matcher.fuzzy_match(&item.1.title, term) {
            Some(score) => Some((score, item)),
            None => None,
        })
        .collect::<Vec<_>>();

    scored.sort_by_key(|(score, _)| -score.abs());
    scored
}

#[function_component(ZettelSearch)]
pub fn zettel_search() -> Html {
    let current = use_state(|| String::new());
    let zettel_map = use_context::<ZettelMap>().expect("no ZettelMap found");
    let callback_current = current.clone();

    let on_change = Callback::from(move |message| match message {
        text_input::Message::Value(value) => callback_current.set(value.clone()),
        text_input::Message::Clear => callback_current.set("".to_string()),
    });

    let list = if *current == "" {
        vec![]
    } else {
        sorted_zettels(zettel_map, &current)
            .iter()
            .map(|(_, (id, zettel))| {
                html! {
                    <li><Link<Route> to={Route::Zettel { id: id.clone() }}>{ zettel.title.clone() }</Link<Route>></li>
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <>
        <div class="row">
            <TextInput on_change={on_change} />
        </div>
        <div class="row">
            <ul class="search">
            {list}
            </ul>
        </div>
        </>
    }
}
