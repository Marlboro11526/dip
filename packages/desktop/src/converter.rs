use crate::event::Location;
use bevy::{input::keyboard::KeyCode, window::CursorIcon};
use dioxus::events::*;
use std::{any::Any, sync::Arc};
use wry::application::window::CursorIcon as TaoCursorIcon;

pub fn convert_cursor_icon(cursor_icon: CursorIcon) -> TaoCursorIcon {
    match cursor_icon {
        CursorIcon::Default => TaoCursorIcon::Default,
        CursorIcon::Crosshair => TaoCursorIcon::Crosshair,
        CursorIcon::Hand => TaoCursorIcon::Hand,
        CursorIcon::Arrow => TaoCursorIcon::Arrow,
        CursorIcon::Move => TaoCursorIcon::Move,
        CursorIcon::Text => TaoCursorIcon::Text,
        CursorIcon::Wait => TaoCursorIcon::Wait,
        CursorIcon::Help => TaoCursorIcon::Help,
        CursorIcon::Progress => TaoCursorIcon::Progress,
        CursorIcon::NotAllowed => TaoCursorIcon::NotAllowed,
        CursorIcon::ContextMenu => TaoCursorIcon::ContextMenu,
        CursorIcon::Cell => TaoCursorIcon::Cell,
        CursorIcon::VerticalText => TaoCursorIcon::VerticalText,
        CursorIcon::Alias => TaoCursorIcon::Alias,
        CursorIcon::Copy => TaoCursorIcon::Copy,
        CursorIcon::NoDrop => TaoCursorIcon::NoDrop,
        CursorIcon::Grab => TaoCursorIcon::Grab,
        CursorIcon::Grabbing => TaoCursorIcon::Grabbing,
        CursorIcon::AllScroll => TaoCursorIcon::AllScroll,
        CursorIcon::ZoomIn => TaoCursorIcon::ZoomIn,
        CursorIcon::ZoomOut => TaoCursorIcon::ZoomOut,
        CursorIcon::EResize => TaoCursorIcon::EResize,
        CursorIcon::NResize => TaoCursorIcon::NResize,
        CursorIcon::NeResize => TaoCursorIcon::NeResize,
        CursorIcon::NwResize => TaoCursorIcon::NwResize,
        CursorIcon::SResize => TaoCursorIcon::SResize,
        CursorIcon::SeResize => TaoCursorIcon::SeResize,
        CursorIcon::SwResize => TaoCursorIcon::SwResize,
        CursorIcon::WResize => TaoCursorIcon::WResize,
        CursorIcon::EwResize => TaoCursorIcon::EwResize,
        CursorIcon::NsResize => TaoCursorIcon::NsResize,
        CursorIcon::NeswResize => TaoCursorIcon::NeswResize,
        CursorIcon::NwseResize => TaoCursorIcon::NwseResize,
        CursorIcon::ColResize => TaoCursorIcon::ColResize,
        CursorIcon::RowResize => TaoCursorIcon::RowResize,
    }
}

