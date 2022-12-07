use bevy::{
    asset::Asset,
    ecs::{
        event::Event,
        system::{Command, EntityCommands},
    },
    prelude::*,
    utils::HashMap,
};
use itertools::Itertools;
use std::ops::{Deref, DerefMut};

use crate::{ElementsBuilder, PointerInput, WithElements};

use super::RelationsSystems;
pub trait Signal: Event {
    fn sources(&self) -> &[Entity];
}

impl Signal for PointerInput {
    fn sources(&self) -> &[Entity] {
        &self.entities
    }
}

pub struct ConnectionGeneralContext<'a, 'w, 's, S: Signal> {
    pub(crate) source_event: &'a S,
    pub(crate) source: Entity,
    pub(crate) time_resource: &'a Time,
    pub(crate) asset_server: AssetServer,
    pub(crate) commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's, S: Signal> ConnectionGeneralContext<'a, 'w, 's, S> {
    pub fn event(&self) -> &S {
        self.source_event
    }
    pub fn source<'x>(&'x mut self) -> EntityCommands<'w, 's, 'x> {
        let source = self.source;
        self.commands.entity(source)
    }
    pub fn load<T: Asset>(&self, path: &str) -> Handle<T> {
        self.asset_server.load(path)
    }
    pub fn add<C: Command>(&mut self, command: C) {
        self.commands.add(command);
    }
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }
    pub fn time(&self) -> &Time {
        self.time_resource
    }
}

pub struct ConnectionEntityContext<'a, 'w, 's, 'c, S: Signal> {
    pub(crate) target: Entity,
    pub(crate) ctx: &'c mut ConnectionGeneralContext<'a, 'w, 's, S>,
}

impl<'a, 'w, 's, 'c, S: Signal> ConnectionEntityContext<'a, 'w, 's, 'c, S> {
    pub fn target<'x>(&'x mut self) -> EntityCommands<'w, 's, 'x> {
        let target = self.target;
        self.commands.entity(target)
    }

    pub fn render(&mut self, eml: ElementsBuilder) {
        self.target().with_elements(eml);
    }

    pub fn replace(&mut self, eml: ElementsBuilder) {
        self.target().despawn_descendants();
        self.target().with_elements(eml);
    }
}

impl<'a, 'w, 's, 'c, S: Signal> Deref for ConnectionEntityContext<'a, 'w, 's, 'c, S> {
    type Target = ConnectionGeneralContext<'a, 'w, 's, S>;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a, 'w, 's, 'c, S: Signal> DerefMut for ConnectionEntityContext<'a, 'w, 's, 'c, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub enum ConnectionTo<C: Component, S: Signal> {
    General {
        handler: fn(&mut ConnectionGeneralContext<S>),
    },
    Entity {
        target: Entity,
        handler: fn(&mut ConnectionEntityContext<S>),
    },
    Component {
        target: Entity,
        handler: fn(&mut ConnectionEntityContext<S>, &mut Mut<C>),
    },
}

#[derive(Component)]
pub struct WithoutComponent;

impl<C: Component, S: Signal> ConnectionTo<C, S> {
    pub fn component(
        target: Entity,
        handler: fn(&mut ConnectionEntityContext<S>, &mut Mut<C>),
    ) -> ConnectionTo<C, S> {
        ConnectionTo::Component { target, handler }
    }

    pub fn filter(self, filter: fn(&S) -> bool) -> Connection<C, S> {
        Connection {
            target: self,
            filter,
        }
    }

    pub fn id(&self) -> Option<Entity> {
        match self {
            ConnectionTo::Component { target, handler: _ } => Some(*target),
            ConnectionTo::Entity { target, handler: _ } => Some(*target),
            _ => None,
        }
    }
}

impl<S: Signal> ConnectionTo<WithoutComponent, S> {
    pub fn entity(
        target: Entity,
        handler: fn(&mut ConnectionEntityContext<S>),
    ) -> ConnectionTo<WithoutComponent, S> {
        ConnectionTo::Entity { target, handler }
    }

