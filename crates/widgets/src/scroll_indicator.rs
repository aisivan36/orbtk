use crate::prelude::*;

/// The `ScrollIndicatorState` handles the `ScrollIndicator` widget.
#[derive(Default)]
pub struct ScrollIndicatorState;

impl State for ScrollIndicatorState {
    fn update_post_layout(&self, ctx: &mut Context<'_>) {
        let padding = *ctx.widget().get::<Thickness>("padding");
        let scroll_offset = *ctx.widget().get::<Point>("scroll_offset");
        let content_id = ctx.widget().get::<ContentId>("content_id").0;
        let content_bounds = ctx
            .get_widget(Entity::from(content_id))
            .get::<Rectangle>("bounds")
            .clone();
        let bounds = ctx.widget().get::<Rectangle>("bounds").clone();

        let horizontal_p = bounds.width / content_bounds.width;
        let vertical_p = bounds.height / content_bounds.height;

        // calculate vertical scroll bar height and position.
        if let Some(mut vertical_scroll_bar) = ctx.child_by_id("vertical-scroll-bar") {
            if vertical_p < 1.0 {
                vertical_scroll_bar.set("visibility", Visibility::from("visible"));
                let scroll_bar_margin_bottom =
                    vertical_scroll_bar.get::<Thickness>("margin").bottom();
                let vertical_min_height = vertical_scroll_bar
                    .get::<Constraint>("constraint")
                    .min_height();
                let height =
                    ((bounds.height - padding.top - padding.bottom - scroll_bar_margin_bottom)
                        * vertical_p)
                        .max(vertical_min_height);

                let scroll_bar_bounds = vertical_scroll_bar.get_mut::<Rectangle>("bounds");
                scroll_bar_bounds.height = height;
                scroll_bar_bounds.y = -(scroll_offset.y as f64 * vertical_p);
            } else {
                vertical_scroll_bar.set("visibility", Visibility::from("Collapsed"));
            }
        }

        // calculate horizontal scroll bar width and position.
        if let Some(mut horizontal_scroll_bar) = ctx.child_by_id("horizontal-scroll-bar") {
            if horizontal_p < 1.0 {
                horizontal_scroll_bar.set("visibility", Visibility::from("Visible"));
                let scroll_bar_margin_right =
                    horizontal_scroll_bar.get::<Thickness>("margin").right();
                let horizontal_min_width = horizontal_scroll_bar
                    .get::<Constraint>("constraint")
                    .min_width();
                let width =
                    ((bounds.width - padding.left - padding.right - scroll_bar_margin_right)
                        * horizontal_p)
                        .max(horizontal_min_width);
                let scroll_bar_bounds = horizontal_scroll_bar.get_mut::<Rectangle>("bounds");
                scroll_bar_bounds.width = width;
                scroll_bar_bounds.x = -(scroll_offset.x as f64 * horizontal_p);
            } else {
                horizontal_scroll_bar.set("visibility", Visibility::from("Collapsed"));
            }
        }
    }
}

property!(
    /// Internal content id property of ScrollIndicator. Is used to get the size of the scrolled content.
    #[derive(Default)]
    ContentId(u32)
);

widget!(
    /// The `ScrollIndicator` widget contains two scroll bars.
    ///
    /// **CSS element:** `scroll-indicator`
    ScrollIndicator<ScrollIndicatorState> {
        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the scroll offset property.
        scroll_offset: Point,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the content id property.
        content_id: ContentId
    }
);

impl Template for ScrollIndicator {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ScrollIndicator")
            .selector("scroll-indicator")
            .vertical_alignment("Stretch")
            .horizontal_alignment("Stretch")
            .padding(0.0)
            .child(
                Grid::create()
                    .child(
                        ScrollBar::create()
                            .selector(SelectorValue::from("scroll-bar").id("vertical-scroll-bar"))
                            .min_height(8.0)
                            .margin((0.0, 0.0, 0.0, 6.0))
                            .horizontal_alignment("end")
                            .build(ctx),
                    )
                    .child(
                        ScrollBar::create()
                            .selector(SelectorValue::from("scroll-bar").id("horizontal-scroll-bar"))
                            .min_width(8.0)
                            .margin((0.0, 0.0, 6.0, 0.0))
                            .height(4.0)
                            .vertical_alignment("end")
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}
