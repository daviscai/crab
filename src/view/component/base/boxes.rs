//!
//! 基础组件 - 容器组件
//! 
//! 支持：
//! 1、支持响应式设计    done
//! 2、钉住固定位置(pin) done
//! 3、固定大小         done
//! 4、支持自定义主题风格 done
//! 5、支持自定义样式    done
//! 6、支持添加多个子组件 done
//! 7、支持显示/隐藏动画 
//! 8、支持事件响应（点击、双击、拖动、长按、鼠标滑过、拖拽）
//! 
extern crate css_in_rust;

use yew::prelude::*;
use css_in_rust::Style;
use yew::html::Children;

use crate::component::base::theme::Theme;

#[derive(Debug)]
pub enum Hovered {
    Stay,
    None,
}

#[derive(Debug)]
pub enum Clicked {
    Choose,
    None,
}


#[derive(Debug)]
pub enum Msg {
    
}


#[derive(Properties, PartialEq, Clone)]
pub struct BoxesProps {

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub style: String,

    #[prop_or_default]
    pub theme: String,

    #[prop_or_default]
    pub width: String,

    #[prop_or_default]
    pub height: String,

    #[prop_or_default]
    pub pin: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub on_hover: Callback<Hovered>,

    #[prop_or_default]
    pub on_click: Callback<Clicked>,
}

impl BoxesProps {
    pub fn styles(&self) -> String {

        let mut css = Vec::new();

        // 根据 theme 设置 css
        //log::info!("====: {}", theme);
        let theme_css = Theme::new(self.theme.as_str());
        css.push( format!("{}", theme_css) );

        if !self.width.is_empty() && self.width.len()>0 {
            css.push( format!("width: {};", self.width) );
        }

        if !self.height.is_empty() && self.height.len()>0 {
            css.push( format!("height: {};", self.height) );
        }

        if !self.style.is_empty() && self.style.len()>0 {
            css.push( format!("{};", self.style) );
        }

        // 钉在某处， pin="top,right,bottom,left" , pin=",50px,20%" 
        if !self.pin.is_empty() && self.pin.len()>0 {
            css.push("position: fixed;".into());
            let position: Vec<&str> = self.pin.split(',').collect();
            // log::info!("====: {}", position[0].len());
            if position.len()>0 && !position[0].is_empty() && position[0].len()>0 {
                css.push( format!("top:{};", position[0]) );
            }
            if position.len()>1 && !position[1].is_empty() && position[1].len()>0 {
                css.push( format!("right:{};", position[1]) );
            }
            if position.len()>2 && !position[2].is_empty() && position[2].len()>0 {
                css.push( format!("bottom:{};", position[2]) );
            }
            if position.len()>3 && !position[3].is_empty() && position[3].len()>0 {
                css.push( format!("left:{};", position[2]) );
            }
        }

        css.concat()
    }
}

pub struct Boxes {
    style: Style,
    props: BoxesProps,
    link: ComponentLink<Self>
}

impl Component for Boxes {
    type Message = Msg;
    type Properties = BoxesProps;

    fn create( props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let css = props.styles();
        let style = match Style::create("crab-container", css) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };

        Boxes {
            style: style,
            props: props,
            link: link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("====: {:?}", msg);
        true
    }

    fn change(&mut self,  _: Self::Properties) -> bool { 
	    false 
    }

    fn view(&self) -> Html {
        let mut theme_class = "".into();
        if !self.props.theme.is_empty() {
            theme_class = format!("crab-theme-{}", self.props.theme);
        };

        let onmouseover = self.props.on_hover.reform(|_| Hovered::Stay);
        let onmouseout = self.props.on_hover.reform(|_| Hovered::None);
        let onclick = self.props.on_click.reform(|_| Clicked::Choose);
        html! {
            <div id={&self.props.id} class=("crab-container", theme_class, &self.props.class, self.style.clone()) 
             onmouseover=onmouseover onmouseout=onmouseout  onclick=onclick
            >
            { self.view_children() }
            </div>
        }
    }
}

impl Boxes {
    fn view_children(&self) -> Html {
        if self.props.children.is_empty() {
            return html! {};
        }

        html! {
            { self.props.children.render() }
        }
    }
}