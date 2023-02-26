use crate::components::{new_todo::*, todo_list::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/todomvc-common@1.0.5/base.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/todomvc-app-css@2.3.0/index.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(
        cx,
        vec![],
    );

    let add_new_todo = move |title: String| {
        let todo = Todo::new(cx, title);
        set_todos.update(|todos| todos.push(todo));
    };

    view! { cx,
        <div class="todomvc-wrapper">
            <section class="todoapp">
                <header class="header">
                    <h1>"todos"</h1>
                    <NewTodoInput on_todo_entered={add_new_todo} />
                </header>
                <section class="main">
                    <input id="toggle-all" class="toggle-all" type="checkbox" />
                    <label for="toggle-all" />
                    <TodoList todos={todos} />
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>0</strong>" item(s) left"
                    </span>
                    <ul class="filters">
                        <li><a class="selected" href="#/all">{"All"}</a></li>
                        <li><a class="not-selected" href="#/active">{"Active"}</a></li>
                        <li><a class="not-selected" href="#/completed">{"Completed"}</a></li>
                    </ul>
                    <button class="clear-completed">
                        { format!("Clear completed ({})", 0) }
                    </button>
                </footer>
            </section>
            <footer class="info">
                <p>"Double-click to edit a todo"</p>
            </footer>
        </div>
    }
}
