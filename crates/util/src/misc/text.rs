use bevy::{prelude::*, sprite::Anchor};

pub fn spawn(
    commands: &mut Commands,
    entity: Entity,
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    justify: JustifyText,
    anchor: Anchor,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {
    let style = TextStyle {
        font,
        font_size,
        color,
    };
    let text_entity = commands
        .spawn(Text2dBundle {
            text: Text::from_section(text, style).with_justify(justify),
            transform: Transform::from_xyz(x, y, z),
            text_anchor: anchor,
            ..Default::default()
        })
        .id();
    commands.entity(entity).push_children(&[text_entity]);
    text_entity
}
pub fn set_size(text: &mut Text, font_size: f32) {
    for section in text.sections.iter_mut() {
        section.style.font_size = font_size;
    }
}
pub fn set_color(text: &mut Text, color: Color) {
    for section in text.sections.iter_mut() {
        section.style.color = color;
    }
}
pub fn set_size_color(text: &mut Text, font_size: f32, color: Color) {
    for section in text.sections.iter_mut() {
        section.style.font_size = font_size;
        section.style.color = color;
    }
}
pub fn set_value(text: &mut Text, v: String) {
    for section in text.sections.iter_mut() {
        section.value = v;
        return;
    }
}
