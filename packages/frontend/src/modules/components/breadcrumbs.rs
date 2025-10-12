// %%% components / breadcrumbs.rs %%%
/// The top navigation which shows you where you are.
//
// %% includes %%
use super::utils::*;

// %% main %%
#[item]
pub fn Breadcrumbs() -> Element {
    let route_str = router().full_route_string();
    let mut crumbs: Vec<&str> = route_str.rsplit_terminator("/").collect();
    crumbs.pop();
    let mut assembled = String::new();
    rsx! {
        div {
            Link { to: Route::Start {}, HomeIcon {} }
            for crumb in crumbs.iter() {
                RightBracketIcon {}
                Link {
                    to: {
                        assembled.push('/');
                        assembled.push_str(crumb);
                        assembled.parse::<NavigationTarget<Route>>().unwrap()
                    },
                    {crumb.to_string()}
                }
            }
        }
    }
}
