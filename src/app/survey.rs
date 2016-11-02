
pub struct Survey {
    pub id:i32,
    pub title: String,
    pub duration: i32,
    pub active: bool,
    pub start_date: String,
}

impl Survey {
    // return first active survey
    pub fn active(storage:StorageProvider) -> Survey {
        storage.select(vec!["surveys"])
               .cond(vec!["active=1"])
               .and(
               .first()
    }
}

