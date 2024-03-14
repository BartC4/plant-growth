use std::any::Any;
use leptos::{*, ev::SubmitEvent};
use leptos_meta::*;
use crate::linden_system::LindenSystem;

mod linden_system;


fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (lSystem, write_lSystem) = create_signal(LindenSystem::new());
    provide_context(write_lSystem);
    provide_context(lSystem);
    view! {
        // Unsure if <Stylesheet> works for trunk CSR or just cargo-leptos SSR.
        // Tailwind styling is currently linked in index.html.
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <main class="bg-lime-400 h-screen w-screen mx-auto text-center">
            <LindenCanvas/>
            <RuleInput/>
        </main>
    }
}

#[component]
fn RuleInput() -> impl IntoView {
    use leptos::html::Input;
    let (variable, set_variable) = create_signal(String::from("F"));
    let (conversion, set_conversion) = create_signal(String::from("F+F+F+F"));

    let variable_input_element: NodeRef<Input> = create_node_ref();
    let conversion_input_element: NodeRef<Input> = create_node_ref();

    let write_lSystem = use_context::<WriteSignal<LindenSystem>>()
       .expect("Problem getting write_lSystem context in main.rs -> RuleInput()");
    let lSystem = use_context::<ReadSignal<LindenSystem>>()
        .expect("Problem getting lSystem context in main.rs -> RuleInput()");

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let var = variable_input_element.get().unwrap().value();
        let conv = conversion_input_element.get().unwrap().value();
        set_variable.set(var);
        set_conversion.set(conv);
        // write_lSystem.update(|val| val.add_rule(variable.get().chars().next().unwrap(), conversion.get()));
    };

    view! {
        <form on:submit=on_submit>
            <div class="container bg-teal-400 border-solid mx-auto max-w-3xl">
                <input type="text" value=variable node_ref=variable_input_element/>
                <input type="text" value=conversion node_ref=conversion_input_element/>
                <button
                    class="border-2 border-solid bg-sky-500 hover:bg-sky-600 active:bg-sky-700"
                    type="submit"
                >Submit</button>
            </div>
        </form>
    }
}

#[component]
fn LindenCanvas() -> impl IntoView {

    let lSystem = use_context::<ReadSignal<LindenSystem>>()
        .expect("Problem getting lSystem context in main.rs -> LindenCanvas()");

    view! {
        <canvas class="bg-lime-100"/>
    }
}

#[component]
fn RuleBox() -> impl IntoView {
    let (rules, set_rules) = create_signal(vec![]);
    let (angle, set_angle) = create_signal(45.0);
    let (length_factor, set_length_factor) = create_signal(1.2);
    let (iteration, set_iteration) = create_signal(0);

    view! {

    }
}