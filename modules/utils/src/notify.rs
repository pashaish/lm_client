use std::path::PathBuf;

/// # Panics
pub fn validation(validation_message: impl Into<String>) {
    let validation_message = validation_message.into();

    native_dialog::DialogBuilder::message()
        .set_title("Validation")
        .set_text(validation_message)
        .set_level(native_dialog::MessageLevel::Warning)
        .alert()
        .show()
        .expect("Failed to show validation dialog");
}

/// # Panics
pub async fn confirmation(confirmation_message: impl Into<String>) -> bool {
    let confirmation_message = confirmation_message.into();

    native_dialog::DialogBuilder::message()
        .set_title("Confirmation")
        .set_text(confirmation_message)
        .confirm()
        .spawn()
        .await
        .expect("Failed to show confirmation dialog")
}

/// # Panics
pub fn file_selection(title: impl Into<String>) -> Vec<PathBuf> {
    native_dialog::DialogBuilder::file()
        .set_title(title.into())
        .open_multiple_file()
        .show()
        .expect("Failed to show file selection dialog")
}
