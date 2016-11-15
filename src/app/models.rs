use lib::storageprovider::{StorageProvider, FieldType, DataTraits};

model!( struct Survey {
                    title: String,
                    duration: i32,
                    start_date: i64,
                    active: bool }, "surveys");

