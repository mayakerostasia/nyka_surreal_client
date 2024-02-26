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
    name: String,
    age: u8,
}

// Then Implement the `Storable` trait for the object
// The `Storable` trait requires the `table` and `id` methods to be implemented
// The `table` method returns the name of the table in the database
// The `id` method returns the name of the id field in the database
// The `Item` associated type is the type of the object being stored
impl<'a> Storable<'a> for Person {}
impl StorableId for Person {
    fn id(&self) -> String {
        self.name.clone()
    }
}

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