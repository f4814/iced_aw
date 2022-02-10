//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use std::hash::Hash;

use iced_native::{
    alignment::{Horizontal, Vertical},
    event, mouse, renderer, touch, Alignment, Clipboard, Color, Element, Event, Layout, Length,
    Padding, Point, Rectangle, Shell, Size, Widget,
};

use crate::graphics::icons::Icon;
pub use crate::style::card::{Style, StyleSheet};

/// The default padding of a [`Card`](Card).
const DEFAULT_PADDING: f32 = 10.0;

/// A card consisting of a head, body and optional foot.
///
/// # Example
/// ```
/// # use iced_native::{renderer::Null, Text};
/// #
/// # pub type Card<'a, Message> = iced_aw::native::Card<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     ClosingCard,
/// }
///
/// let card = Card::new(
///     Text::new("Head"),
///     Text::new("Body")
/// )
/// .foot(Text::new("Foot"))
/// .on_close(Message::ClosingCard);
///
/// ```
#[allow(missing_debug_implementations)]
pub struct Card<'a, Message, Renderer> {
    /// The width of the [`Card`](Card).
    width: Length,
    /// The height of the [`Card`](Card).
    height: Length,
    /// The maximum width of the [`Card`](Card).
    max_width: u32,
    /// The maximum height of the [`Card`](Card).
    max_height: u32,
    /// The padding of teh head fo the [`Card`](Card).
    padding_head: f32,
    /// The padding of the body of the [`Card`](Card).
    padding_body: f32,
    /// The padding of the foot of the [`Card`](Card).
    padding_foot: f32,
    /// The optional size of the close icon of the [`Card`](Card).
    close_size: Option<f32>,
    /// The optional message that is send if the close icon of the [`Card`](Card) is pressed.
    on_close: Option<Message>,
    /// The head [`Element`](iced_native::Element) of the [`Card`](Card).
    head: Element<'a, Message, Renderer>,
    /// The body [`Element`](iced_native::Element) of the [`Card`](Card).
    body: Element<'a, Message, Renderer>,
    /// The optional foot [`Element`](iced_native::Element) of the [`Card`](Card).
    foot: Option<Element<'a, Message, Renderer>>,
    /// The style of the [`Card`](Card).
    style_sheet: Box<dyn StyleSheet + 'a>,
}

