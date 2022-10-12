
pub trait EventTrait: serde_traitobject::Serialize + Send + Sync + Clone {
    fn test(&self);
}
