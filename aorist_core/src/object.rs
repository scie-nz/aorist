use enum_dispatch::enum_dispatch;

#[enum_dispatch(AoristObject)]
pub trait TAoristObject {
    fn get_name(&self) -> &String;
}
