use dominator::{html, Dom, clone};
use std::rc::Rc;
use web_sys::HtmlCanvasElement;
use crate::{
    ui::state::State,
    scene::Scene
};

pub struct Main {
    state: Rc<State>
}

impl Main {
    pub fn render(state:Rc<State>) -> Dom {
        let _self = Rc::new(Self::new(state.clone()));

        html!("main", {
            .child(
                html!("canvas" => HtmlCanvasElement, {
                    .after_inserted(clone!(state => move |canvas| {
                        *state.scene.borrow_mut() = Some(Scene::new(canvas));
                    }))
                })
            )
        })
    }

    fn new(state:Rc<State>) -> Self {
        Self { state }
    }
}
