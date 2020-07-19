//! crab-yew 1.0 基于yew的前端框架组件库
//! 
//! 组件：
//! 1、基础组件：容器、颜色、字体、文本、输入框、单选框、复选框、Icon、switch、line、按钮、link
//! 2、复合组件：tab、list、下拉选框、menu、tag、code-block、进度条、图片、音频、视频、slider、
//! 3、功能组件：search-bar、pageing、loading、smart-list（图片列表/商品列表/瀑布流）、video-player、image-player
//! 
//! 功能：
//! 1、支持响应式自适应 PC web布局 和 Mobile App布局
//! 2、支持主题设置（颜色（灰色系/暖色系/亮色系/蓝色系/红色系）、风格（简约/多彩/电商））和自定义主题
//! 3、支持多线程渲染
//! 4、支持鼠标和手触事件
//! 
//!  Button
//!  Layout
//!  Navbar
//!  Form
//!  Card
//!  Message
//!  Table
//！ Pagination
//！ Modal
//!  Sidebar
//!  Tab
//!  Tooltips
//!  Calendar
//!  Assets
//! 
//! 组件支持：
//! 1、默认和内置的主题风格（bootstrap、ant design、material design、metro design、gray-white、color-gradient、black、blue、pink、Medium、tiktok）
//! 2、属性配置化，零逻辑代码
//! 3、响应式，支持 PC 和 mobile 
//! 4、点击、点触、长按、滑动、双击、拖动等事件
//! 


//! 

pub mod app;
pub mod component;

use wasm_bindgen::prelude::*;
use crate::app::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
    Ok(())
}