// %%% components / search.rs %%%

// %% includes %%
use super::search_result::{InternSearchResult, SearchResult};
use super::utils::*;

// %% main %%
#[component]
pub fn Search() -> Element {
    let mut intern_search_results = use_signal(Vec::<InternSearchResult>::new);
    let mut search_results = use_signal(Vec::<Node>::new);

    let input = move |e: FormEvent| {
        search_results.set(search(e.value()));
        intern_search_results.clear();
    };

    use_effect(move || {
        intern_search_results.set(
            search_results()
                .iter_mut()
                .map(|result| InternSearchResult::from(result.clone()))
                .collect::<Vec<InternSearchResult>>(),
        );
    });

    rsx! {
        div {
            overflow: "visible",
            class: "Search",
            header {
                input {
                    oninput: input,
                    type: "search",
                    placeholder: "search"
                }
                // nav {
                // 	button { "your nodes" }
                // 	button { "installed nodes" }
                // 	button { "profiles" }
                // }
            }
            main {
                overflow: "visible",
                div { class: "spacer" }
                section {
                    class: "results",
                    height: "100%",
                    for intern in intern_search_results() {
                        SearchResult { intern }
                    }
                }
            }
        }
    }
}
