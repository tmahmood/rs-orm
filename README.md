# rs-orm
A ORM system for Rust with support for Postgres, MySQL, SQLite. [WIP]


NOTE: It's not meant to be anything serious, [Diesel](http://diesel.rs/guides/getting-started/) is way to go if you want something real. This was just a fun project. 

Here's a example code, how it supposed to work:

# Defining Models

```rust
    model!( struct Survey {
                    title: String,
                    duration: i32,
                    start_date: i64,
                    active: bool
                }, "surveys");

    model!( struct QuestionSet {
                    title: String
                }, "questionsets");
```

# Connecting and Creating
```rust
    let storage = connect();
    // clean table
    let _ = storage.clear(Survey::name());
    let _ = storage.clear(QuestionSet::name());

    // create new entry
    let time = Local::now().timestamp();
    let mut survey = Survey::new( format!("Oct 2016 Survey"), 8, time, true);
    assert_eq!(true, storage.insert(&mut survey));
    
    // check if id was set
    assert!(survey.id > 0);
```

# Loading and updating
```rust
    // loading, check if all value matches
    let mut new_one:Survey = storage.find(Survey::name(), survey.id);
    // check if all are set correctly
    assert_eq!(new_one.title, "Oct 2016 Survey");
    assert_eq!(new_one.duration, 8);
    assert_eq!(new_one.active, true);
    assert_eq!(new_one.start_date, time);
    // update data
    new_one.title(String::from("Nov 2016 Survey"))
           .duration(6)
           .active(false);
    assert_eq!(true, storage.update(&new_one));
```

The above codes works, but seems postgresql is broken right now. You are free to fork and do anything you want.
