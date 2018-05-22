// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate stdweb;
#[macro_use]
extern crate yew;

mod num;

use stdweb::web::document;
use stdweb::web::{IElement, IParentNode};
use yew::prelude::*;

type Context = ();

struct Model {
    input: String,
    base: u32,
    size: u32,
}

enum Msg {
    Input(String),
    Base(u32),
    Size(u32),
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            input: String::new(),
            base: 10,
            size: 32,
        }
    }

    fn update(&mut self, msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Input(s) => self.input = s,
            Msg::Base(b) => self.base = b,
            Msg::Size(s) => self.size = s,
        }
        true
    }
}

impl Model {
    fn view_control_text(&self) -> Html<Context, Self> {
        html! {
            <>
            <label for="inputText", >{ "Input" }</label>
            <input id="inputText",
                   class="form-control",
                   type="text",
                   oninput=|e: InputData| Msg::Input(e.value), />
            </>
        }
    }

    fn view_control_base(&self) -> Html<Context, Self> {
        static OPTIONS: &[(u32, &str)] = &[
            (10, "Decimal"),
            (2, "Binary"),
            (8, "Octal"),
            (16, "Hexadecimal"),
        ];
        Self::make_select("inputBase", "Input Base", OPTIONS, Msg::Base, self.base)
    }

    fn view_control_size(&self) -> Html<Context, Self> {
        static OPTIONS: &[(u32, &str)] = &[
            (8, "8-bit"),
            (16, "16-bit"),
            (32, "32-bit"),
            (64, "64-bit"),
        ];
        Self::make_select("inputSize", "Int size", OPTIONS, Msg::Size, self.size)
    }

    fn make_select<V, F>(id: &str, title: &str, v: &[(V, &str)], f: F, cur: V) -> Html<Context, Self>
    where
        V: std::fmt::Display + std::str::FromStr + PartialEq,
        <V as std::str::FromStr>::Err: std::fmt::Debug,
        F: 'static + Fn(V) -> Msg,
    {
        let value = move |cd: ChangeData| match cd {
            ChangeData::Select(se) => {
                let options = se.selected_options();
                assert_eq!(options.len(), 1);
                f(options
                    .item(0)
                    .unwrap()
                    .get_attribute("value_")
                    .unwrap()
                    .parse()
                    .unwrap())
            }
            _ => unreachable!(),
        };
        let iter = v.iter().map(|(v, s)| {
            if cur == *v {
                html! {<option value_={v}, selected=1,>{s}</option>}
            } else {
                html! {<option value_={v},>{s}</option>}
            }
        });
        html! {
            <>
            <label for={id}, >{ title }</label>
            <select id={id}, class="form-control", onchange={ value }, >
                { for iter }
            </select>
            </>
        }
    }

    fn view_controls(&self) -> Html<Context, Self> {
        html! {
            <form>
                <div class="form-row", >
                    <div class="col-sm", >
                        { self.view_control_text() }
                    </div>
                    <div class="col-sm", >
                        { self.view_control_base() }
                    </div>
                    <div class="col-sm", >
                        { self.view_control_size() }
                    </div>
                </div>
            </form>
        }
    }

    fn view_result(&self, out_base: u32) -> Html<Context, Self> {
        html! {
            <div class="input-group", >
                <div class="input-group-prepend", >
                    <div class="input-group-text", style="width: 3em", >{ out_base.to_string() }</div>
                </div>
                <input type="text", class="form-control", readonly=1,
                       style="background: white",
                       value=num::process(&self.input, self.base, out_base, self.size), />
            </div>
        }
    }

    fn view_results(&self) -> Html<Context, Self> {
        html! {
            <>
            { self.view_result(2) }
            { self.view_result(8) }
            { self.view_result(10) }
            { self.view_result(16) }
            </>
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div class="container", >
                <div class="row", class="mb-2", >
                    <div class="col-md-12", >
                    { self.view_controls() }
                    </div>
                </div>
                <div class="row", >
                    <div class="col-md-12", >
                    <div class="mb-1", >{ "Result:" }</div>
                    { self.view_results() }
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    let element = document().query_selector("#app").unwrap().unwrap();
    app.mount(element);
    yew::run_loop();
}
