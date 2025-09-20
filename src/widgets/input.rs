use iced::{
    widget::{text_input, Column, Container, Text},
    Element, Length, Theme,
};

pub struct Input<'a, Message>
where
    Message: Clone + 'a,
{
    id: Option<String>,
    value: String,
    placeholder: String,
    label: Option<String>,
    width: Length,
    on_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    error: Option<String>,
    disabled: bool,
    secure: bool,
}

impl<'a, Message> Input<'a, Message>
where
    Message: Clone + 'a,
{
    /// Create a new input field
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
            placeholder: String::new(),
            label: None,
            width: Length::Fill,
            on_change: None,
            on_submit: None,
            error: None,
            disabled: false,
            secure: false,
        }
    }

    /// Set the input id
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the label for the input
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the input width
    pub const fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Set the `on_change` callback
    pub fn on_change<F>(mut self, on_change: F) -> Self 
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Set the action to perform when submitting
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Set an error message
    pub fn error(mut self, error: Option<impl Into<String>>) -> Self {
        self.error = error.map(std::convert::Into::into);
        self
    }

    pub const fn secure(mut self) -> Self {
      self.secure = true;

      self
    }

    /// Set whether the input is disabled
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let mut column = Column::new().spacing(5);

        if let Some(label) = self.label {
            column = column.push(
                Text::new(label)
                    .size(14)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();
                        iced::widget::text::Style {
                            color: Some(palette.secondary.base.text),
                        }
                    }),
            );
        }

        let mut input = text_input(&self.placeholder, &self.value)
            .padding(8)
            .width(self.width);

        // Apply on_change only if provided and input is not disabled
        if !self.disabled {
            if let Some(on_change) = self.on_change {
                input = input.on_input(on_change);
            }
            
            if let Some(msg) = self.on_submit {
                input = input.on_submit(msg);
            }
        }

        if let Some(id) = self.id {
            input = input.id(id);
        }

        let error = self.error.clone();
        let disabled = self.disabled;
        
        input = input.style(move |theme: &Theme, status| {
            let mut style = text_input::default(theme, status);

            if error.is_some() {
                let palette = theme.extended_palette();
                style.border.color = palette.danger.strong.color;
            }
            
            if disabled {
                let palette = theme.extended_palette();
                style.background = palette.background.weak.color.into();
                style.border.color = palette.background.strong.color;
            }

            style
        });

        if self.secure {
            input = input.secure(true);
        }

        column = column.push(input);

        if let Some(error) = self.error {
            column = column.push(
                Text::new(error)
                    .size(12)
                    .style(|theme: &Theme| {
                        iced::widget::text::Style {
                            color: Some(theme.extended_palette().danger.base.color),
                        }
                    }),
            );
        }

        Container::new(column).into()
    }
}

impl<'a, Message> From<Input<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(input: Input<'a, Message>) -> Self {
        input.view()
    }
}
