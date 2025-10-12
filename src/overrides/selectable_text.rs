// use framework::utils::selection::{self, selection};
// use iced::advanced::renderer::Quad;
// use iced::advanced::text::highlighter::Format;
// use iced::advanced::text::{Paragraph, paragraph};
// use iced::advanced::widget::{Operation, Tree, operation, tree};
// use iced::advanced::{Layout, Widget, layout, mouse, renderer, text, widget};
// use iced::widget::text::{Fragment, IntoFragment, Wrapping};
// use iced::widget::text_input::Value;
// use iced::{
//     Alignment, Border, Color, Element, Length, Pixels, Point, Rectangle, Shadow, Size, Task,
//     alignment, touch,
// };

// pub use self::text::{LineHeight, Shaping};

// pub fn selectable_text<'a, Theme, Renderer>(
//     fragment: impl IntoFragment<'a>,
// ) -> Text<'a, Theme, Renderer>
// where
//     Renderer: text::Renderer,
//     Theme: Catalog,
// {
//     Text::new(fragment)
// }

// pub struct Text<'a, Theme, Renderer>
// where
//     Renderer: text::Renderer,
//     Theme: Catalog,
// {
//     fragment: Fragment<'a>,
//     format: Format<Renderer::Font>,
//     class: Theme::Class<'a>,
// }

// impl<'a, Theme, Renderer> Text<'a, Theme, Renderer>
// where
//     Renderer: text::Renderer,
//     Theme: Catalog,
// {
//     pub fn new(fragment: impl IntoFragment<'a>) -> Self {
//         Text {
//             fragment: fragment.into_fragment(),
//             format: Format {
//                 // TODO:
//                 // #[cfg(debug_assertions)]
//                 // shaping: Shaping::Basic,
//                 // #[cfg(not(debug_assertions))]
//                 // shaping: Shaping::Advanced,
//                 // wrapping: Wrapping::WordOrGlyph,
//                 ..Format::default()
//             },
//             class: Theme::default(),
//         }
//     }

//     pub fn size(mut self, size: impl Into<Pixels>) -> Self {
//         self.format.size = Some(size.into());
//         self
//     }

//     pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
//         self.format.line_height = line_height.into();
//         self
//     }

//     pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
//         self.format.font = Some(font.into());
//         self
//     }

//     pub fn font_maybe(mut self, font: Option<impl Into<Renderer::Font>>) -> Self {
//         self.format.font = font.map(Into::into);
//         self
//     }

//     pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
//     where
//         Theme::Class<'a>: From<StyleFn<'a, Theme>>,
//     {
//         self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
//         self
//     }

//     pub fn width(mut self, width: impl Into<Length>) -> Self {
//         self.format.width = width.into();
//         self
//     }

//     pub fn height(mut self, height: impl Into<Length>) -> Self {
//         self.format.height = height.into();
//         self
//     }

//     pub fn align_x(mut self, alignment: Alignment) -> Self {
//         self.format.align_x = alignment;
//         self
//     }

//     pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
//         self.format.align_y = alignment;
//         self
//     }

//     pub fn shaping(mut self, shaping: Shaping) -> Self {
//         self.format.shaping = shaping;
//         self
//     }

//     pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
//         self.class = class.into();
//         self
//     }
// }

// impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Text<'_, Theme, Renderer>
// where
//     Renderer: text::Renderer,
//     Theme: Catalog,
// {
//     fn size(&self) -> Size<Length> {
//         Size {
//             width: self.format.width,
//             height: self.format.height,
//         }
//     }

//     fn tag(&self) -> tree::Tag {
//         tree::Tag::of::<State<Renderer::Paragraph>>()
//     }

//     fn state(&self) -> tree::State {
//         tree::State::new(State::<Renderer::Paragraph>::default())
//     }

//     fn layout(
//         &self,
//         tree: &mut Tree,
//         renderer: &Renderer,
//         limits: &layout::Limits,
//     ) -> layout::Node {
//         let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

//         layout::sized(limits, self.format.width, self.format.height, |limits| {
//             let bounds = limits.max();

