#[allow(unused)]
pub fn maybe_get_a_value(give_something: bool) -> Option<u8> {
    if give_something {
        return Some(100);
    }
    None // => Nie wieder null!!!
}

// Beispiel enum. Enums können Values enthalten.
#[allow(unused)]
enum MyEnum {
    Green,

    Red(u8),

    Yellow {
        name: String,
        age: u16,
    }
}

// Option vereinfacht dargestellt.
#[allow(unused)]
enum MyOption<T> {
    None,
    Some(T),
}


#[allow(unused)]
pub fn check_value(value: i64) -> Result<(), MyErrors> {
    match value {
        ..0 => Ok(()),

        0..=100 => Err(MyErrors::Voltage(5.6321)),

        _ => Err(MyErrors::Temperature {
            temperature: 666.666,
            too_high: true,
        }),
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum MyErrors {
    // Note: No need for a NoError entry.

    // Member with simple unnamed value
    Voltage(f32),

    // Member with named value
    Temperature {
        temperature: f32,
        too_high: bool,
    },

    Undefined,
}
