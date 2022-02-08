use abi_stable::std_types::ROption;
use aorist_util::{AOption, AString, AUuid, AVec, ATaskId};
use inflector::cases::snakecase::to_snake_case;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AncestorRecord {
    pub uuid: AUuid,
    pub object_type: AString,
    pub tag: AOption<AString>,
    pub ix: usize,
}
impl AncestorRecord {
    pub fn new(uuid: AUuid, object_type: AString, tag: AOption<AString>, ix: usize) -> Self {
        Self {
            uuid,
            object_type,
            tag,
            ix,
        }
    }
    pub fn get_key(&self) -> ATaskId {
        ATaskId::new(self.uuid.clone(), self.object_type.clone())
    }
    pub fn compute_relative_path(ancestors: &AVec<AncestorRecord>) -> AString {
        let mut relative_path: String = "".into();
        for record in ancestors.iter().rev() {
            if let AOption(ROption::RSome(ref t)) = record.tag {
                relative_path = format!("{}__{}", relative_path, t);
                break;
            }
            if record.ix > 0 {
                relative_path = format!(
                    "{}__{}_{}",
                    relative_path,
                    to_snake_case(record.object_type.as_str()),
                    record.ix
                );
            }
        }
        relative_path.as_str().into()
    }
}
