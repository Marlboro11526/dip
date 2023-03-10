use crate::ui_state::UiTodo;
use dip::prelude::*;

// Internal events (System -> System)

pub struct UpdateTodoMeta {
    pub entity: Entity,
}

pub struct NewUiTodoListRequested;

pub struct NewUiTodoListReady {
    pub todo_list: Vec<UiTodo>,
}
