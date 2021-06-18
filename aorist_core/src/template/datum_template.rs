use aorist_attributes::Attribute;

pub trait TDatumTemplate {
    fn get_attributes(&self) -> Vec<Attribute>;
    fn get_name(&self) -> String;
}