    pub fn general(
        handler: fn(&mut ConnectionGeneralContext<S>),
    ) -> ConnectionTo<WithoutComponent, S> {
        ConnectionTo::General { handler }
    }
}

pub struct Connection<C: Component, S: Signal> {
    pub target: ConnectionTo<C, S>,
    filter: fn(&S) -> bool,
}

impl<C: Component, S: Signal> Connection<C, S> {
    pub fn handles(&self, signal: &S) -> bool {
        (self.filter)(signal)
    }

    pub fn from(self, source: Entity) -> Connect<C, S> {
        Connect {
            source,
            target: self,
        }
    }
}

pub struct Connect<C: Component, S: Signal> {
    source: Entity,
    target: Connection<C, S>,
}

impl<C: Component, S: Signal> Connect<C, S> {
    pub fn write(self, world: &mut World) {
        {
            let systems = world.get_resource_or_insert_with(RelationsSystems::default);
            systems.0.write().unwrap().add_signals_processor::<C, S>();
        }
        {
            let mut connections = world.get_resource_or_insert_with(Connections::<C, S>::default);
            connections.add(self);
        }
    }
}

// impl<C: Component, S: Signal> Command for Connect<C, S> { }

#[derive(Resource)]
pub struct Connections<C: Component, S: Signal> {
    map: HashMap<Entity, Vec<Connection<C, S>>>,
    index: HashMap<Entity, Vec<Entity>>,
}

impl<C: Component, S: Signal> Connections<C, S> {
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.map
            .keys()
            .chain(self.index.keys())
            .map(|e| *e)
            .unique()
    }
}

impl<C: Component, S: Signal> Deref for Connections<C, S> {
    type Target = HashMap<Entity, Vec<Connection<C, S>>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<C: Component, S: Signal> Default for Connections<C, S> {
    fn default() -> Self {
        Connections {
            map: Default::default(),
            index: Default::default(),
        }
    }
}

impl<C: Component, S: Signal> Connections<C, S> {
    pub fn add(&mut self, connection: Connect<C, S>) {
        if let Some(target) = connection.target.target.id() {
            self.index
                .entry(target)
                .or_default()
                .push(connection.source)
        }
        self.map
            .entry(connection.source)
            .or_default()
            .push(connection.target);
    }
    pub fn remove(&mut self, source: &Entity) {
        if let Some(connections_to) = self.index.remove(source) {
            for connection_to in connections_to.iter() {
                self.map
                    .entry(*connection_to)
                    .and_modify(|e| e.retain(|c| c.target.id() != Some(*source)));
            }
        }
        self.map.remove(&source);
    }
}

#[macro_export]
macro_rules! connect {
    ($entity:expr, |$ctx:ident, $arg:ident: $typ:ty| $cb:expr) => {
        $crate::relations::ConnectionTo::component(
            $entity,
            |$ctx, $arg: &mut ::bevy::prelude::Mut<$typ>| $cb,
        )
    };
    ($entity:expr, |$ctx:ident, $arg:ident: $typ:ty| $cb:block) => {
        $crate::relations::ConnectionTo::component(
            $entity,
            |$ctx, $arg: &mut ::bevy::prelude::Mut<$typ>| $cb,
        )
    };
    ($entity:expr, |$arg:ident: $typ:ty| $cb:expr) => {
        $crate::relations::ConnectionTo::component(
            $entity,
            |_, $arg: &mut ::bevy::prelude::Mut<$typ>| $cb,
        )
    };
    ($entity:expr, |$arg:ident: $typ:ty| $cb:block) => {
        $crate::relations::ConnectionTo::component($entity, |_, $arg| $cb)
    };
    ($entity:expr, |$ctx:ident| $cb:expr) => {
        $crate::relations::ConnectionTo::entity($entity, |$ctx| $cb)
    };
    (|$ctx:ident| $cb:expr) => {
        $crate::relations::ConnectionTo::general(|$ctx| $cb)
    };
}