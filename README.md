# Env Required
- DB_PATH
- DB_NS
- DB_DB
- DB_USER
- DB_SECRET

DB_PATH is the path to the database. It can be a file path or a URL.  
&nbsp;&nbsp;&nbsp;&nbsp;*note* : If using file path... prepend the path with some file descriptor like, but perhaps not exactly, `file://`  [More Info](https://docs.rs/surrealdb/latest/surrealdb/engine/any/index.html)  
DB_NS is the namespace of the database.  
DB_DB is the name of the database.  
DB_USER is the username of the database.  
DB_SECRET is the password of the database.  

# Example
[Here](./examples/person.rs) is an example of how to use the library.
```rust

// First Define the structure of the Object
// Ensure to derive the Serialize and Deserialize traits
// `Clone` is only necessary for the example

// Definition
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: SurrealID,
    name: String,
    age: u8,
}
/// Adds the .save() .select() .delete() methods to the object
impl Storable for Person {}

/// Debug + Serialize + DeserializeOwned + Sized + Clone
impl DBThings for Person {}

/// Indicates that the object has a SurrealID in the "id" field 
impl HasSurrealIdentifier for Person {}
/// Indicates that the object has data beside the id field
impl SurrealData for Person {}

impl SurrealIDIdent for Person {
    fn id(&self) -> String {
        self.id.id()
    }
}
impl SurrealIDTable for Person {
    fn table(&self) -> String {
        self.id.table()
    }
}
impl From<Record<Person>> for Person {
    fn from(record: Record<Person>) -> Self {
        let data = record.into_inner().unwrap();
        data
    }
}

// Then Implement the `Storable` trait for the object

fn main() -> Result<(), SurrealError> {
    // Record To Database
    let john = Person {
        name: "John".to_string(),
        age: 20,
    };
    let saved_john = john.clone().save().await?;
    let selected_john = john.clone().select().await?;
    let deleted_john = john.clone().delete().await?;
    Ok(())
}
```


Might be worth a look -> https://github.com/liamwh/surreal-id