// reference: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values
pub fn try_convert_key_code(key: &String, location: &Location) -> Option<KeyCode> {
    match (key.as_str(), location) {
        ("0" | ")", Location::Standard) => Some(KeyCode::Key0),
        ("1" | "!", Location::Standard) => Some(KeyCode::Key1),
        ("2" | "@", Location::Standard) => Some(KeyCode::Key2),
        ("3" | "#", Location::Standard) => Some(KeyCode::Key3),
        ("4" | "$", Location::Standard) => Some(KeyCode::Key4),
        ("5" | "%", Location::Standard) => Some(KeyCode::Key5),
        ("6" | "^", Location::Standard) => Some(KeyCode::Key6),
        ("7" | "&", Location::Standard) => Some(KeyCode::Key7),
        ("8" | "*", Location::Standard) => Some(KeyCode::Key8),
        ("9" | "(", Location::Standard) => Some(KeyCode::Key9),

        ("a" | "A", _) => Some(KeyCode::A),
        ("b" | "B", _) => Some(KeyCode::B),
        ("c" | "C", _) => Some(KeyCode::C),
        ("d" | "D", _) => Some(KeyCode::D),
        ("e" | "E", _) => Some(KeyCode::E),
        ("f" | "F", _) => Some(KeyCode::F),
        ("g" | "G", _) => Some(KeyCode::G),
        ("h" | "H", _) => Some(KeyCode::H),
        ("i" | "I", _) => Some(KeyCode::I),
        ("j" | "J", _) => Some(KeyCode::J),
        ("k" | "K", _) => Some(KeyCode::K),
        ("l" | "L", _) => Some(KeyCode::L),
        ("m" | "M", _) => Some(KeyCode::M),
        ("n" | "N", _) => Some(KeyCode::N),
        ("o" | "O", _) => Some(KeyCode::O),
        ("p" | "P", _) => Some(KeyCode::P),
        ("q" | "Q", _) => Some(KeyCode::Q),
        ("r" | "R", _) => Some(KeyCode::R),
        ("s" | "S", _) => Some(KeyCode::S),
        ("t" | "T", _) => Some(KeyCode::T),
        ("u" | "U", _) => Some(KeyCode::U),
        ("v" | "V", _) => Some(KeyCode::V),
        ("w" | "W", _) => Some(KeyCode::W),
        ("x" | "X", _) => Some(KeyCode::X),
        ("y" | "Y", _) => Some(KeyCode::Y),
        ("z" | "Z", _) => Some(KeyCode::Z),

        ("Escape", _) => Some(KeyCode::Escape),

        ("F1", _) => Some(KeyCode::F1),
        ("F2", _) => Some(KeyCode::F2),
        ("F3", _) => Some(KeyCode::F3),
        ("F4", _) => Some(KeyCode::F4),
        ("F5", _) => Some(KeyCode::F5),
        ("F6", _) => Some(KeyCode::F6),
        ("F7", _) => Some(KeyCode::F7),
        ("F8", _) => Some(KeyCode::F8),
        ("F9", _) => Some(KeyCode::F9),
        ("F10", _) => Some(KeyCode::F10),
        ("F11", _) => Some(KeyCode::F11),
        ("F12", _) => Some(KeyCode::F12),
        ("F13", _) => Some(KeyCode::F13),
        ("F14", _) => Some(KeyCode::F14),
        ("F15", _) => Some(KeyCode::F15),
        ("F16", _) => Some(KeyCode::F16),
        ("F17", _) => Some(KeyCode::F17),
        ("F18", _) => Some(KeyCode::F18),
        ("F19", _) => Some(KeyCode::F19),
        ("F20", _) => Some(KeyCode::F20),
        ("F21", _) => Some(KeyCode::F21),
        ("F22", _) => Some(KeyCode::F22),
        ("F23", _) => Some(KeyCode::F23),
        ("F24", _) => Some(KeyCode::F24),

        ("PrintScreen", _) => Some(KeyCode::Snapshot),
        ("ScrollLock", _) => Some(KeyCode::Scroll),
        ("Pause", _) => Some(KeyCode::Pause),

        ("Insert", _) => Some(KeyCode::Insert),
        ("Home", _) => Some(KeyCode::Home),
        ("Delete", _) => Some(KeyCode::Delete),
        ("End", _) => Some(KeyCode::Delete),
        ("PageDown", _) => Some(KeyCode::PageDown),
        ("PageUp", _) => Some(KeyCode::PageUp),

        ("Left", _) | ("ArrowLeft", _) => Some(KeyCode::Left),
        ("Up", _) | ("ArrowUp", _) => Some(KeyCode::Up),
        ("Right", _) | ("ArrowRight", _) => Some(KeyCode::Right),
        ("Down", _) | ("ArrowDown", _) => Some(KeyCode::Down),

        ("Backspace", _) => Some(KeyCode::Back),
        ("Enter", Location::Standard) => Some(KeyCode::Return),
        (" ", _) => Some(KeyCode::Space),

        ("Compose", _) => Some(KeyCode::Compose),

        // Caret,
        ("NumLock", _) => Some(KeyCode::Numlock),
        ("0", Location::Numpad) => Some(KeyCode::Numpad0),
        ("1", Location::Numpad) => Some(KeyCode::Numpad1),
        ("2", Location::Numpad) => Some(KeyCode::Numpad2),
        ("3", Location::Numpad) => Some(KeyCode::Numpad3),
        ("4", Location::Numpad) => Some(KeyCode::Numpad4),
        ("5", Location::Numpad) => Some(KeyCode::Numpad5),
        ("6", Location::Numpad) => Some(KeyCode::Numpad6),
        ("7", Location::Numpad) => Some(KeyCode::Numpad7),
        ("8", Location::Numpad) => Some(KeyCode::Numpad8),
        ("9", Location::Numpad) => Some(KeyCode::Numpad9),

        // AbntC1,
        // AbntC2,
        ("NumpadAdd", _) => Some(KeyCode::NumpadAdd),
        ("'" | "\"", _) => Some(KeyCode::Apostrophe),
        // Apps,
        // Asterisk,
        // Plus,
        // At,
        // Ax,
        ("\\" | "|", _) => Some(KeyCode::Backslash),
        // Calculator,
        // Capital,
        // Colon,
        ("," | "<", _) => Some(KeyCode::Comma),
        ("Convert", _) => Some(KeyCode::Convert),
        ("NumpadDecimal", _) => Some(KeyCode::NumpadDecimal),
        ("NumpadDivide", _) => Some(KeyCode::NumpadDivide),
        ("=" | "+", _) => Some(KeyCode::Equals),
        ("`" | "~", _) => Some(KeyCode::Grave),
        // Kana,
        // Kanji,
        ("Alt", Location::Left) => Some(KeyCode::LAlt),
        ("Bracket", Location::Left) => Some(KeyCode::LBracket),
        ("Control", Location::Left) => Some(KeyCode::LControl),
        ("Shift", Location::Left) => Some(KeyCode::LShift),
        ("Meta", Location::Left) => Some(KeyCode::LWin),
        // Mail,
        // MediaSelect,
        // MediaStop,
        ("-" | "_", _) => Some(KeyCode::Minus),
        ("Multiply", Location::Numpad) => Some(KeyCode::NumpadMultiply),
        // Mute,
        // MyComputer,
        // "BrowserForward" => Some(KeyCode::NavigateForward),
        // "BrowserBackward" => Some(KeyCode::NavigateBackward),
        // NextTrack,
        ("NonConvert", _) => Some(KeyCode::NoConvert),
        ("Comma", Location::Numpad) => Some(KeyCode::NumpadComma),
        ("Enter", Location::Numpad) => Some(KeyCode::NumpadEnter),
        ("Equal", Location::Numpad) => Some(KeyCode::NumpadEquals),
        // Oem102,
        ("." | ">", _) => Some(KeyCode::Period),
        // PlayPause,
        ("Power", _) => Some(KeyCode::Power),
        // PrevTrack,
        ("Alt", Location::Right) => Some(KeyCode::RAlt),
        ("Bracket", Location::Right) => Some(KeyCode::RBracket),
        ("Control", Location::Right) => Some(KeyCode::RControl),
        ("Shift", Location::Right) => Some(KeyCode::RShift),
        ("Meta", Location::Right) => Some(KeyCode::RWin),
        (";" | ":", _) => Some(KeyCode::Semicolon),
        ("/" | "?", _) => Some(KeyCode::Slash),
        // Sleep,
        // Stop,
        ("NumpadSubtract", _) => Some(KeyCode::NumpadSubtract),
        // Sysrq,
        ("Tab", _) => Some(KeyCode::Tab),
        // Underline,
        // Unlabeled,
        // VolumeDown,
        // VolumeUp,
        // Wake,
        // WebBack,
        // WebFavorites,
        // WebForward,
        // WebHome,
        // WebRefresh,
        // WebSearch,
        // WebStop,
        ("IntlYen", _) => Some(KeyCode::Yen),
        ("Copy", _) => Some(KeyCode::Copy),
        ("Paste", _) => Some(KeyCode::Paste),
        ("Cut", _) => Some(KeyCode::Cut),

        _ => None,
    }
}

