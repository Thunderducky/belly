use super::parse;
use crate::style_property;
use bevy::prelude::*;

style_property! {
    #[doc = " Specify element flex direction by providing value to `Style.direction`:"]
    #[doc = " ```css"]
    #[doc = " flex-direction: column;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " The `flex-direction` property sets how flex items are placed in the flex"]
    #[doc = " container defining the main axis and the direction (normal or reversed)."]
    #[doc = " "]
    #[doc = " Supported values:"]
    #[doc = " - `row`: The flex container's main-axis is defined to be the same as the"]
    #[doc = "   text direction."]
    #[doc = " - `column`: The flex container's main-axies is defined to be vertical, items"]
    #[doc = "   are placed from top to bottom."]
    #[doc = " - `row-reverse`: Behaves the same as `row` but opposite to the content direction."]
    #[doc = " - `column-reverse`: Behaves the same as `row` but items are placed from bottom to"]
    #[doc = "    top."]
    #[doc = " <!-- @property-category=Flex Container -->"]
    FlexDirectionProperty("flex-direction") {
        Default = "row";
        Item = FlexDirection;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::IdentifierParser<FlexDirection>;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.flex_direction != value {
                style.flex_direction = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element flex wrap by providing value to `Style.flex_wrap`:"]
    #[doc = " ```css"]
    #[doc = " flex-wrap: wrap;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " The `flex-wrap` property sets whether flex items are forced onto one"]
    #[doc = " line or can wrap onto multiple lines. If wrapping is allowed, it sets"]
    #[doc = " the direction that lines are stacked."]
    #[doc = " "]
    #[doc = " Supported values:"]
    #[doc = " - `no-wrap`: The flex items are laid out in a single line which may cause"]
    #[doc = "   the flex container to overflow."]
    #[doc = " - `wrap`: The flex items break into multiple lines."]
    #[doc = " - `wrap-reverse`: Behaves the same as wrap but the new line is placed before"]
    #[doc = "    the previous"]
    #[doc = " <!-- @property-category=Flex Container -->"]
    FlexWrapProperty("flex-wrap") {
        Default = "no-wrap";
        Item = FlexWrap;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::IdentifierParser<FlexWrap>;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.flex_wrap != value {
                style.flex_wrap = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element items alignment by providing value to `Style.align_items`:"]
    #[doc = " ```css"]
    #[doc = " align-items: center;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " The `align-items` property sets the `align-self` value on all direct children"]
    #[doc = " as a group. In [Flexbox](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout)."]
    #[doc = " it controls the alignment of items on the [Cross Axis](https://developer.mozilla.org/en-US/docs/Glossary/Cross_Axis)."]
    #[doc = " "]
    #[doc = " Supported values:"]
    #[doc = " - `flex-start`: The cross-start margin edges of the flex items are flushed with"]
    #[doc = "   the cross-start edge of the line."]
    #[doc = " - `flex-end`: The cross-start margin edges of the flex items are flushed with"]
    #[doc = "   the cross-start edge of the line."]
    #[doc = " - `center`: The flex items' margin boxes are centered within the line on the"]
    #[doc = "   cross-axis. If the cross-size of an item is larger than the flex container,"]
    #[doc = "   it will overflow equally in both directions."]
    #[doc = " - `baseline`: All flex items are aligned such that their"]
    #[doc = "   [flex container baselines](https://drafts.csswg.org/css-flexbox-1/#flex-baselines) align"]
    #[doc = " - `stretch`: Flex items are stretched such that the cross-size of the item's margin"]
    #[doc = "   box is the same as the line while respecting width and height constraints."]
    #[doc = " <!-- @property-category=Flex Container -->"]
    AlignItemsProperty("align-items") {
        Default = "stretch";
        Item = AlignItems;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::IdentifierParser<AlignItems>;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.align_items != value {
                style.align_items = *value;
            }
        };
    }
}

style_property! {
    #[doc = " TODO: write AlignContent description"]
    #[doc = " <!-- @property-category=Flex Container -->"]
    AlignContentProperty("align-content") {
        Default = "stretch";
        Item = AlignContent;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::IdentifierParser<AlignContent>;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.align_content != value {
                style.align_content = *value;
            }
        };
    }
}

style_property! {
    #[doc = " TODO: write JustifyContent description"]
    #[doc = " <!-- @property-category=Flex Container -->"]
    JustifyContentProperty("justify-content") {
        Default = "flex-start";
        Item = JustifyContent;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::IdentifierParser<JustifyContent>;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.justify_content != value {
                style.justify_content = *value;
            }
        };
    }
}
