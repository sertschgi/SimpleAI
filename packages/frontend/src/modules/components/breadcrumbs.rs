use simple_ai_backend::modules::utils::project::*;
use uuid::Uuid;

use super::utils::*;

#[item]
pub fn Breadcrumbs() -> Element {
    let route_str = router().full_route_string();
    let mut crumbs: Vec<&str> = route_str.split_terminator("/").collect();
    crumbs.remove(0);
    let mut assembled = String::new();
    let to_text = move |crumb: &str| match Uuid::parse_str(crumb) {
        Ok(id) => Project::try_from_id(id).unwrap().id.to_string(),
        Err(_) => crumb.to_string(),
    };
    rsx! {
        main {
            Link { to: Route::Start {}, HomeIcon {} }
            for crumb in crumbs.iter() {
                RightBracketIcon {}
                Link {
                    to: {
                        assembled.push('/');
                        assembled.push_str(crumb);
                        assembled.parse::<NavigationTarget<Route>>().unwrap_or(Route::Projects {}.into())
                    },
                    p { {to_text(crumb)} }
                }
            }
        }
    }
}
