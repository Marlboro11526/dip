use dip::{
    bevy::{input::keyboard::KeyboardInput, log::LogPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .insert_non_send_resource(DesktopSettings::<NoRootProps> {
            keyboard_event: true,
            ..Default::default()
        })
        .add_plugin(DesktopPlugin::<UiState, UiAction, NoAsyncAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_ui_action)
        .add_system(send_ui_state)
        .add_system(log_keyboard_event)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let event_type = use_read(&cx, EVENT_TYPE);
    let input_result = use_read(&cx, INPUT_RESULT);
    let window = use_window::<UiAction, NoAsyncAction>(&cx);

    cx.render(rsx! {
        h1 { "Keyboard Event Example" }
        p { "💡 Type any keys and checkout console. (TODO: You might need to click screen to focus.)" }

        div {
            select {
                value: format_args!("{:?}", event_type),
                onchange: |e| {
                    match e.value.as_str() {
                        "KeyboardEvent" => {
                            window.send(UiAction::keyboard_event());
                        },
                        "KeyboardInput" => {
                            window.send(UiAction::keyboard_input());
                        },
                        "ReceivedCharacter" => {
                            window.send(UiAction::received_char());
                        }
                        _ => {}
                    };
                },

                option {
                    value: "KeyboardEvent",
                    "KeyboardEvent"
                }
                option {
                    value: "KeyboardInput",
                    "KeyboardInput"
                }
                option {
                    value: "ReceivedCharacter",
                    "ReceivedCharacter"
                }
            }
        }

        code {
            [format_args!("Input result: {:#?}", input_result)],
        }
    })
}

#[ui_state]
struct UiState {
    event_type: EventType,
    input_result: InputResult,
}

// UI -> ECS
#[ui_action]
impl ActionCreator {
    fn keyboard_event() -> EventType {
        EventType::KeyboardEvent
    }

    fn keyboard_input() -> EventType {
        EventType::KeyboardInput
    }

    fn received_char() -> EventType {
        EventType::ReceivedCharacter
    }
}

#[derive(Clone, Debug)]
pub enum InputResult {
    KeyboardEvent(KeyboardEvent),
    KeyboardInput(KeyboardInput),
    ReceivedCharacter(ReceivedCharacter),
    None,
}

impl Default for InputResult {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub enum EventType {
    KeyboardEvent,
    KeyboardInput,
    ReceivedCharacter,
}

impl Default for EventType {
    fn default() -> Self {
        Self::KeyboardEvent
    }
}

fn handle_ui_action(mut events: EventReader<UiAction>, mut event_type: ResMut<EventType>) {
    for action in events.iter() {
        match action {
            UiAction::EventType(e) => {
                info!("🧠　EventType: {:?}", e);
                *event_type = e.clone();
            }
        }
    }
}

fn send_ui_state(
    event_type: Res<EventType>,
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut keyboard_inputs: EventReader<KeyboardInput>,
    mut received_characters: EventReader<ReceivedCharacter>,
    mut input_result: ResMut<InputResult>,
) {
    match *event_type {
        EventType::KeyboardEvent => {
            for e in keyboard_events.iter() {
                *input_result = InputResult::KeyboardEvent(e.clone());
            }
        }
        EventType::KeyboardInput => {
            for e in keyboard_inputs.iter() {
                *input_result = InputResult::KeyboardInput(e.clone());
            }
        }
        EventType::ReceivedCharacter => {
            for e in received_characters.iter() {
                *input_result = InputResult::ReceivedCharacter(e.clone());
            }
        }
    };
}

fn log_keyboard_event(
    event_type: Res<EventType>,
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut received_character_events: EventReader<ReceivedCharacter>,
) {
    match *event_type {
        EventType::KeyboardEvent => {
            for event in keyboard_events.iter() {
                info!("🧠 {:?}", event.clone());
            }
        }
        EventType::KeyboardInput => {
            for input in keyboard_input_events.iter() {
                info!("🧠 {:?}", input.clone());
            }
        }
        EventType::ReceivedCharacter => {
            for received_char in received_character_events.iter() {
                info!("🧠 {:?}", received_char.clone());
            }
        }
    }
}
