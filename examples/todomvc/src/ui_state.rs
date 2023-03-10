use crate::component::*;
use chrono::{DateTime, Utc};
use dip::prelude::*;

#[ui_state]
pub struct UiState {
    todo_list: Vec<UiTodo>,
    filter: Filter,
}

#[derive(Clone, Debug)]
pub struct UiTodo {
    pub entity: Entity,
    pub title: String,
    pub done_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<(Entity, &Title, Option<&DoneAt>, &Timestamp)> for UiTodo {
    fn from(
        (entity, title, done_at, timestamp): (Entity, &Title, Option<&DoneAt>, &Timestamp),
    ) -> Self {
        Self {
            entity,
            title: title.value.clone(),
            done_at: match done_at {
                Some(done_at) => Some(done_at.time),
                None => None,
            },
            created_at: timestamp.created_at,
            updated_at: timestamp.updated_at,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

// Actions
#[derive(Clone, Debug)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct ChangeTitle {
    pub entity: Entity,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct ToggleDone {
    pub entity: Entity,
}

#[derive(Clone, Debug)]
pub struct RemoveTodo {
    pub entity: Entity,
}

#[derive(Clone, Debug)]
pub struct ToggleAll;

#[derive(Clone, Debug)]
pub struct ChangeFilter {
    pub filter: Filter,
}

#[derive(Clone, Debug)]
pub struct ClearCompleted;

#[ui_action]
impl ActionCreator {
    pub fn create_todo(title: &String) -> CreateTodo {
        CreateTodo {
            title: title.clone(),
        }
    }

    pub fn change_title(entity: &Entity, title: &String) -> ChangeTitle {
        ChangeTitle {
            entity: entity.clone(),
            title: title.clone(),
        }
    }

    pub fn toggle_done(entity: &Entity) -> ToggleDone {
        ToggleDone {
            entity: entity.clone(),
        }
    }

    pub fn remove_todo(entity: &Entity) -> RemoveTodo {
        RemoveTodo {
            entity: entity.clone(),
        }
    }

    pub fn filter_all() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::All,
        }
    }

    pub fn filter_active() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::Active,
        }
    }

    pub fn filter_completed() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::Completed,
        }
    }

    pub fn toggle_all() -> ToggleAll {
        ToggleAll
    }

    pub fn clear_completed() -> ClearCompleted {
        ClearCompleted
    }
}
