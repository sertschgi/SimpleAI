#[sai_macros::element("component")]
pub fn Search(style: String, icons: Icons) -> Element {
    use super::prelude::*;
    use sai_backend::utils::search::*;

    let mut search_results = use_signal(Vec::<InternSearchResult>::new);

    let onchange = move |e: FormEvent| {
        let mut results = search(e.value()).tree;
        search_results.set(
            results
                .iter_mut()
                .map(|result| InternSearchResult::from(result.clone()))
                .collect::<Vec<InternSearchResult>>(),
        );
    };

    let rendered_search_results = search_results
        .iter()
        .map(|intern| rsx! { SearchResult { intern: intern.clone() } });

    rsx! {
        style { { style } }
        div {
            overflow: "visible",
            class: "Search",
            header {
                input {
                    onchange: onchange,
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
                    {rendered_search_results}
                }
            }
        }
    }
}
