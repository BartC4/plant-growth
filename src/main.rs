use leptos::*;
use leptos::ev::{Event, MouseEvent};

mod linden_system;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="main">
            <RuleBox/>
            <canvas id="canvas"/>
        </main>
    }
}

#[component]
fn RuleBox() -> impl IntoView {
    let first_rule = (1, create_signal("F".to_string()), create_signal("F+FX".to_string()));
    let second_rule = (2, create_signal("X".to_string()), create_signal("FF".to_string()));
    let initial_rules = Vec::from([first_rule, second_rule]);

    let (rules, set_rules) = create_signal(initial_rules);
    provide_context(RuleContext(rules));
    provide_context(SetRuleContext(set_rules));

    let (axiom, set_axiom) = create_signal("X".to_string());
    let (angle, set_angle) = create_signal(45.0);
    let (length_factor, set_length_factor) = create_signal(1.2);

    let draw_linden_system = move |ev: MouseEvent| {
        ev.prevent_default();
    };

    view! {
        <div class="rule-box">
            <SimpleStringInput input_id="Axiom" reader=axiom writer=set_axiom/>
            <SimpleFloatInput input_id="Angle" reader=angle writer=set_angle max="360"/>
            <SimpleFloatInput input_id="Length Factor" reader=length_factor writer=set_length_factor max="10"/>
            <RuleList/>
            <button on:click=draw_linden_system>"Draw!"</button>
        </div>
    }
}

// SimpleStringInput()
// 
#[component]
fn SimpleStringInput(
    input_id: &'static str,
    reader: ReadSignal<String>,
    writer: WriteSignal<String>
) -> impl IntoView {
    let on_input = move |ev: Event| {
        ev.prevent_default();
        writer.set(event_target_value(&ev));
    };
    view! {
        <label for=input_id>{input_id}": "</label>
        <input id=input_id type="text"
            on:input=on_input
            prop:value=reader
        ></input>
        <p>"Test "{reader}</p>
    }
}

#[component]
fn SimpleFloatInput(
    input_id: &'static str,
    reader: ReadSignal<f64>,
    writer: WriteSignal<f64>,
    #[prop(default = "0")] min: &'static str,
    #[prop(default = "100")] max: &'static str
) -> impl IntoView {
    let on_change = move |ev: Event| {
        ev.prevent_default();
        match event_target_value(&ev).parse::<f64>() {
            Ok(val) => writer.set(val),
            Err(_) => writer.set(0.0)
        }
    };
    view! {
        <label for=input_id>{input_id}": "</label>
        <input id=input_id type="number" min=min max=max step="any"
            on:change=on_change
            prop:value=reader
        ></input>
        <p>"Test "{reader}</p>
    }
}

#[derive(Copy, Clone)]
struct RuleContext(ReadSignal<Vec<(i32, (ReadSignal<String>, WriteSignal<String>), (ReadSignal<String>, WriteSignal<String>))>>);
#[derive(Copy, Clone)]
struct SetRuleContext(WriteSignal<Vec<(i32, (ReadSignal<String>, WriteSignal<String>), (ReadSignal<String>, WriteSignal<String>))>>);

#[component]
fn RuleList() -> impl IntoView {
    let rules = use_context::<RuleContext>().unwrap().0;
    let set_rules = use_context::<SetRuleContext>().unwrap().0;

    let create_new_rule = move |ev: MouseEvent| {
        ev.prevent_default();
        let rule_id = match rules.get().last() {
            Some(p) => p.0 + 1i32,
            None => 1,
        };
        let new_rule =(rule_id,  create_signal("".to_string()), create_signal("".to_string()));
        set_rules.update(|vec| vec.push(new_rule));
    };

    view! {
        <ul class="border-solid border-2 border-indigo-600 p-2">
            <For
                each=move || rules.get()
                key=|rule| rule.0
                children=move |(id, (pred, _set_pred),(conv, _set_conv))| {
                    view! {
                        <li>
                            <input type="text" value=pred/>
                            <input type="text" value=conv/>
                            <button
                                class="delete-button"
                                on:click=move |_| {
                                    set_rules.update(|rules| {
                                        rules.retain(|(rule_id,(pred_sig, set_pred_sig),(conv_sig, set_conv_sig))| {
                                        // Must dispose of signals manually or else they will exist until dismount e.g. mem leak.
                                        // "This is only necessary in an example with nested signals like this one"
                                        // source- https://book.leptos.dev/view/04_iteration.html
                                            if rule_id == &id {
                                                pred_sig.dispose();
                                                set_pred_sig.dispose();
                                                conv_sig.dispose();
                                                set_conv_sig.dispose();
                                            }
                                            rule_id != &id
                                        })
                                    });
                                }
                            >X</button>
                        </li>
                    }
                }
            />
            <button on:click=create_new_rule>New Rule</button>
        </ul>
    }
}