impl<'a, Message, Renderer> Card<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    /// Creates a new [`Card`](Card) containing the given head and body.
    ///
    /// It expects:
    ///     * the head [`Element`](iced_native::Element) to display at the top of
    ///         the [`Card`](Card).
    ///     * the body [`Element`](iced_native::Element) to display at the middle
    ///         of the [`Card`](Card).
    pub fn new<H, B>(head: H, body: B) -> Self
    where
        H: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        Card {
            width: Length::Fill,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding_head: DEFAULT_PADDING,
            padding_body: DEFAULT_PADDING,
            padding_foot: DEFAULT_PADDING,
            close_size: None,
            on_close: None,
            head: head.into(),
            body: body.into(),
            foot: None,
            style_sheet: std::boxed::Box::default(),
        }
    }

    /// Sets the [`Element`](iced_native::Element) of the foot of the
    /// [`Card`](Card).
    pub fn foot<F>(mut self, foot: F) -> Self
    where
        F: Into<Element<'a, Message, Renderer>>,
    {
        self.foot = Some(foot.into());
        self
    }

    /// Sets the width of the [`Card`](Card).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Card`](Card).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Card`](Card).
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the maximum height of the [`Card`](Card).
    pub fn max_height(mut self, height: u32) -> Self {
        self.max_height = height;
        self
    }

    /// Sets the padding of the [`Card`](Card).
    ///
    /// This will set the padding of the head, body and foot to the
    /// same value.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self.padding_body = padding;
        self.padding_foot = padding;
        self
    }

    /// Sets the padding of the head of the [`Card`](Card).
    pub fn padding_head(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self
    }

    /// Sets the padding of the body of the [`Card`](Card).
    pub fn padding_body(mut self, padding: f32) -> Self {
        self.padding_body = padding;
        self
    }

    /// Sets the padding of the foot of the [`Card`](Card).
    pub fn padding_foot(mut self, padding: f32) -> Self {
        self.padding_foot = padding;
        self
    }

    /// Sets the size of the close icon of the [`Card`](Card).
    pub fn close_size(mut self, size: f32) -> Self {
        self.close_size = Some(size);
        self
    }

    /// Sets the message that will be produced when the close icon of the
    /// [`Card`](Card) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the [`Card`](Card).
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Sets the style of the [`Card`](Card).
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Card<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.max_width(self.max_width).max_height(self.max_height);

        let head_node = head_node(
            renderer,
            &limits,
            &self.head,
            self.padding_head,
            self.width,
            self.on_close.is_some(),
            self.close_size,
        );

        let mut body_node = body_node(renderer, &limits, &self.body, self.padding_body, self.width);

        body_node.move_to(Point::new(
            body_node.bounds().x,
            body_node.bounds().y + head_node.bounds().height,
        ));

        let mut foot_node = self
            .foot
            .as_ref()
            .map_or_else(iced_native::layout::Node::default, |foot| {
                foot_node(renderer, &limits, foot, self.padding_foot, self.width)
            });

        foot_node.move_to(Point::new(
            foot_node.bounds().x,
            foot_node.bounds().y + head_node.bounds().height + body_node.bounds().height,
        ));

        iced_native::layout::Node::with_children(
            Size::new(
                body_node.size().width,
                head_node.size().height + body_node.size().height + foot_node.size().height,
            ),
            vec![head_node, body_node, foot_node],
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Shell<Message>,
    ) -> event::Status {
        let mut children = layout.children();

        let head_layout = children
            .next()
            .expect("Native: Layout should have a head layout");
        let mut head_children = head_layout.children();
        let head_status = self.head.on_event(
            event.clone(),
            head_children
                .next()
                .expect("Native: Layout should have a head content layout"),
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let close_status = head_children
            .next()
            .map_or(event::Status::Ignored, |close_layout| {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. }) => self
                        .on_close
                        .clone()
                        // TODO: `let` expressions in this position are experimental
                        // see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
                        .filter(|_| close_layout.bounds().contains(cursor_position))
                        .map_or(event::Status::Ignored, |on_close| {
                            messages.publish(on_close);
                            event::Status::Captured
                        }),
                    _ => event::Status::Ignored,
                }
            });

        let body_layout = children
            .next()
            .expect("Native: Layout should have a body layout");
        let mut body_children = body_layout.children();
        let body_status = self.body.on_event(
            event.clone(),
            body_children
                .next()
                .expect("Native: Layout should have a body content layout"),
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let foot_layout = children
            .next()
            .expect("Native: Layout should have a foot layout");
        let mut foot_children = foot_layout.children();
        let foot_status = self.foot.as_mut().map_or(event::Status::Ignored, |foot| {
            foot.on_event(
                event,
                foot_children
                    .next()
                    .expect("Native: Layout should have a foot content layout"),
                cursor_position,
                renderer,
                clipboard,
                messages,
            )
        });

        head_status
            .merge(close_status)
            .merge(body_status)
            .merge(foot_status)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();

        let mut head_layout_children = children
            .next()
            .expect("Graphics: Layout should have a head layout")
            .children();
        let _head = head_layout_children.next();
        let close_layout = head_layout_children.next();

        let is_mouse_over_close = close_layout.map_or(false, |layout| {
            let bounds = layout.bounds();
            bounds.contains(cursor_position)
        });

        let mouse_interaction = if is_mouse_over_close {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        mouse_interaction
            .max(
                self.head
                    .mouse_interaction(layout, cursor_position, viewport, renderer),
            )
            .max(
                self.body
                    .mouse_interaction(layout, cursor_position, viewport, renderer),
            )
            .max(
                self.foot
                    .as_ref()
                    .map_or(mouse::Interaction::default(), |foot| {
                        foot.mouse_interaction(layout, cursor_position, viewport, renderer)
                    }),
            )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let style_sheet = self.style_sheet.active();

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: style_sheet.border_radius,
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color,
            },
            style_sheet.background,
        );

        // Border
        renderer.fill_quad(
            // TODO: fill not necessary
            renderer::Quad {
                bounds,
                border_radius: style_sheet.border_radius,
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color,
            },
            Color::TRANSPARENT,
        );

        // ----------- Head ----------------------
        let head_layout = children
            .next()
            .expect("Graphics: Layout should have a head layout");
        draw_head(
            renderer,
            &self.head,
            head_layout,
            cursor_position,
            viewport,
            &style_sheet,
        );

        // ----------- Body ----------------------
        let body_layout = children
            .next()
            .expect("Graphics: Layout should have a body layout");
        draw_body(
            renderer,
            &self.body,
            body_layout,
            cursor_position,
            viewport,
            &style_sheet,
        );

        // ----------- Foot ----------------------
        let foot_layout = children
            .next()
            .expect("Graphics: Layout should have a foot layout");
        draw_foot(
            renderer,
            &self.foot,
            foot_layout,
            cursor_position,
            viewport,
            &style_sheet,
        );
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);
        self.head.hash_layout(state);
        self.body.hash_layout(state);
        if let Some(foot) = self.foot.as_ref() {
            foot.hash_layout(state);
        };
    }
}

