use inflector::cases::snakecase::to_snake_case;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AncestorRecord {
    pub uuid: Uuid,
    pub object_type: String,
    pub tag: Option<String>,
    pub ix: usize,
}
impl AncestorRecord {
    pub fn new(uuid: Uuid, object_type: String, tag: Option<String>, ix: usize) -> Self {
        Self {
            uuid,
            object_type,
            tag,
            ix,
        }
    }
    pub fn get_key(&self) -> (Uuid, String) {
        (self.uuid.clone(), self.object_type.clone())
    }
    pub fn compute_relative_path(ancestors: &Vec<AncestorRecord>) -> String {
        let mut relative_path: String = "".to_string();
        for record in ancestors.iter().rev() {
            if let Some(ref t) = record.tag {
                relative_path = format!("{}__{}", relative_path, t);
                break;
            }
            if record.ix > 0 {
                relative_path = format!(
                    "{}__{}_{}",
                    relative_path,
                    to_snake_case(&record.object_type),
                    record.ix
                );
            }
        }
        relative_path
    }
}

