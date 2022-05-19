use bevy::prelude::*;

use crate::interaction::{Drag, Hover};

#[derive(Debug, PartialEq)]
pub enum InteractionEvent {
    DragStarted(Entity),
    DragStopped(Entity),
    HoverStarted(Entity),
    HoverStopped(Entity),
    NoInteraction(Entity),
}

#[allow(clippy::type_complexity)]
pub(crate) fn send_events(
    mut event_writer: EventWriter<InteractionEvent>,
    interaction_query: Query<
        (
            Entity,
            &Hover,
            &Drag,
            ChangeTrackers<Hover>,
            ChangeTrackers<Drag>,
        ),
        Or<(Changed<Hover>, Changed<Drag>)>,
    >,
) {
    for (entity, hover, drag, hover_track, drag_track) in interaction_query.iter() {

        if hover_track.is_changed() || drag_track.is_changed() {
            if drag_track.is_changed() && !drag_track.is_added() {
                event_writer.send(match drag.dragged {
                    true => InteractionEvent::DragStarted(entity),
                    false => InteractionEvent::DragStopped(entity)
                });
            }
            if hover_track.is_changed() && !hover_track.is_added() {
                event_writer.send(match hover.hovered {
                    true => InteractionEvent::HoverStarted(entity),
                    false => InteractionEvent::HoverStopped(entity),
                });
            }
            if !hover.hovered && !drag.dragged && !drag_track.is_added() && !hover_track.is_added() {
                event_writer.send(InteractionEvent::NoInteraction(entity))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::core::CorePlugin;
    use bevy::prelude::*;
    use crate::event::{send_events, InteractionEvent};
    use crate::interaction::{Drag, Hover};

    #[derive(PartialEq)]
    enum TestStep {
        // This is needed to ignore the first run when the components are being considered 'added'
        Initial,
        // Test that the starting events are being sent
        StartedEvents,
        // Test that the stopping events are being sent, including the 'NoInteraction' event
        StoppedEvents
    }

    fn get_world_and_entity() -> (World, Entity) {

        let mut app = App::new();
        app.add_plugin(CorePlugin);
        app.add_event::<InteractionEvent>();

        let mut world = app.world;
        let entity = world
            .spawn()
            .insert(Hover::default())
            .insert(Drag::default())
            .id();

        (world, entity)
    }

    fn propagate_test(mut test_step: ResMut<TestStep>) {
        match *test_step {
            TestStep::Initial => *test_step = TestStep::StartedEvents,
            TestStep::StartedEvents => *test_step = TestStep::StoppedEvents,
            _ => {}
        }
    }

    fn trigger_events(mut query: Query<(&mut Hover, &mut Drag)>, test_step: Res<TestStep>) {
        if *test_step != TestStep::Initial {
            for (mut hover, mut drag) in query.iter_mut() {
                hover.hovered = !hover.hovered;
                drag.dragged = !drag.dragged;
            }
        }
    }

    #[test]
    fn test_events() {

        let (mut world, entity) = get_world_and_entity();
        world.insert_resource(TestStep::Initial);

        let read_and_assert = move |mut events: EventReader<InteractionEvent>, test_step: ResMut<TestStep>| {

            match *test_step {
                TestStep::StartedEvents => {
                    assert_eq!(events.len(), 2, "Expected {} events got {}", 2, events.len());

                    let mut iter = events.iter();
                    let event1 = iter.next().unwrap();
                    let event2 = iter.next().unwrap();

                    assert_eq!(InteractionEvent::DragStarted(entity), *event1);
                    assert_eq!(InteractionEvent::HoverStarted(entity), *event2);
                },
                TestStep::StoppedEvents => {
                    assert_eq!(events.len(), 3, "Expected {} events got {}", 3, events.len());

                    let mut iter = events.iter();
                    let event1 = iter.next().unwrap();
                    let event2 = iter.next().unwrap();
                    let event3 = iter.next().unwrap();

                    assert_eq!(InteractionEvent::DragStopped(entity), *event1);
                    assert_eq!(InteractionEvent::HoverStopped(entity), *event2);
                    assert_eq!(InteractionEvent::NoInteraction(entity), *event3);
                }
                _ => {}
            }
        };

        let mut stage = SystemStage::parallel();
        stage.add_system_set(
            SystemSet::new()
                .with_system(trigger_events)
                .with_system(send_events.after(trigger_events))
                .with_system(read_and_assert.after(send_events))
                .with_system(propagate_test.after(read_and_assert))
        );

        stage.run(&mut world);
        stage.run(&mut world);
        stage.run(&mut world);
    }
}
