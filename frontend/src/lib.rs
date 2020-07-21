use wasm_bindgen::prelude::*;
use yew::services::storage::{StorageService, Area};

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use serde::{Serialize, Deserialize};

mod webgl;
use webgl::*;

use std::fmt;
use std::slice::Iter;

use yew::format::Json;
use yew::App;

const RESTORE_KEY: &'static str = "tab_navigation";

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Tab {
    Home,
    WebGl,
}
impl Tab {
    pub fn iterator() -> Iter<'static, Tab> {
        static DIRECTIONS: [Tab; 2] = [Tab::Home, Tab::WebGl];
        DIRECTIONS.iter()
    }
}
impl Default for Tab {
    fn default() -> Self {
        Tab::Home
    }
}
impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tab::Home => write!(f, "Home"),
            Tab::WebGl => write!(f, "WebGl"),
        }
    }
}

pub enum Msg {
    ChangeTab(Tab),
}

pub struct Nav {
    link: ComponentLink<Self>,
    active_tab: Tab,
    storage: StorageService,
}

impl Component for Nav {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let active_tab: Tab = {
            if let Json(Ok(tab)) = storage.restore(RESTORE_KEY) {
                tab
            } else {
                Tab::default()
            }
        };

        Self {
            link,
            active_tab,
            storage,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let old_tab = self.active_tab.clone();
        match msg {
            // active tab enum
            Msg::ChangeTab(tab) => {
                self.active_tab = tab;
            }
        }
        self.storage.store(RESTORE_KEY, Json(&self.active_tab));

        old_tab != self.active_tab
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let inner = match self.active_tab {
            Tab::Home => {
                html!{<p>{"Home page"}</p>}
            },
            Tab::WebGl => {
                html!{<WebGl/>}
            }
        };

        html! {
            <div class="nav-container">
                <div class="nav">{ for Tab::iterator().map(|tab| html!{<p onclick={self.link.callback(move |_| Msg::ChangeTab(*tab))}>{tab.to_string()}</p>}) }</div>
                {inner}
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    App::<Nav>::new().mount_to_body();
}
