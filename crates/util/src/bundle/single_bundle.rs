use bevy::prelude::*;

use crate::prelude::entity::calc_name;

#[derive(Bundle, Debug)]
pub struct SingleBundle<T: Component> {
    pub name: Name,
    pub value: T,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<T: Component> From<(String, T, Transform)> for SingleBundle<T> {
    fn from(v: (String, T, Transform)) -> Self {
        Self {
            name: calc_name(v.0),
            value: v.1,
            transform: v.2,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
impl<T: Component + ToString> From<(T, Transform)> for SingleBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        (v.0.to_string(), v.0, v.1).into()
    }
}
impl<T: Component + ToString> From<T> for SingleBundle<T> {
    fn from(v: T) -> Self {
        (v, Transform::default()).into()
    }
}

#[derive(Clone, Debug, Component)]
pub struct SingleData<T: Send + Sync + 'static>(pub T);

impl<T: ToString + Send + Sync + 'static> ToString for SingleData<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T: ToString + Send + Sync + 'static> From<T> for SingleBundle<SingleData<T>> {
    fn from(v: T) -> Self {
        (SingleData::<T>(v), Transform::default()).into()
    }
}

