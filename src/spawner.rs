use bevy::app::App;

pub trait SpawnType {
    type Event;
    type EventWriter<'a>;
    type EventReader<'a>;

    fn add_to(self, app: &mut App) -> &mut App;
}

pub trait AppSpawnerExt {
    fn add_spawn_system(&mut self, spawner: impl SpawnType) -> &mut Self;
}

impl AppSpawnerExt for App {
    fn add_spawn_system(&mut self, spawner: impl SpawnType) -> &mut Self {
        spawner.add_to(self)
    }
}

#[macro_export]
macro_rules! tuple_or_single {
    () => {
        ()
    };
    ($x:ty) => {
        $x
    };
    ($x1:ty, $($xx:ty),+ $(,)?) => {
        ($x1, $($xx),*)
    };
}

#[macro_export]
macro_rules! spawner {
    ($category:ident ($($ity:ty),* $(,)?) {$($variant:ident => $fname:ident),* $(,)?}) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub enum $category {
            #[default]
            $($variant),*
        }

        impl $crate::spawner::SpawnType for $category {
            type Event = spew::prelude::SpawnEvent<$category, $crate::tuple_or_single!($($ity),*)>;
            type EventWriter<'a> = bevy::prelude::EventWriter::<'a, Self::Event>;
            type EventReader<'a> = bevy::prelude::EventReader::<'a, 'a, Self::Event>;

            fn add_to(self, app: &mut bevy::app::App) -> &mut bevy::app::App {
                use spew::prelude::*;
                app
                .add_plugin(spew::prelude::SpewPlugin::<$category, $crate::tuple_or_single!($($ity),*)>::default())
                $(.add_spawner(($category :: $variant, $fname)))*
            }
        }
    };
}
