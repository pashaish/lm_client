pub fn take_component<Component, OriginMessage, TargetMessage>(
    tasks: &mut Vec<iced::Task<TargetMessage>>,
    converter: impl FnMut(OriginMessage) -> TargetMessage + Send + 'static,
    pair: (Component, iced::Task<OriginMessage>),
) -> Component
where
    OriginMessage: Send + 'static,
    TargetMessage: Send + 'static,
{
    tasks.push(pair.1.map(converter));
    pair.0
}

#[must_use] pub fn diff_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();

    loop {
        match (a_chars.next(), b_chars.next()) {
            (Some(ac), Some(bc)) => {
                if ac != bc {
                    result.push(bc);
                }
            }
            (None, Some(bc)) => result.push(bc),
            (Some(_) | None, None) => break,
            }
    }

    result
}