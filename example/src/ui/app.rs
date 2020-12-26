use dominator::{html, Dom};
use std::rc::Rc;
use super::{
  state::State,
  sidebar::Sidebar,
  main::Main
};

pub fn render() -> Dom {
    let state = Rc::new(State::new());

    html!("div", {
        .class("page")
        .children(vec![
          Sidebar::render(state.clone()),
          Main::render(state.clone()),
        ])
    })
}



/*
<main id="main">
  <aside id="sidebar">
    <div class="label">Check it out:</div>
    <div id="border-drag"></div>
  </aside>
  <div id="container">
    <div id="wrapper">
      <div id="content">
        <img class="smiley-face" src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/SNice.svg/1024px-SNice.svg.png" />
        <div class="hello-world">Hello World!</div>
      </div>
    </div>
  </div>
</main>
*/

