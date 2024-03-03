use bevy::prelude::*;

use edger_bevy_util::prelude::entity::calc_name;

use crate::prelude::LayoutData;

#[derive(Bundle, Debug)]
pub struct ViewBundle<T: Component> {
    pub name: Name,
    pub view: T,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<T: Component> From<(String, T, Transform)> for ViewBundle<T> {
    fn from(v: (String, T, Transform)) -> Self {
        Self {
            name: calc_name(v.0),
            view: v.1,
            layout: LayoutData::default(),
            transform: v.2,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
impl<T: Component + ToString> From<(T, Transform)> for ViewBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        (v.0.to_string(), v.0, v.1).into()
    }
}
impl<T: Component + ToString> From<T> for ViewBundle<T> {
    fn from(v: T) -> Self {
        (v, Transform::default()).into()
    }
}
