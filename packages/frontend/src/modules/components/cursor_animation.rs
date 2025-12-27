use super::utils::*;
use tokio::time::*;

const RUN_CURSOR_ANIMATION: &str = r#"
    window.runCursorAnimation();
"#;

#[item]
pub fn CursorAnimationLayout() -> Element {
    rsx! {
        main {
            id: "cursor_anim_bg",
            onmount: move |_| async move {
                sleep(Duration::from_millis(50)).await;
                document::eval(RUN_CURSOR_ANIMATION);
            },
            article { Outlet::<Route> {} }
            document::Script { src: asset!("/assets/scripts/cursor-animation.js") }
        }
    }
}
