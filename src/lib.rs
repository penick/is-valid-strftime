use wasm_bindgen::prelude::*;
use chrono::prelude::*;

#[wasm_bindgen]
pub fn is_valid_strftime(format: &str) -> bool {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let date_time = Utc::now();
    let tz = date_time.timezone();
    let formatted = format!("{}", date_time.format(format));
    tz.datetime_from_str(&formatted, format).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(is_valid_strftime("%a"), false);
        assert_eq!(is_valid_strftime("%Y-%m-%d %H:%M:%S"), true);
    }
}
