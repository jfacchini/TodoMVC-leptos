use leptos::*;
use uuid::Uuid;

#[component]
pub fn TodoList(cx: Scope, todos: ReadSignal<Vec<Todo>>) -> impl IntoView {
    view! {
        cx,
        <ul class="todo-list">
            <For
                each=move || todos.get().clone()
                key=|todo| todo.id
                view= move |cx, todo: Todo| view! {cx, <Todo todo />}
            />
        </ul>
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Mode {
    New,
    Completed,
    Editing,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: RwSignal<String>,
    pub mode: RwSignal<Mode>,
}

impl Todo {
    pub fn new(cx: Scope, title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: create_rw_signal(cx, title.into()),
            mode: create_rw_signal(cx, Mode::New),
        }
    }
}

#[component]
pub fn Todo(cx: Scope, todo: Todo) -> impl IntoView {
    let is_completed = matches!(todo.mode.get(), Mode::Completed);
    let is_editing = matches!(todo.mode.get(), Mode::Editing);
    view! {
        cx,
        <li class:completed={is_completed}
            class:editing={is_editing}
        >
            <div class="view">
                <input type="checkbox" class="toggle" prop:checked={is_completed} />
                <label>{todo.title.get()}</label>
                <button class="destroy" />
            </div>
            <input class="edit" type="text" prop:value={todo.title.get()} hidden={!is_editing} />
        </li>
    }
}
