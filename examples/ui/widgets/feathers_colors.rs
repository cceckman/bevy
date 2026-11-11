//! Feathers' theme module allows for dynamic re-theming.

use bevy::{
    color::palettes,
    feathers::{
        constants::{fonts, icons},
        containers::*,
        controls::*,
        dark_theme::create_dark_theme,
        display::{caption, icon, label, label_dim, label_small},
        font_styles::InheritableFont,
        palette,
        rounded_corners::RoundedCorners,
        theme::{ThemeBackgroundColor, UiTheme},
        tokens, FeathersPlugins,
    },
    input_focus::{tab_navigation::TabGroup, AutoFocus, InputFocus},
    picking::cursor::{EntityCursor, OverrideCursor},
    prelude::*,
    text::{EditableText, TextEdit, TextEditChange},
    ui::{Checked, InteractionDisabled, Selected},
    ui_widgets::{
        checkbox_self_update, listbox_update_selection,
        popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide},
        radio_self_update, slider_self_update, Activate, ActivateOnPress, RadioGroup, RequestClose,
        SliderPrecision, SliderStep, SliderValue, ValueChange,
    },
    window::SystemCursorIcon,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .insert_resource(SelectedTheme)
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, update_colors)
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! {
        Camera2d
        --
        @demo_root()
    }
}

fn demo_root() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(8),
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)

        Children [

        ]
    }
}
