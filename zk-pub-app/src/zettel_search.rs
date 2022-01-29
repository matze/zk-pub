use crate::app::Route;
use crate::TextInput;
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

    let on_change = Callback::from(move |value: String| {
        callback_current.set(value.clone());
    });

    let list = if *current == "" {
        vec![]
    } else {
        sorted_zettels(zettel_map, &current)
            .iter()
            .map(|(_, (id, zettel))| {
                let history = use_history().unwrap();
                let id = id.clone();

                let current = current.clone();
                let on_select = {
                    Callback::from(move |_| {
                        // Clear the search bar.
                        current.set("".to_string());
                        history.push(Route::Zettel { id: id.clone() })
                    })
                };

                html! {
                    <li onclick={on_select}>{format!("{}", zettel.title)}</li>
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <>
        <div class="row">
            <TextInput on_change={on_change} value={(*current).clone()}/>
        </div>
        <div class="row">
            <ul class="search">
            {list}
            </ul>
        </div>
        </>
    }
}
