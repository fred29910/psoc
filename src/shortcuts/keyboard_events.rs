//! Keyboard event conversion utilities
//!
//! This module provides utilities to convert iced keyboard events
//! to our internal shortcut system types.

use super::shortcut_types::{ShortcutKey, ShortcutModifiers};

/// Convert iced keyboard key to our ShortcutKey
pub fn iced_key_to_shortcut_key(key: &iced::keyboard::Key) -> Option<ShortcutKey> {
    match key {
        iced::keyboard::Key::Character(c) => {
            // Convert to lowercase for consistency
            if let Some(ch) = c.chars().next() {
                Some(ShortcutKey::Character(ch.to_ascii_lowercase()))
            } else {
                None
            }
        }
        iced::keyboard::Key::Named(named_key) => match named_key {
            iced::keyboard::key::Named::Escape => Some(ShortcutKey::Escape),
            iced::keyboard::key::Named::Enter => Some(ShortcutKey::Enter),
            iced::keyboard::key::Named::Space => Some(ShortcutKey::Space),
            iced::keyboard::key::Named::Tab => Some(ShortcutKey::Tab),
            iced::keyboard::key::Named::Backspace => Some(ShortcutKey::Backspace),
            iced::keyboard::key::Named::Delete => Some(ShortcutKey::Delete),
            iced::keyboard::key::Named::Insert => Some(ShortcutKey::Insert),
            iced::keyboard::key::Named::Home => Some(ShortcutKey::Home),
            iced::keyboard::key::Named::End => Some(ShortcutKey::End),
            iced::keyboard::key::Named::PageUp => Some(ShortcutKey::PageUp),
            iced::keyboard::key::Named::PageDown => Some(ShortcutKey::PageDown),
            iced::keyboard::key::Named::ArrowUp => Some(ShortcutKey::ArrowUp),
            iced::keyboard::key::Named::ArrowDown => Some(ShortcutKey::ArrowDown),
            iced::keyboard::key::Named::ArrowLeft => Some(ShortcutKey::ArrowLeft),
            iced::keyboard::key::Named::ArrowRight => Some(ShortcutKey::ArrowRight),
            iced::keyboard::key::Named::F1 => Some(ShortcutKey::Function(1)),
            iced::keyboard::key::Named::F2 => Some(ShortcutKey::Function(2)),
            iced::keyboard::key::Named::F3 => Some(ShortcutKey::Function(3)),
            iced::keyboard::key::Named::F4 => Some(ShortcutKey::Function(4)),
            iced::keyboard::key::Named::F5 => Some(ShortcutKey::Function(5)),
            iced::keyboard::key::Named::F6 => Some(ShortcutKey::Function(6)),
            iced::keyboard::key::Named::F7 => Some(ShortcutKey::Function(7)),
            iced::keyboard::key::Named::F8 => Some(ShortcutKey::Function(8)),
            iced::keyboard::key::Named::F9 => Some(ShortcutKey::Function(9)),
            iced::keyboard::key::Named::F10 => Some(ShortcutKey::Function(10)),
            iced::keyboard::key::Named::F11 => Some(ShortcutKey::Function(11)),
            iced::keyboard::key::Named::F12 => Some(ShortcutKey::Function(12)),
            _ => None,
        },
        _ => None,
    }
}

/// Convert iced modifiers to our ShortcutModifiers
pub fn iced_modifiers_to_shortcut_modifiers(
    modifiers: iced::keyboard::Modifiers,
) -> ShortcutModifiers {
    ShortcutModifiers {
        ctrl: modifiers.control(),
        shift: modifiers.shift(),
        alt: modifiers.alt(),
        meta: modifiers.logo(),
    }
}

/// Check if a key event represents a printable character
pub fn is_printable_key(key: &iced::keyboard::Key) -> bool {
    matches!(key, iced::keyboard::Key::Character(_))
}

