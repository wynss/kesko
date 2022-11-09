use bevy::prelude::*;

use crate::interaction::{Drag, Hover, Select};

#[derive(Debug, PartialEq, Eq)]
pub enum InteractionEvent {
    DragStarted(Entity),
    DragStopped(Entity),
    Selected(Entity),
    Deselected(Entity),
    HoverStarted(Entity),
    HoverStopped(Entity),
    NoInteraction(Entity),
}

/// Event that can be sent to select/deselect an entity
#[derive(Debug, PartialEq, Eq)]
pub enum SelectEvent {
    Select(Entity),
    Deselect(Entity),
}

/// System that will select an entity based on SelectEvents. This in order to be able to select
/// entities programmatically.
pub(crate) fn handle_select_events<T: Component + Default>(
    mut select_event_reader: EventReader<SelectEvent>,
    mut query: Query<&mut Select<T>, With<Select<T>>>,
) {
    for event in select_event_reader.iter() {
        let (should_select, entity) = match event {
            SelectEvent::Select(entity) => (true, entity),
            SelectEvent::Deselect(entity) => (false, entity),
        };

        if let Ok(mut select) = query.get_mut(*entity) {
            select.selected = should_select;
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn send_interaction_events<T: Component + Default>(
    mut event_writer: EventWriter<InteractionEvent>,
    interaction_query: Query<
        (
            Entity,
            &Hover<T>,
            &Drag<T>,
            &Select<T>,
            ChangeTrackers<Hover<T>>,
            ChangeTrackers<Drag<T>>,
            ChangeTrackers<Select<T>>,
        ),
        Or<(Changed<Hover<T>>, Changed<Drag<T>>, Changed<Select<T>>)>,
    >,
) {
    for (entity, hover, drag, select, hover_track, drag_track, select_track) in
        interaction_query.iter()
    {
        if drag_track.is_changed() && !drag_track.is_added() {
            event_writer.send(match drag.dragged {
                true => InteractionEvent::DragStarted(entity),
                false => InteractionEvent::DragStopped(entity),
            });
        }

        if select_track.is_changed() && !select_track.is_added() {
            event_writer.send(match select.selected {
                true => InteractionEvent::Selected(entity),
                false => InteractionEvent::Deselected(entity),
            });
        }

        if hover_track.is_changed() && !hover_track.is_added() {
            event_writer.send(match hover.hovered {
                true => InteractionEvent::HoverStarted(entity),
                false => InteractionEvent::HoverStopped(entity),
            });
        }

        if !hover.hovered
            && !drag.dragged
            && !select.selected
            && !select_track.is_added()
            && !hover_track.is_added()
            && !drag_track.is_added()
        {
            event_writer.send(InteractionEvent::NoInteraction(entity))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{send_interaction_events, InteractionEvent};
    use crate::interaction::{Drag, Hover, Select};
    use bevy::core::CorePlugin;
    use bevy::prelude::*;

    use super::{handle_select_events, SelectEvent};

    #[derive(Component, Default)]
    struct TestGroup;

    #[derive(PartialEq)]
    enum TestStep {
        // This is needed to ignore the first run when the components are being considered 'added'
        Initial,
        // Test that the starting events are being sent
        StartedEvents,
        // Test that the stopping events are being sent, including the 'NoInteraction' event
        StoppedEvents,
    }

    fn get_world_and_entity() -> (World, Entity) {
        let mut app = App::new();
        app.add_plugin(CorePlugin);
        app.add_event::<InteractionEvent>();
        app.add_event::<SelectEvent>();

        let mut world = app.world;
        let entity = world
            .spawn()
            .insert(Hover::<TestGroup>::default())
            .insert(Drag::<TestGroup>::default())
            .insert(Select::<TestGroup>::default())
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

    fn trigger_events(
        mut query: Query<(&mut Hover<TestGroup>, &mut Drag<TestGroup>)>,
        test_step: Res<TestStep>,
    ) {
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

        let read_and_assert = move |mut events: EventReader<InteractionEvent>,
                                    test_step: ResMut<TestStep>| {
            match *test_step {
                TestStep::StartedEvents => {
                    assert_eq!(
                        events.len(),
                        2,
                        "Expected {} events got {}",
                        2,
                        events.len()
                    );

                    let mut iter = events.iter();
                    let event1 = iter.next().unwrap();
                    let event2 = iter.next().unwrap();

                    assert_eq!(InteractionEvent::DragStarted(entity), *event1);
                    assert_eq!(InteractionEvent::HoverStarted(entity), *event2);
                }
                TestStep::StoppedEvents => {
                    assert_eq!(
                        events.len(),
                        3,
                        "Expected {} events got {}",
                        3,
                        events.len()
                    );

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
                .with_system(send_interaction_events::<TestGroup>.after(trigger_events))
                .with_system(read_and_assert.after(send_interaction_events::<TestGroup>))
                .with_system(propagate_test.after(read_and_assert)),
        );

        stage.run(&mut world);
        stage.run(&mut world);
        stage.run(&mut world);
    }

    #[test]
    fn test_select_events() {
        let (mut world, entity) = get_world_and_entity();

        // make sure that the entity is not selected before running
        let mut query = world.query::<&Select<TestGroup>>();
        let select_before = query
            .get_single(&world)
            .expect("World should have one entity");
        assert!(
            !select_before.selected,
            "Entity was selected but should have been deselected"
        );

        // setup and run systems once
        let mut stage = SystemStage::single_threaded();

        world.send_event(SelectEvent::Select(entity));
        stage.add_system_set(SystemSet::new().with_system(handle_select_events::<TestGroup>));
        stage.run(&mut world);

        // check that the entity is now selected
        let mut query = world.query::<&Select<TestGroup>>();
        let select_after = query
            .get_single(&world)
            .expect("World should have one entity");
        assert!(
            select_after.selected,
            "Entity was deselected but should have been selected"
        );

        world.send_event(SelectEvent::Deselect(entity));
        stage.run(&mut world);

        // check that the entity is now deselected
        let mut query = world.query::<&Select<TestGroup>>();
        let select_after = query
            .get_single(&world)
            .expect("World should have one entity");
        assert!(
            !select_after.selected,
            "Entity was selected but should have been deselected"
        );
    }
}
