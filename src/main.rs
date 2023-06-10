#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use std::sync::{Arc, Mutex, RwLock};

use dioxus::prelude::{ScopeId, ScopeState};

#[derive(Clone, Debug)]
pub struct PersistentState<V: Clone + Send + 'static> {
    value: Arc<RwLock<V>>,
}

impl<V: Clone + Send + 'static> Drop for PersistentState<V> {
    fn drop(&mut self) {
        println!("references left: {}", Arc::strong_count(&self.value));
    }
}

pub fn use_buggy_state<'a, V>(
    cx: &'a ScopeState,
    init: impl 'static + FnOnce() -> V,
) -> Option<&PersistentState<V>>
where
    V: Clone + 'static + Send,
{
    dioxus_hooks::use_future(cx, (), move |_| async move {
        println!("creating");
        return PersistentState {
            value: Arc::new(RwLock::new(init())),
        };
    })
    .value()
}

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

pub fn Inner(cx: Scope) -> Element {
    println!("0");
    let game = use_buggy_state(cx, || ()).cloned();
    println!("2 {game:?}");

    cx.render(rsx! { "peekaboo" })
}

pub fn Game(cx: Scope) -> Element {
    println!("1");
    let mount = use_state(cx, || false);

    if *mount.get() {
        return cx.render(rsx! {
            button {
                onclick: move |_| {
                    println!("!!!!");
                    mount.set(false);
                },
                "Unmount"
            },
            Inner {}
        });
    } else {
        return cx.render(rsx! {
            button {
                onclick: move |_| {
                    println!("!!");
                    mount.set(true);
                },
                "Mount"
            }
        });
    }
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
        Game {}
    })
}
