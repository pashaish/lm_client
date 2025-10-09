                // TODO: NEED UPDATE

// use iced::{
//     Element, Point, Size, Vector,
//     advanced::{
//         Overlay,
//         layout::{self, Limits, Node},
//         widget::Tree,
//     },
// };

// pub struct DraggingPlaceholderOverlay<'a, Message, Theme, Renderer> {
//     position: Point,
//     tree: &'a mut Tree,
//     content: Element<'a, Message, Theme, Renderer>,
//     max_size: Size,
// }

// impl<'a, Message, Theme, Renderer> DraggingPlaceholderOverlay<'a, Message, Theme, Renderer>
// where
//     Message: Clone,
//     Renderer: iced::advanced::renderer::Renderer,
//     Theme: 'a,
// {
//     pub fn new<'b, C>(position: Point, tree: &'b mut Tree, content: C, max_size: Size) -> Self
//     where
//         C: Into<Element<'a, Message, Theme, Renderer>>,
//         'b: 'a,
//     {
//         DraggingPlaceholderOverlay {
//             position,
//             tree,
//             content: content.into(),
//             max_size,
//         }
//     }
// }

// impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
//     for DraggingPlaceholderOverlay<'a, Message, Theme, Renderer>
// where
//     Message: Clone,
//     Renderer: iced::advanced::renderer::Renderer,
//     Theme: 'a,
// {
//     fn layout(&mut self, renderer: &Renderer, bounds: iced::Size) -> iced::advanced::layout::Node {
//         let limits = iced::advanced::layout::Limits::new(
//             Size::new(0.0, 0.0),
//             Size::new(self.max_size.width, self.max_size.height),
//         );
//         let origin_limits = Limits::new(Size::ZERO, bounds);
//         let max_size = origin_limits.max();

//         let mut layout = self
//             .content
//             .as_widget()
//             .layout(self.tree, renderer, &limits);

//         let half_width = layout.size().width / 2.0;
//         let half_height = layout.size().height / 2.0;
//         layout.move_to_mut(self.position - Vector::new(half_width, half_height));

//         Node::with_children(max_size, vec![layout])
//     }

//     fn on_event(
//         &mut self,
//         _event: iced::Event,
//         _layout: iced::advanced::Layout<'_>,
//         _cursor: iced::advanced::mouse::Cursor,
//         _renderer: &Renderer,
//         _clipboard: &mut dyn iced::advanced::Clipboard,
//         _shell: &mut iced::advanced::Shell<'_, Message>,
//     ) -> iced::advanced::graphics::core::event::Status {
//         iced::advanced::graphics::core::event::Status::Ignored
//     }

//     fn is_over(
//         &self,
//         _layout: layout::Layout<'_>,
//         _renderer: &Renderer,
//         _cursor_position: Point,
//     ) -> bool {
//         false
//     }

//     fn draw(
//         &self,
//         renderer: &mut Renderer,
//         theme: &Theme,
//         style: &iced::advanced::renderer::Style,
//         layout: iced::advanced::Layout<'_>,
//         cursor: iced::advanced::mouse::Cursor,
//     ) {
//         let bounds = layout.bounds();

//         let content_layout = layout
//             .children()
//             .next()
//             .expect("widget: Layout should have a content layout.");

//         self.content.as_widget().draw(
//             self.tree,
//             renderer,
//             theme,
//             style,
//             content_layout,
//             cursor,
//             &bounds,
//         );
//     }
// }
