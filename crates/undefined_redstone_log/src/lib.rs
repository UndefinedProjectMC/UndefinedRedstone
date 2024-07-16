use rust_i18n::i18n;
i18n!("locales");

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        undefined_redstone_log::_rust_i18n_translate(&rust_i18n::locale(), $key)
    };

    // t!("foo", locale = "en")
    ($key:expr, locale = $locale:expr) => {
        undefined_redstone_log::_rust_i18n_translate($locale, $key)
    };

    // t!("foo", locale = "en", a = 1, b = "Foo")
    ($key:expr, locale = $locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            let message = undefined_redstone_log::_rust_i18n_translate($locale, $key);
            let patterns: &[&str] = &[
                $(rust_i18n::key!($var_name)),+
            ];
            let values = &[
                $(format!("{}", $var_val)),+
            ];

            let output = rust_i18n::replace_patterns(message.as_ref(), patterns, values);
            std::borrow::Cow::from(output)
        }
    };

    // t!("foo %{a} %{b}", a = "bar", b = "baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };
}

pub fn set_locale(locale: &str) {
    rust_i18n::set_locale(locale);
}