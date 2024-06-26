use super::event::Event;
use crate::{
    prelude::{Error, NonEmptyVec},
    Unit,
};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait Command<State>: Send + Sync
where
    State: Debug + Clone + Send + Sync + 'static,
{
    type T: Event<State> + Debug + DeserializeOwned + Serialize + 'static;

    /// Validate the command. This function will be called before the `directive`
    /// function. If the command is invalid, an error should be returned.
    fn validate(&self, state: &State) -> Result<Unit, Error>;

    /// Yield a directive. Essentially, it should return an event or a list of events.
    /// Event order is ensured and enforced by the engine.
    fn directive(&self, state: &State) -> Result<NonEmptyVec<Box<Self::T>>, Error>;

    /// Return the entity id of the entity.
    ///
    /// Make sure that all commands that are sent to the same entity have the same
    /// entity id.
    ///
    /// The entity id will be validated by the engine, if the entity id is invalid, the
    /// command will be rejected.
    ///
    /// The format of the entity id is up to the user, up to certain constraints:
    ///
    /// - The entity id must be unique.
    /// - The entity id must be a string.
    fn entity_id(&self) -> String;

    /// Performs side effects based on the application of the event.
    ///
    /// This method is not pure and may trigger side effects. It does not modify the state.
    #[allow(unused_variables)]
    fn effects(&self, before: &State, after: &State) -> impl Future<Output = Result<Unit, Error>> {
        async move { Ok(()) }
    }

    /// Return the name of the command.
    fn name(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }
}
