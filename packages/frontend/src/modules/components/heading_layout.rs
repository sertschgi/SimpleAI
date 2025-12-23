use super::utils::*;
use change_case::pascal_case;

#[item]
pub fn HeadingLayout() -> Element {
    let route_str = router().full_route_string();
    let route_elms: Vec<&str> = route_str.split_terminator("/").collect();
    let unfmt_name = route_elms.last().expect(&format!(
        "Heading Layout errored because of unsatisfied route elements: {:?}",
        route_elms,
    ));
    let parsed_name = pj_name_from_str(&unfmt_name);
    let name = pascal_case(&parsed_name);
    rsx! {
        main {
            h1 { {name} }
            article { Outlet::<Route> {} }
        }
    }
}