/// Calculates the layout of the head.
fn head_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    head: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
    on_close: bool,
    close_size: Option<f32>,
) -> iced_native::layout::Node
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    let pad = Padding::from(padding as u16);
    let mut limits = limits.loose().width(width).height(head.height()).pad(pad);

    let close_size = close_size.unwrap_or_else(|| f32::from(renderer.default_size()));
    let mut close = if on_close {
        limits = limits.shrink(Size::new(close_size, 0.0));
        Some(iced_native::layout::Node::new(Size::new(
            close_size, close_size,
        )))
    } else {
        None
    };

    let mut head = head.layout(renderer, &limits);
    let mut size = limits.resolve(head.size());

    head.move_to(Point::new(padding, padding));
    head.align(Alignment::Start, Alignment::Center, head.size());

    if let Some(node) = close.as_mut() {
        size = Size::new(size.width + close_size, size.height);

        node.move_to(Point::new(size.width - padding, padding));
        node.align(Alignment::End, Alignment::Center, node.size());
    }

    iced_native::layout::Node::with_children(
        size.pad(pad),
        match close {
            Some(node) => vec![head, node],
            None => vec![head],
        },
    )
}

/// Calculates the layout of the body.
fn body_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    body: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
) -> iced_native::layout::Node
where
    Renderer: iced_native::Renderer,
{
    let pad = Padding::from(padding as u16);
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(body.height())
        .pad(pad);

    let mut body = body.layout(renderer, &limits);
    let size = limits.resolve(body.size());

    body.move_to(Point::new(padding, padding));
    body.align(Alignment::Start, Alignment::Start, size);

    iced_native::layout::Node::with_children(size.pad(pad), vec![body])
}

/// Calculates the layout of the foot.
fn foot_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    foot: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
) -> iced_native::layout::Node
where
    Renderer: iced_native::Renderer,
{
    let pad = Padding::from(padding as u16);
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(foot.height())
        .pad(pad);

    let mut foot = foot.layout(renderer, &limits);
    let size = limits.resolve(foot.size());

    foot.move_to(Point::new(padding, padding));
    foot.align(Alignment::Start, Alignment::Center, size);

    iced_native::layout::Node::with_children(size.pad(pad), vec![foot])
}

/// Draws the head of the card.
fn draw_head<Message, Renderer>(
    renderer: &mut Renderer,
    head: &Element<'_, Message, Renderer>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    let mut head_children = layout.children();

    // Head background
    renderer.fill_quad(
        renderer::Quad {
            bounds: layout.bounds(),
            border_radius: style.border_radius,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style.head_background,
    );

    head.draw(
        renderer,
        &renderer::Style {
            text_color: style.head_text_color,
        },
        head_children
            .next()
            .expect("Graphics: Layout should have a head content layout"),
        cursor_position,
        viewport,
    );

    let mut buffer = [0; 4];

    if let Some(close_layout) = head_children.next() {
        let close_bounds = close_layout.bounds();
        let is_mouse_over_close = close_bounds.contains(cursor_position);

        renderer.fill_text(iced_native::text::Text {
            content: char::from(Icon::X).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: close_bounds.center_x(),
                y: close_bounds.center_y(),
                ..close_bounds
            },
            size: close_layout.bounds().height + if is_mouse_over_close { 5.0 } else { 0.0 },
            color: style.close_color,
            font: crate::graphics::icons::ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });
    }
}

/// Draws the body of the card.
fn draw_body<Message, Renderer>(
    renderer: &mut Renderer,
    body: &Element<'_, Message, Renderer>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    let mut body_children = layout.children();

    // Body background
    renderer.fill_quad(
        renderer::Quad {
            bounds: layout.bounds(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style.body_background,
    );

    body.draw(
        renderer,
        &renderer::Style {
            text_color: style.body_text_color,
        },
        body_children
            .next()
            .expect("Graphics: Layout should have a body content layout"),
        cursor_position,
        viewport,
    );
}

/// Draws the foot of the card.
fn draw_foot<Message, Renderer>(
    renderer: &mut Renderer,
    foot: &Option<Element<'_, Message, Renderer>>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    let mut foot_children = layout.children();

    // Foot background
    renderer.fill_quad(
        renderer::Quad {
            bounds: layout.bounds(),
            border_radius: style.border_radius,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style.foot_background,
    );

    if let Some(foot) = foot.as_ref() {
        foot.draw(
            renderer,
            &renderer::Style {
                text_color: style.foot_text_color,
            },
            foot_children
                .next()
                .expect("Graphics: Layout should have a foot content layout"),
            cursor_position,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<Card<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font> + 'a,
    Message: Clone + 'a,
{
    fn from(card: Card<'a, Message, Renderer>) -> Self {
        Element::new(card)
    }
}
