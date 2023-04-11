use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

#[derive(Properties, PartialEq)]
struct TodoItemProps {
    todo: Todo,
    on_toggle: Callback<ToggleTodo>,
    on_remove: Callback<usize>,
}

#[function_component(TodoItem)]
fn todo_item(
    TodoItemProps {
        todo,
        on_toggle,
        on_remove,
    }: &TodoItemProps,
) -> Html {
    let on_change = {
        let todo = todo.clone();
        let on_toggle = on_toggle.clone();

        Callback::from(move |evt: Event| {
            let input = evt
                .target()
                .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
                .unwrap_throw();

            on_toggle.emit(ToggleTodo {
                id: todo.clone().id,
                done: input.checked(),
            })
        })
    };

    let todo_id = todo.id;

    let on_remove = {
        let on_remove = on_remove.clone();

        move |_| {
            on_remove.emit(todo_id);
        }
    };

    html! {
        <li key={todo.id} class={classes!("w-full flex-1".to_string())}>
            <label
                class={classes!(
                    "w-full flex gap-2 items-center cursor-pointer justify-between".to_string(),
                    todo.done.then_some("line-through"),
                )}
            >
                <div class={classes!("flex gap-2".to_string())}>
                    <input type={"checkbox"} onchange={on_change} checked={todo.done} />
                    <span>{todo.text.to_string()}</span>
                </div>
                <button onclick={on_remove} class={classes!("btn btn-xs btn-circle btn-ghost text-red-400 text-lg leading-3".to_string())}>{"×"}</button>
            </label>
        </li>
    }
}

struct ToggleTodo {
    id: usize,
    done: bool,
}

#[derive(Clone, PartialEq)]
enum Filter {
    All,
    Pending,
    Done,
}

fn get_filter_button_class(btn: Filter, active: &Filter) -> String {
    match (btn, active) {
        (Filter::All, Filter::All) => "btn-primary".to_string(),
        (Filter::Pending, Filter::Pending) => "btn-primary".to_string(),
        (Filter::Done, Filter::Done) => "btn-primary".to_string(),
        _ => "btn-outline border-primary".to_string(),
    }
}

#[function_component(App)]
fn app() -> Html {
    let todos: UseStateHandle<Vec<Todo>> = use_state(|| {
        vec![
            Todo {
                id: 1,
                text: "First".to_string(),
                done: false,
            },
            Todo {
                id: 2,
                text: "Second".to_string(),
                done: false,
            },
        ]
    });
    let next_todo = use_state(|| String::default());
    let filter = use_state(|| Filter::All);
    let next_id = todos.len();

    let on_toggle: Callback<ToggleTodo> = {
        let todos = todos.clone();

        Callback::from(move |ToggleTodo { id, done }| {
            let next_todos = todos
                .iter()
                .map(|todo| {
                    if todo.id == id {
                        return Todo {
                            id,
                            text: todo.text.to_string(),
                            done,
                        };
                    }

                    todo.clone()
                })
                .collect();

            todos.set(next_todos);
        })
    };

    let on_input = {
        let next_todo = next_todo.clone();

        Callback::from(move |evt: InputEvent| {
            let input = evt
                .target()
                .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
                .unwrap_throw();

            next_todo.set(input.value());
        })
    };

    let on_add = {
        let next_todo = next_todo.clone();
        let todos = todos.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            let mut next = todos.to_vec();

            next.push(Todo {
                id: next_id,
                text: next_todo.to_string(),
                done: false,
            });

            todos.set(next);
            next_todo.set(String::default());
        })
    };

    let on_remove = {
        let todos = todos.clone();

        Callback::from(move |id: usize| {
            let next: Vec<Todo> = todos.iter().filter(|todo| todo.id != id).cloned().collect();

            todos.set(next);
        })
    };

    let on_filter_all = {
        let filter = filter.clone();
        Callback::from(move |_| filter.set(Filter::All))
    };

    let on_filter_pending = {
        let filter = filter.clone();
        Callback::from(move |_| filter.set(Filter::Pending))
    };

    let on_filter_done = {
        let filter = filter.clone();
        Callback::from(move |_| filter.set(Filter::Done))
    };

    let filtered_todos = todos.iter().filter_map(|todo| match *filter {
        Filter::Done if !todo.done => None,
        Filter::Pending if todo.done => None,
        _ => Some(html! {
            <TodoItem key={todo.id} todo={todo.clone()} on_toggle={on_toggle.clone()} on_remove={on_remove.clone()} />
        }),
    });

    html! {
        <div class={classes!("p-4 flex flex-col gap-3".to_string())}>
            <h1 class={classes!("text-3xl font-bold".to_string())}>{"Todos"}</h1>

            <div class={classes!("flex btn-group".to_string())}>
               <button onclick={on_filter_all} class={classes!("btn btn-sm".to_string(), get_filter_button_class(Filter::All, &filter))}>{"All"}</button>
               <button onclick={on_filter_pending} class={classes!("btn btn-sm".to_string(), get_filter_button_class(Filter::Pending, &filter))}>{"Pending"}</button>
               <button onclick={on_filter_done} class={classes!("btn btn-sm".to_string(), get_filter_button_class(Filter::Done, &filter))}>{"Done"}</button>
            </div>

            <ul class={classes!("flex w-[300px] flex-col".to_string())}>
                if filtered_todos.clone().count() == 0 {
                    <p>{"Nothing here"}</p>
                } else {
                    {for filtered_todos}
                }

                <li class={classes!("flex-1 mt-2".to_string())}>
                    <form onsubmit={on_add}>
                        <input type="text" class={classes!("p-1 rounded w-full".to_string())} value={next_todo.to_string()} oninput={on_input} autofocus={true} placeholder="Add todo..." />
                    </form>
                </li>
            </ul>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
