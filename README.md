*Surreal Client, by Nico*

# Env Required
- SRQL_PATH
- SRQL_NS
- SRQL_DB
- SRQL_USER
- SRQL_PASS

SRQL_PATH is the path to the database. It can be a file path or a URL.  
&nbsp;&nbsp;&nbsp;&nbsp;*note* : If using file path... prepend the path with some file descriptor like, but perhaps not exactly, `file://`  [More Info](https://docs.rs/surrealdb/latest/surrealdb/engine/any/index.html)  
SRQL_NS is the namespace of the database.  
SRQL_DB is the name of the database.  
SRQL_USER is the username of the database.  
SRQL_PASS is the password of the database.  

# Example
[Here](./examples/person.rs) is an example of how to use the library.
```rust
// First Define the structure of the Object
// Ensure to derive the Serialize and Deserialize traits
// `Clone` is only necessary for the example

// Definition
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: SurrealId,
    name: String,
    age: u8,
}

impl DBThings for Person {}

impl Storable<Person> for Person {
    fn thing(&self) -> Thing {
        Thing::from((self.table().unwrap(), self.id().unwrap()))
    }
    fn id(&self) -> Option<Id> {
        Some(Id::Number(1))
    }

    fn table(&self) -> Option<String> {
        Some(TEST_TABLE.to_string())
    }

    fn data(&self) -> Person {
        self.clone()
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