pub fn convert_synthetic_event(name: &str, val: serde_json::Value) -> Arc<dyn Any + Send + Sync> {
    match name {
        "copy" | "cut" | "paste" => {
            //
            Arc::new(ClipboardData {})
        }
        "compositionend" | "compositionstart" | "compositionupdate" => {
            Arc::new(serde_json::from_value::<dioxus::events::CompositionData>(val).unwrap())
        }
        "keydown" | "keypress" | "keyup" => {
            let evt = serde_json::from_value::<KeyboardData>(val).unwrap();
            Arc::new(evt)
        }
        "focus" | "blur" | "focusout" | "focusin" => {
            //
            Arc::new(FocusData {})
        }

        // todo: these handlers might get really slow if the input box gets large and allocation pressure is heavy
        // don't have a good solution with the serialized event problem
        "change" | "input" | "invalid" | "reset" | "submit" => {
            Arc::new(serde_json::from_value::<FormData>(val).unwrap())
        }

        "click" | "contextmenu" | "dblclick" | "doubleclick" | "drag" | "dragend" | "dragenter"
        | "dragexit" | "dragleave" | "dragover" | "dragstart" | "drop" | "mousedown"
        | "mouseenter" | "mouseleave" | "mousemove" | "mouseout" | "mouseover" | "mouseup" => {
            Arc::new(serde_json::from_value::<MouseData>(val).unwrap())
        }
        "pointerdown" | "pointermove" | "pointerup" | "pointercancel" | "gotpointercapture"
        | "lostpointercapture" | "pointerenter" | "pointerleave" | "pointerover" | "pointerout" => {
            Arc::new(serde_json::from_value::<PointerData>(val).unwrap())
        }
        "select" => {
            //
            Arc::new(serde_json::from_value::<SelectionData>(val).unwrap())
        }

        "touchcancel" | "touchend" | "touchmove" | "touchstart" => {
            Arc::new(serde_json::from_value::<TouchData>(val).unwrap())
        }

        "scroll" => Arc::new(()),

        "wheel" => Arc::new(serde_json::from_value::<WheelData>(val).unwrap()),

        "animationstart" | "animationend" | "animationiteration" => {
            Arc::new(serde_json::from_value::<AnimationData>(val).unwrap())
        }

        "transitionend" => Arc::new(serde_json::from_value::<TransitionData>(val).unwrap()),

        "abort" | "canplay" | "canplaythrough" | "durationchange" | "emptied" | "encrypted"
        | "ended" | "error" | "loadeddata" | "loadedmetadata" | "loadstart" | "pause" | "play"
        | "playing" | "progress" | "ratechange" | "seeked" | "seeking" | "stalled" | "suspend"
        | "timeupdate" | "volumechange" | "waiting" => {
            //
            Arc::new(MediaData {})
        }

        "toggle" => Arc::new(ToggleData {}),

        _ => Arc::new(()),
    }
}

