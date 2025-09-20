use iced::{
    Point, Rectangle, Size,
    advanced::{
        Widget,
        graphics::core::Element,
        overlay::{self},
        renderer,
        widget::Tree,
    },
};

use super::dragging_placeholder_overlay::DraggingPlaceholderOverlay;

/* #region WIDGET */

pub struct DraggingPlaceholder<'a, Message, Content, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::renderer::Renderer,
    Content: Fn() -> iced::Element<'a, Message, Theme, Renderer>,
{
    pub(super) content: Content,
    #[allow(dead_code)]
    pub(super) is_visible: bool,

    pub(super) position: Point,
    pub(super) max_size: Size,
}

impl<'a, Message, Content, Theme, Renderer>
    DraggingPlaceholder<'a, Message, Content, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::renderer::Renderer,
    Content: Fn() -> iced::Element<'a, Message, Theme, Renderer>,
{
    pub const fn new(content: Content, is_visible: bool, max_size: Size) -> Self {
        Self {
            max_size,
            content,
            is_visible,
            position: Point::ORIGIN,
        }
    }
}

impl<'a, Message, Content, Theme, Renderer> Widget<Message, Theme, Renderer>
    for DraggingPlaceholder<'a, Message, Content, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced::advanced::renderer::Renderer + 'a,
    Theme: 'a,
    Content: Fn() -> iced::Element<'a, Message, Theme, Renderer> + 'a,
{
    fn on_event(
        &mut self,
        _state: &mut iced::advanced::widget::Tree,
        _event: iced::Event,
        _layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        self.position = cursor.position().unwrap_or_default();

        iced::advanced::graphics::core::event::Status::Ignored
    }

    fn size(&self) -> iced::Size<iced::Length> {
        (self.content)().as_widget().size()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        (self.content)().as_widget().state()
    }

    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        iced::advanced::layout::Node::new(iced::Size::new(0.0, 0.0))
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        _layout: iced::advanced::Layout<'_>,
        _renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if self.position == Point::ORIGIN {
            return None;
        }

        let elem = (self.content)();

        elem.as_widget().diff(&mut state.children[0]);

        let ov = DraggingPlaceholderOverlay::new(
            self.position + translation,
            &mut state.children[0],
            elem,
            self.max_size,
        );

        Some(iced::advanced::overlay::Element::new(Box::new(ov)))
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new((self.content)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&(self.content)()]);
    }
}
impl<'a, Message, Content, Theme, Renderer>
    From<DraggingPlaceholder<'a, Message, Content, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a,
    Content: 'a + Fn() -> Self,
{
    fn from(modal: DraggingPlaceholder<'a, Message, Content, Theme, Renderer>) -> Self {
        Element::new(modal)
    }
}

/* #endregion */