//             let size = self.format.size.unwrap_or_else(|| renderer.default_size());
//             let font = self.format.font.unwrap_or_else(|| renderer.default_font());

//             state.paragraph.update(text::Text {
//                 content: &self.fragment,
//                 bounds,
//                 size,
//                 line_height: self.format.line_height,
//                 font,
//                 shaping: self.format.shaping,
//                 wrapping: self.format.wrapping,
//                 horizontal_alignment: self.format.align_x,
//                 vertical_alignment: self.format.align_y,
//             });

//             state.paragraph.min_bounds()
//         })
//     }

//     fn on_event(
//         &mut self,
//         tree: &mut Tree,
//         event: iced::Event,
//         layout: Layout<'_>,
//         cursor: mouse::Cursor,
//         _renderer: &Renderer,
//         _clipboard: &mut dyn iced::advanced::Clipboard,
//         shell: &mut iced::advanced::Shell<'_, Message>,
//         _viewport: &Rectangle,
//     ) -> iced::advanced::graphics::core::event::Status {
//         let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

//         let bounds = layout.bounds();

//         let prev_hovered = state.hovered;
//         let prev_interaction = state.interaction;
//         state.hovered = cursor.is_over(bounds);

//         match event {
//             iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
//             | iced::Event::Touch(touch::Event::FingerPressed { .. }) => {
//                 if let Some(cursor) = cursor.position() {
//                     state.interaction = Interaction::Selecting(selection::Raw {
//                         start: cursor,
//                         end: cursor,
//                     });
//                 } else {
//                     state.interaction = Interaction::Idle;
//                 }
//             }
//             iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
//             | iced::Event::Touch(touch::Event::FingerLifted { .. })
//             | iced::Event::Touch(touch::Event::FingerLost { .. }) => {
//                 if let Interaction::Selecting(raw) = state.interaction {
//                     state.interaction = Interaction::Selected(raw);
//                 } else {
//                     state.interaction = Interaction::Idle;
//                 }
//             }
//             iced::Event::Mouse(mouse::Event::CursorMoved { .. })
//             | iced::Event::Touch(touch::Event::FingerMoved { .. }) => {
//                 if let Some(cursor) = cursor.position()
//                     && let Interaction::Selecting(raw) = &mut state.interaction
//                 {
//                     raw.end = cursor;
//                 }
//             }
//             _ => {}
//         }

//         if prev_hovered != state.hovered || prev_interaction != state.interaction {
//             shell.request_redraw(iced::window::RedrawRequest::NextFrame);
//         }
//     }

//     fn draw(
//         &self,
//         tree: &Tree,
//         renderer: &mut Renderer,
//         theme: &Theme,
//         style: &renderer::Style,
//         layout: Layout<'_>,
//         _cursor_position: mouse::Cursor,
//         viewport: &Rectangle,
//     ) {
//         let bounds = layout.bounds();

//         if viewport.intersection(&bounds).is_none() {
//             return;
//         }

//         let appearance = theme.style(&self.class);

//         let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();

//         if let Some(selection) = state
//             .interaction
//             .selection()
//             .and_then(|raw| raw.resolve(bounds))
//         {
//             let line_height = f32::from(
//                 self.format
//                     .line_height
//                     .to_absolute(self.format.size.unwrap_or_else(|| renderer.default_size())),
//             );

//             let baseline_y =
//                 bounds.y + ((selection.start.y - bounds.y) / line_height).floor() * line_height;

//             let height = selection.end.y - baseline_y - 0.5;
//             let rows = (height / line_height).ceil() as usize;

//             for row in 0..rows {
//                 let (x, width) = if row == 0 {
//                     (
//                         selection.start.x,
//                         if rows == 1 {
//                             f32::min(selection.end.x, bounds.x + bounds.width) - selection.start.x
//                         } else {
//                             bounds.x + bounds.width - selection.start.x
//                         },
//                     )
//                 } else if row == rows - 1 {
//                     (bounds.x, selection.end.x - bounds.x)
//                 } else {
//                     (bounds.x, bounds.width)
//                 };
//                 let y = baseline_y + row as f32 * line_height;

