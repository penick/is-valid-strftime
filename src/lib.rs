use wasm_bindgen::prelude::*;
use chrono::prelude::*;

#[wasm_bindgen]
pub fn is_valid_strftime(format: &str) -> bool {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let date_time = Utc::now();
    let tz = date_time.timezone();

    let formatted = std::panic::catch_unwind(|| {
        format!("{}", date_time.format(format))
    });

   formatted.map_or(false, |formatted| { tz.datetime_from_str(&formatted, format).is_ok() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(is_valid_strftime("%z"), false);
        assert_eq!(is_valid_strftime("%a"), false);
        assert_eq!(is_valid_strftime("%Y-%m-%d %H:%M:%S"), true);
    }
}
