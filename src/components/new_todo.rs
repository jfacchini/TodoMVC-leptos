use leptos::*;
use leptos::ev::KeyboardEvent;
use leptos::html::Input;

#[component]
pub fn NewTodoInput<F>(cx: Scope, on_todo_entered: F) -> impl IntoView
    where
        F: FnMut(String) + 'static,
{
    let new_todo_ref = create_node_ref::<Input>(cx);
    let mut on_todo_entered = on_todo_entered;
    let onkeypress = {
        move |e: KeyboardEvent| {
            let input = new_todo_ref.get().unwrap();
            if e.key() == "Enter" {
                let value = input.value();
                on_todo_entered(value);
                input.set_value("");
            }
        }
    };

    view! {
        cx,
        <input
            class="new-todo"
            placeholder="What needs to be done?"
            on:keypress=onkeypress
            node_ref=new_todo_ref
        />
    }
}
