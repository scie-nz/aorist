pub trait TConstraintEnum {
    type BuilderT;
    fn builders() -> Vec<Self::BuilderT>; 
}
pub trait ConstraintEnum {}