//                 renderer.fill_quad(
//                     Quad {
//                         bounds: Rectangle::new(Point::new(x, y), Size::new(width, line_height)),
//                         border: Border {
//                             radius: 0.0.into(),
//                             width: 0.0,
//                             color: Color::TRANSPARENT,
//                         },
//                         shadow: Shadow::default(),
//                         // snap: true,
//                     },
//                     appearance.selection_color,
//                 );
//             }
//         }

//         // TODO: This method is better for ensuring whole letters are visually selected,
//         // but breaks down once wrapping comes to play.
//         // if let Some(Selection { start, end }) = state.selection().and_then(|raw| {
//         //     selection(
//         //         raw,
//         //         renderer,
//         //         self.font,
//         //         self.size,
//         //         self.line_height,
//         //         layout.bounds(),
//         //         &value,
//         //     )
//         // }) {
//         //     let pre_value = (start > 0).then(|| value.select(0, start));
//         //     let value = value.select(start, end);

//         //     let pre_width = pre_value
//         //         .as_ref()
//         //         .map(|value| measure(renderer, value, self.size, self.font));
//         //     let selected_width = measure(renderer, &value, self.size, self.font);

//         //     let line_height = f32::from(
//         //         self.line_height
//         //             .to_absolute(self.size.unwrap_or_else(|| renderer.default_size()).into()),
//         //     );

//         //     let bounds = layout.bounds();

//         //     let mut position = bounds.position();
//         //     let mut remaining = pre_width.unwrap_or_default();

//         //     while remaining > 0.0 {
//         //         let max_width = bounds.width - (position.x - bounds.x);
//         //         let width = remaining.min(max_width);

//         //         position = if width == max_width {
//         //             Point::new(bounds.x, position.y + line_height)
//         //         } else {
//         //             Point::new(position.x + width, position.y)
//         //         };
//         //         remaining -= width;
//         //     }

//         //     let mut remaining = selected_width;

//         //     while remaining > 0.0 {
//         //         let max_width = bounds.width - (position.x - bounds.x);
//         //         let width = remaining.min(max_width);

//         //         renderer.fill_quad(
//         //             Quad {
//         //                 bounds: Rectangle::new(position, Size::new(width, line_height)),
//         //                 border_radius: 0.0.into(),
//         //                 border_width: 0.0,
//         //                 border_color: Color::TRANSPARENT,
//         //             },
//         //             theme.selection_color(&self.style),
//         //         );

//         //         position = if width == max_width {
//         //             Point::new(bounds.x, position.y + line_height)
//         //         } else {
//         //             Point::new(position.x + width, position.y)
//         //         };
//         //         remaining -= width;
//         //     }
//         // }

//         draw(renderer, style, layout, state, appearance, viewport);
//     }

//     fn mouse_interaction(
//         &self,
//         tree: &Tree,
//         _layout: Layout<'_>,
//         _cursor: mouse::Cursor,
//         _viewport: &Rectangle,
//         _renderer: &Renderer,
//     ) -> mouse::Interaction {
//         let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();

//         if state.hovered {
//             mouse::Interaction::Text
//         } else {
//             mouse::Interaction::default()
//         }
//     }

//     fn operate(
//         &self,
//         tree: &mut Tree,
//         layout: Layout<'_>,
//         _renderer: &Renderer,
//         operation: &mut dyn Operation<()>,
//     ) {
//         let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();

//         let bounds = layout.bounds();
//         let value = Value::new(&self.fragment);
//         if let Some(selection) = state
//             .interaction
//             .selection()
//             .and_then(|raw| selection(raw, bounds, state.paragraph.raw(), &value))
//         {
//             let mut content = value.select(selection.start, selection.end).to_string();
//             operation.custom(None, bounds, &mut content);
//         }
//     }
// }

// fn draw<Renderer>(
//     renderer: &mut Renderer,
//     style: &renderer::Style,
//     layout: Layout<'_>,
//     state: &State<Renderer::Paragraph>,
//     appearance: Style,
//     viewport: &Rectangle,
// ) where
//     Renderer: text::Renderer,
// {
//     let State { paragraph, .. } = &state;
//     let anchor = layout.bounds().anchor(
//         paragraph.min_bounds(),
//         paragraph.horizontal_alignment(),
//         paragraph.vertical_alignment(),
//     );

