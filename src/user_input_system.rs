use amethyst::shrev::EventChannel;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{
        prelude::World,
        prelude::*,
        prelude::{Read, System},
    },
    input::{InputEvent, StringBindings},
    winit::MouseButton,
};

type EventType = InputEvent<StringBindings>;

#[derive(SystemDesc)]
pub struct UserInputSystem {
    pub reader_id: ReaderId<EventType>,
}

impl UserInputSystem {
    pub fn new(reader_id: ReaderId<EventType>) -> Self {
        UserInputSystem { reader_id }
    }
}

impl<'a> System<'a> for UserInputSystem {
    type SystemData = Read<'a, EventChannel<EventType>>;

    fn run(&mut self, channel: Self::SystemData) {
        for event in channel.read(&mut self.reader_id) {
            match event {
                EventType::MouseButtonPressed(MouseButton::Left) => {
                    println!("Received event value of: {:?}", event)
                }
                _ => (),
            }
        }
    }
}

pub struct UserInputSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UserInputSystem> for UserInputSystemDesc {
    fn build(self, world: &mut World) -> UserInputSystem {
        <UserInputSystem as System<'_>>::SystemData::setup(world);

        let reader_id = world
            .fetch_mut::<EventChannel<EventType>>()
            .register_reader();
        UserInputSystem::new(reader_id)
    }
}
