use yew::prelude::*;
use crate::component::base::{boxes::Boxes, boxes::Hovered, theme::Theme};

// #[derive(Properties, PartialEq, Clone)]
// pub struct AppProps {

// }

#[derive(Debug)]
pub struct App {
    theme: String,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
    Hover
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            // 设置主题风格，有 bootstrap
            theme: "bootstrap".into(),
            link: link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("====11: {:?}", msg);
        true
    }

    fn change(&mut self,  _: Self::Properties) -> bool { 
	    false 
    }

    fn view(&self) -> Html {
        let theme_class = Theme::get_theme_class(self.theme.as_str());
       
        html! {
            <div id="container" class={theme_class}>
                <Boxes class="container-sm">
                    <div class="row">
                        <div class="col-4">
                        { "One of three columns" }
                        </div>
                        <div class="col-4">
                        { "One of three columns" }
                        </div>
                        <div class="col-4">
                        { "One of three columns" }
                        </div>
                    </div>
                    <Boxes style="background:#eee"  width="100%" height="50px" />
                </Boxes>
            </div>
        }
    }
}
