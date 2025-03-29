use crate::prelude::{components::prelude::params::*, *};
use simple_ai_backend::utils::prelude::*;

pub(crate) static DRAG_NODE: GlobalSignal<Option<Node>> = Signal::global(|| None);
pub(crate) static CONNECTION: GlobalSignal<Option<InternConnection>> = Signal::global(|| None);