pub fn convert_event_type_to_name(r#type: &str) -> &'static str {
    match r#type {
        "copy" => "copy",
        "cut" => "cut",
        "paste" => "paste",
        "compositionend" => "compositionend",
        "compositionstart" => "compositionstart",
        "compositionupdate" => "compositionupdate",
        "keydown" => "keydown",
        "keypress" => "keypress",
        "keyup" => "keyup",
        "focus" => "focus",
        "focusout" => "focusout",
        "focusin" => "focusin",
        "blur" => "blur",
        "change" => "change",
        "input" => "input",
        "invalid" => "invalid",
        "reset" => "reset",
        "submit" => "submit",
        "click" => "click",
        "contextmenu" => "contextmenu",
        "doubleclick" => "doubleclick",
        "dblclick" => "dblclick",
        "drag" => "drag",
        "dragend" => "dragend",
        "dragenter" => "dragenter",
        "dragexit" => "dragexit",
        "dragleave" => "dragleave",
        "dragover" => "dragover",
        "dragstart" => "dragstart",
        "drop" => "drop",
        "mousedown" => "mousedown",
        "mouseenter" => "mouseenter",
        "mouseleave" => "mouseleave",
        "mousemove" => "mousemove",
        "mouseout" => "mouseout",
        "mouseover" => "mouseover",
        "mouseup" => "mouseup",
        "pointerdown" => "pointerdown",
        "pointermove" => "pointermove",
        "pointerup" => "pointerup",
        "pointercancel" => "pointercancel",
        "gotpointercapture" => "gotpointercapture",
        "lostpointercapture" => "lostpointercapture",
        "pointerenter" => "pointerenter",
        "pointerleave" => "pointerleave",
        "pointerover" => "pointerover",
        "pointerout" => "pointerout",
        "select" => "select",
        "touchcancel" => "touchcancel",
        "touchend" => "touchend",
        "touchmove" => "touchmove",
        "touchstart" => "touchstart",
        "scroll" => "scroll",
        "wheel" => "wheel",
        "animationstart" => "animationstart",
        "animationend" => "animationend",
        "animationiteration" => "animationiteration",
        "transitionend" => "transitionend",
        "abort" => "abort",
        "canplay" => "canplay",
        "canplaythrough" => "canplaythrough",
        "durationchange" => "durationchange",
        "emptied" => "emptied",
        "encrypted" => "encrypted",
        "ended" => "ended",
        "error" => "error",
        "loadeddata" => "loadeddata",
        "loadedmetadata" => "loadedmetadata",
        "loadstart" => "loadstart",
        "pause" => "pause",
        "play" => "play",
        "playing" => "playing",
        "progress" => "progress",
        "ratechange" => "ratechange",
        "seeked" => "seeked",
        "seeking" => "seeking",
        "stalled" => "stalled",
        "suspend" => "suspend",
        "timeupdate" => "timeupdate",
        "volumechange" => "volumechange",
        "waiting" => "waiting",
        "toggle" => "toggle",
        x => {
            panic!("unsupported event type {:?}", x);
        }
    }
}