//     renderer.fill_paragraph(
//         paragraph.raw(),
//         anchor,
//         appearance.color.unwrap_or(style.text_color),
//         *viewport,
//     );
// }

// impl<'a, Message, Theme, Renderer> From<Text<'a, Theme, Renderer>>
//     for Element<'a, Message, Theme, Renderer>
// where
//     Renderer: text::Renderer + 'a,
//     Theme: Catalog + 'a,
// {
//     fn from(text: Text<'a, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
//         Element::new(text)
//     }
// }

// #[derive(Debug, Default)]
// pub struct State<P: Paragraph> {
//     paragraph: paragraph::Plain<P>,
//     interaction: Interaction,
//     hovered: bool,
// }

// #[derive(Debug, Clone, Copy, Default, PartialEq)]
// pub enum Interaction {
//     #[default]
//     Idle,
//     Selecting(selection::Raw),
//     Selected(selection::Raw),
// }

// impl Interaction {
//     pub fn selection(self) -> Option<selection::Raw> {
//         match &self {
//             Interaction::Idle => None,
//             Interaction::Selecting(raw) | Interaction::Selected(raw) => Some(*raw),
//         }
//     }
// }

// // fn measure<Renderer>(
// //     renderer: &Renderer,
// //     value: &Value,
// //     size: Option<f32>,
// //     font: Option<Renderer::Font>,
// // ) -> f32
// // where
// //     Renderer: text::Renderer,
// // {
// //     let size = size.unwrap_or_else(|| renderer.default_size());
// //     let font = font.unwrap_or_else(|| renderer.default_font());

// //     renderer.measure_width(&value.to_string(), size, font, text::Shaping::Advanced)
// // }

// pub fn selected<Message: Send + 'static>(f: fn(Vec<(f32, String)>) -> Message) -> Task<Message> {
//     struct Selected<T> {
//         contents: Vec<(f32, String)>,
//         f: fn(Vec<(f32, String)>) -> T,
//         bounds: Rectangle,
//     }

//     impl<T> Operation<T> for Selected<T> {
//         fn container(
//             &mut self,
//             _id: Option<&widget::Id>,
//             bounds: Rectangle,
//             operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
//         ) {
//             operate_on_children(self);
//             self.bounds = bounds;
//         }

//         fn custom(
//             &mut self,
//             state: &mut dyn std::any::Any,
//             id: Option<&widget::Id>
//         ) {
//             if let Some(content) = state.downcast_ref::<String>() {
//                 self.contents.push((self.bounds.y, content.clone()));
//             }
//         }

//         fn finish(&self) -> operation::Outcome<T> {
//             operation::Outcome::Some((self.f)(self.contents.clone()))
//         }
//     }

//     widget::operate(Selected {
//         contents: vec![],
//         f,
//         bounds: Rectangle::default(),
//     })
// }

// /// The appearance of some text.
// #[derive(Debug, Clone, Copy)]
// pub struct Style {
//     pub color: Option<Color>,
//     pub selection_color: Color,
// }

// impl Default for Style {
//     fn default() -> Self {
//         Self {
//             color: None,
//             selection_color: Color::WHITE,
//         }
//     }
// }

// /// The theme catalog of a [`Text`].
// pub trait Catalog: Sized {
//     /// The item class of this [`Catalog`].
//     type Class<'a>;

//     /// The default class produced by this [`Catalog`].
//     fn default<'a>() -> Self::Class<'a>;

//     /// The [`Style`] of a class with the given status.
//     fn style(&self, item: &Self::Class<'_>) -> Style;
// }

// /// A styling function for a [`Text`].
// ///
// /// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
// pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

// impl Catalog for iced::Theme {
//     type Class<'a> = StyleFn<'a, Self>;

//     fn default<'a>() -> Self::Class<'a> {
//         Box::new(|_theme| Style::default())
//     }

//     fn style(&self, class: &Self::Class<'_>) -> Style {
//         class(self)
//     }
// }
