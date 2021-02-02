use crate::MethodKind;

/// Change a CamelCase method name to snake case and prepends an optional
/// preamble depending on the kind of method.
pub fn method_to_snake(camel: &str, kind: MethodKind) -> String {
    debug_assert!(!camel.is_empty());
    let mut snake = String::with_capacity(camel.len());

    match kind {
        MethodKind::Set => snake.push_str("set_"),
        MethodKind::Remove => snake.push_str("remove_"),
        _ => {}
    };

    append(snake, camel, |c, buffer| {
        let len = buffer.len();
        buffer.extend(c.to_lowercase());
        buffer.len() - len
    })
}

/// Change a name to snake case.
pub fn to_snake(name: &str) -> String {
    debug_assert!(!name.is_empty());
    let snake = String::with_capacity(name.len());

    append(snake, name, |c, buffer| {
        let len = buffer.len();
        buffer.extend(c.to_lowercase());
        len
    })
}

/// Change a name to upper case.
pub fn to_upper(name: &str) -> String {
    debug_assert!(!name.is_empty());
    let upper = String::with_capacity(name.len());

    append(upper, name, |c, buffer| {
        let len = buffer.len();
        buffer.extend(c.to_uppercase());
        len
    })
}

fn append<ToCase: Fn(char, &mut String) -> usize>(
    mut result: String,
    camel: &str,
    to_case: ToCase,
) -> String {
    // Add any manual fixups here, anything that isn't handled automatically by the algorithm below.
    if camel == "WinRT" {
        to_case('w', &mut result);
        to_case('i', &mut result);
        to_case('n', &mut result);
        to_case('r', &mut result);
        to_case('t', &mut result);
        return result;
    }

    let mut since_last_underscore = 0;
    let mut chars = camel.chars();
    // first character as lowercased
    since_last_underscore += to_case(chars.next().unwrap(), &mut result);

    // zip together iterator of previous characters and next characters
    for (previous, next) in camel.chars().zip(camel.chars().skip(2)) {
        // safe to unwrap since the iterator of next chars produced something
        let current = chars.next().unwrap();

        // If the current character isn't uppercase we can just push it and move on
        if !current.is_uppercase() {
            since_last_underscore += 1;
            result.push(current);
            continue;
        }

        if previous.is_lowercase() || next.is_lowercase() && since_last_underscore > 1 {
            since_last_underscore = 0;

            if previous != '_' {
                result.push('_');
            }
        }

        since_last_underscore += to_case(current, &mut result);
    }

    if let Some(last) = chars.next() {
        to_case(last, &mut result);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn to_snake_works() {
        assert_eq!(to_snake("Windows"), "windows".to_owned());

        assert_eq!(to_snake("ApplicationModel"), "application_model".to_owned());

        assert_eq!(method_to_snake("foo", MethodKind::Normal), "foo".to_owned());

        assert_eq!(
            method_to_snake("UIProgramming", MethodKind::Normal),
            "ui_programming".to_owned()
        );

        assert_eq!(
            method_to_snake("UIProgramming", MethodKind::Set),
            "set_ui_programming".to_owned()
        );

        assert_eq!(
            method_to_snake("CreateUInt8Array", MethodKind::Normal),
            "create_uint8_array".to_owned()
        );

        assert_eq!(
            method_to_snake("Socks", MethodKind::Remove),
            "remove_socks".to_owned()
        );

        assert_eq!(to_snake("appointmentId"), "appointment_id".to_owned());

        assert_eq!(method_to_snake("a", MethodKind::Normal), "a".to_owned());

        assert_eq!(
            method_to_snake("CreateField_Default", MethodKind::Normal),
            "create_field_default".to_owned()
        );

        assert!(to_snake("WinRT") == "winrt");
    }

    #[test]
    fn to_upper_works() {
        assert_eq!(to_upper("Windows"), "WINDOWS".to_owned());

        assert_eq!(to_upper("ApplicationModel"), "APPLICATION_MODEL".to_owned());

        assert_eq!(to_upper("foo"), "FOO".to_owned());

        assert_eq!(to_upper("UIProgramming"), "UI_PROGRAMMING".to_owned());

        assert_eq!(
            to_upper("CreateUInt8Array"),
            "CREATE_UINT8_ARRAY".to_owned()
        );

        assert_eq!(to_upper("appointmentId"), "APPOINTMENT_ID".to_owned());

        assert_eq!(to_upper("a"), "A".to_owned());
        assert_eq!(to_upper("A"), "A".to_owned());

        assert_eq!(
            to_upper("CreateField_Default"),
            "CREATE_FIELD_DEFAULT".to_owned()
        );

        assert!(to_upper("WinRT") == "WINRT");
    }
}