/// Get a human-readable description of an iced key
pub fn describe_iced_key(key: &iced::keyboard::Key) -> String {
    match key {
        iced::keyboard::Key::Character(c) => c.to_uppercase(),
        iced::keyboard::Key::Named(named_key) => match named_key {
            iced::keyboard::key::Named::Escape => "Escape".to_string(),
            iced::keyboard::key::Named::Enter => "Enter".to_string(),
            iced::keyboard::key::Named::Space => "Space".to_string(),
            iced::keyboard::key::Named::Tab => "Tab".to_string(),
            iced::keyboard::key::Named::Backspace => "Backspace".to_string(),
            iced::keyboard::key::Named::Delete => "Delete".to_string(),
            iced::keyboard::key::Named::Insert => "Insert".to_string(),
            iced::keyboard::key::Named::Home => "Home".to_string(),
            iced::keyboard::key::Named::End => "End".to_string(),
            iced::keyboard::key::Named::PageUp => "Page Up".to_string(),
            iced::keyboard::key::Named::PageDown => "Page Down".to_string(),
            iced::keyboard::key::Named::ArrowUp => "↑".to_string(),
            iced::keyboard::key::Named::ArrowDown => "↓".to_string(),
            iced::keyboard::key::Named::ArrowLeft => "←".to_string(),
            iced::keyboard::key::Named::ArrowRight => "→".to_string(),
            iced::keyboard::key::Named::F1 => "F1".to_string(),
            iced::keyboard::key::Named::F2 => "F2".to_string(),
            iced::keyboard::key::Named::F3 => "F3".to_string(),
            iced::keyboard::key::Named::F4 => "F4".to_string(),
            iced::keyboard::key::Named::F5 => "F5".to_string(),
            iced::keyboard::key::Named::F6 => "F6".to_string(),
            iced::keyboard::key::Named::F7 => "F7".to_string(),
            iced::keyboard::key::Named::F8 => "F8".to_string(),
            iced::keyboard::key::Named::F9 => "F9".to_string(),
            iced::keyboard::key::Named::F10 => "F10".to_string(),
            iced::keyboard::key::Named::F11 => "F11".to_string(),
            iced::keyboard::key::Named::F12 => "F12".to_string(),
            _ => format!("{:?}", named_key),
        },
        _ => format!("{:?}", key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_key_conversion() {
        let key = iced::keyboard::Key::Character("a".into());
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, Some(ShortcutKey::Character('a')));
    }

    #[test]
    fn test_uppercase_character_conversion() {
        let key = iced::keyboard::Key::Character("A".into());
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, Some(ShortcutKey::Character('a'))); // Should be lowercase
    }

    #[test]
    fn test_named_key_conversion() {
        let key = iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape);
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, Some(ShortcutKey::Escape));
    }

    #[test]
    fn test_function_key_conversion() {
        let key = iced::keyboard::Key::Named(iced::keyboard::key::Named::F1);
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, Some(ShortcutKey::Function(1)));

        let key_f12 = iced::keyboard::Key::Named(iced::keyboard::key::Named::F12);
        let shortcut_key_f12 = iced_key_to_shortcut_key(&key_f12);
        assert_eq!(shortcut_key_f12, Some(ShortcutKey::Function(12)));
    }

    #[test]
    fn test_arrow_key_conversion() {
        let up_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp);
        assert_eq!(
            iced_key_to_shortcut_key(&up_key),
            Some(ShortcutKey::ArrowUp)
        );

        let down_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown);
        assert_eq!(
            iced_key_to_shortcut_key(&down_key),
            Some(ShortcutKey::ArrowDown)
        );

        let left_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft);
        assert_eq!(
            iced_key_to_shortcut_key(&left_key),
            Some(ShortcutKey::ArrowLeft)
        );

        let right_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight);
        assert_eq!(
            iced_key_to_shortcut_key(&right_key),
            Some(ShortcutKey::ArrowRight)
        );
    }

    #[test]
    fn test_modifiers_conversion() {
        let modifiers = iced::keyboard::Modifiers::CTRL | iced::keyboard::Modifiers::SHIFT;
        let shortcut_modifiers = iced_modifiers_to_shortcut_modifiers(modifiers);

        assert!(shortcut_modifiers.ctrl);
        assert!(shortcut_modifiers.shift);
        assert!(!shortcut_modifiers.alt);
        assert!(!shortcut_modifiers.meta);
    }

    #[test]
    fn test_all_modifiers_conversion() {
        let modifiers = iced::keyboard::Modifiers::CTRL
            | iced::keyboard::Modifiers::SHIFT
            | iced::keyboard::Modifiers::ALT
            | iced::keyboard::Modifiers::LOGO;
        let shortcut_modifiers = iced_modifiers_to_shortcut_modifiers(modifiers);

        assert!(shortcut_modifiers.ctrl);
        assert!(shortcut_modifiers.shift);
        assert!(shortcut_modifiers.alt);
        assert!(shortcut_modifiers.meta);
    }

    #[test]
    fn test_empty_modifiers_conversion() {
        let modifiers = iced::keyboard::Modifiers::empty();
        let shortcut_modifiers = iced_modifiers_to_shortcut_modifiers(modifiers);

        assert!(!shortcut_modifiers.ctrl);
        assert!(!shortcut_modifiers.shift);
        assert!(!shortcut_modifiers.alt);
        assert!(!shortcut_modifiers.meta);
    }

    #[test]
    fn test_printable_key_detection() {
        let char_key = iced::keyboard::Key::Character("a".into());
        let escape_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape);

        assert!(is_printable_key(&char_key));
        assert!(!is_printable_key(&escape_key));
    }

    #[test]
    fn test_key_description() {
        let char_key = iced::keyboard::Key::Character("a".into());
        let escape_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape);
        let f1_key = iced::keyboard::Key::Named(iced::keyboard::key::Named::F1);

        assert_eq!(describe_iced_key(&char_key), "A");
        assert_eq!(describe_iced_key(&escape_key), "Escape");
        assert_eq!(describe_iced_key(&f1_key), "F1");
    }

    #[test]
    fn test_unsupported_key_conversion() {
        // Test that unsupported keys return None
        let key = iced::keyboard::Key::Named(iced::keyboard::key::Named::CapsLock);
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, None);
    }

    #[test]
    fn test_empty_character_key() {
        let key = iced::keyboard::Key::Character("".into());
        let shortcut_key = iced_key_to_shortcut_key(&key);
        assert_eq!(shortcut_key, None);
    }
}
