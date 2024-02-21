use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Array(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Object(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Range(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct String(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ULID(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct UUID(String);
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RAND(String);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SurrealString(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SurrealID {
    // Default,
    Int(i64),
    Float(f64),
    String(SurrealString),
    // Object(Object),
    // Array(Array),
    // UUID,
    // ULID,
    // RAND,
    // Range(Range),
}

impl SurrealID {
    // pub fn default()->Self{SurrealID::Default}
    pub fn int(num:i64)->Self{SurrealID::Int(num)}
    pub fn float(num:f64)->Self{SurrealID::Float(num)}
    pub fn string(s:&str)->Self{SurrealID::String(s.to_string())}
    // pub fn uuid()->Self{SurrealID::UUID}
    // pub fn ulid()->Self{SurrealID::ULID}
    // pub fn rand()->Self{SurrealID::RAND}
    // pub fn array(arr:Array)->Self{SurrealID::Array(arr)}
    // pub fn object(obj:Object)->Self{SurrealID::Object(obj)}
    // pub fn range(range:Range)->Self{SurrealID::Range(range)}
}

impl Default for SurrealID {
    fn default() -> Self {
        SurrealID::Default
    }
}

// impl ParamCombine for SurrealID {
//     fn combine(&self) -> String {
//         self.to_string()
//     }
// }

impl Display for SurrealID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            // SurrealID::Default => String::new(),
            SurrealID::Int(int) => int.to_string(),
            SurrealID::Float(f) => f.to_string(),
            SurrealID::String(s) => SurrealString(String::from(s)),
            // SurrealID::Object(obj) => obj.parse(),
            // SurrealID::Array(arr) => arr.parse(),
            // SurrealID::ULID => ULID.to_string(),
            // SurrealID::UUID => UUID.to_string(),
            // SurrealID::RAND => RAND.to_string(),
            // SurrealID::Range(range) => range.to_string()
        };
        write!(f, "{}", res)
    }
}

impl From<i64> for SurrealID {
    fn from(value: i64) -> Self {
        SurrealID::Int(value)
    }
}

impl From<i32> for SurrealID {
    fn from(value: i32) -> Self {
        SurrealID::Int(value as i64)
    }
}

impl From<f32> for SurrealID {
    fn from(value: f32) -> Self {
        SurrealID::Float(value as f64)
    }
}

impl From<f64> for SurrealID {
    fn from(value: f64) -> Self {
        SurrealID::Float(value)
    }
}

impl From<&str> for SurrealID {
    fn from(value: &str) -> Self {
        match value {
            // "" => SurrealID::Default,
            // RAND => SurrealID::RAND,
            // UUID => SurrealID::UUID,
            // ULID => SurrealID::ULID,
            other => SurrealID::String(String::from(other)),
            _ => SurrealID::String(String::from(value))
        }
    }
}

impl From<String> for SurrealID {
    fn from(value: String) -> Self {
        SurrealID::from(value.as_str())
    }
}

// impl From<Array> for SurrealID {
//     fn from(value: Array) -> Self {
//         SurrealID::Array(value)
//     }
// }

// impl From<Object> for SurrealID {
//     fn from(value: Object) -> Self {
//         SurrealID::Object(value)
//     }
// }

// impl From<Range> for SurrealID {
//     fn from(value: Range) -> Self {
//         SurrealID::Range(value)
//     }
// }
#[derive(Clone, Debug)]
pub enum SurrealIDType {
    // Default,
    Int,
    Float,
    Decimal,
    // UUID,ULID,RAND is Str too
    // Str,
    // Object,
    // Array,
    // Range,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ids() -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
// //! # SurrealID
// //! The ID or field ID used to build a virtualdb table. In fact, it is mainly used to create the table ID.
// //! ```txt
// //! @author:syf20020816@Outlook.com
// //! @date:2023/10/21
// //! @version:0.3.0
// //! @description:
// //! ```

// use std::fmt::{Display, Formatter};
// use serde::{Serialize, Deserialize};
// // use super::constants::{UUID, ULID, RAND, EQ};
// // use super::{SurrealValue, Object, Array, ParamCombine};

// ///# Enumeration type of ID
// ///Quickly generate a table ID or field ID containing a type through SurrealID
// ///It should be noted that there are actually no related rand, uuid, and ulid methods for the field. If you need to use these methods, you can use the external package method.
// ///-used on tables: `table_name:table_id`
// ///-used on fields: `User{ userId : rand_id}`
// ///
// /// ## example
// /// ``` rust
// /// use surrealism::DefaultRes;
// /// use surrealism::db::{Range, SurrealID, Array, SurrealValue, Object};
// /// use serde::{Serialize,Deserialize};
// ///
// /// #[derive(Debug,Serialize,Deserialize)]
// /// struct User<'a>{
// ///     name: &'a str,
// ///     age: u8,
// /// }
// /// #[tokio::main]
// /// async fn main() -> DefaultRes<()> {
// ///     let id1 = SurrealID::RAND;
// ///     let id2 = SurrealID::Default;
// ///     let id3 = SurrealID::from("surrealism");
// ///     let id4 = SurrealID::Int(56_i64);
// ///     let id5 = SurrealID::Float(45.5454647_f64);
// ///     let id6 = SurrealID::Array(Array::from(vec!["John".into(), "Matt".into()]));
// ///     let user = User {
// ///         name: "Mat",
// ///         age: 16,
// ///     };
// ///     let id7 = SurrealID::Object(Object::from_obj(&user));
// ///     let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
// ///     let id9 = SurrealID::from("ulid()");
// ///     dbg!(id1.to_string());
// ///     dbg!(id2.to_string());
// ///     dbg!(id3.to_string());
// ///     dbg!(id4.to_string());
// ///     dbg!(id5.to_string());
// ///     dbg!(id6.to_string());
// ///     dbg!(id7.to_string());
// ///     dbg!(id8.to_string());
// ///     dbg!(id9.to_string());
// ///     Ok(())
// /// }
// /// ```
// impl SurrealID {
//     pub fn default()->Self{SurrealID::Default}
//     pub fn int(num:i64)->Self{SurrealID::Int(num)}
//     pub fn float(num:f64)->Self{SurrealID::Float(num)}
//     pub fn string(s:&str)->Self{SurrealID::String(s.to_string())}
//     pub fn uuid()->Self{SurrealID::UUID}
//     pub fn ulid()->Self{SurrealID::ULID}
//     pub fn rand()->Self{SurrealID::RAND}
//     pub fn array(arr:Array)->Self{SurrealID::Array(arr)}
//     pub fn object(obj:Object)->Self{SurrealID::Object(obj)}
//     pub fn range(range:Range)->Self{SurrealID::Range(range)}
// }

// impl Default for SurrealID {
//     fn default() -> Self {
//         SurrealID::Default
//     }
// }

// impl ParamCombine for SurrealID {
//     fn combine(&self) -> String {
//         self.to_string()
//     }
// }

// impl Display for SurrealID {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let res = match self {
//             SurrealID::Default => String::new(),
//             SurrealID::Int(int) => int.to_string(),
//             SurrealID::Float(f) => f.to_string(),
//             SurrealID::String(s) => String::from(s),
//             SurrealID::Object(obj) => obj.parse(),
//             SurrealID::Array(arr) => arr.parse(),
//             SurrealID::ULID => ULID.to_string(),
//             SurrealID::UUID => UUID.to_string(),
//             SurrealID::RAND => RAND.to_string(),
//             SurrealID::Range(range) => range.to_string()
//         };
//         write!(f, "{}", res)
//     }
// }

// impl From<i64> for SurrealID {
//     fn from(value: i64) -> Self {
//         SurrealID::Int(value)
//     }
// }

// impl From<i32> for SurrealID {
//     fn from(value: i32) -> Self {
//         SurrealID::Int(value as i64)
//     }
// }

// impl From<f32> for SurrealID {
//     fn from(value: f32) -> Self {
//         SurrealID::Float(value as f64)
//     }
// }

// impl From<f64> for SurrealID {
//     fn from(value: f64) -> Self {
//         SurrealID::Float(value)
//     }
// }

// impl From<&str> for SurrealID {
//     fn from(value: &str) -> Self {
//         match value {
//             "" => SurrealID::Default,
//             RAND => SurrealID::RAND,
//             UUID => SurrealID::UUID,
//             ULID => SurrealID::ULID,
//             other => SurrealID::String(String::from(other))
//         }
//     }
// }

// impl From<String> for SurrealID {
//     fn from(value: String) -> Self {
//         SurrealID::from(value.as_str())
//     }
// }

// impl From<Array> for SurrealID {
//     fn from(value: Array) -> Self {
//         SurrealID::Array(value)
//     }
// }

// impl From<Object> for SurrealID {
//     fn from(value: Object) -> Self {
//         SurrealID::Object(value)
//     }
// }

// impl From<Range> for SurrealID {
//     fn from(value: Range) -> Self {
//         SurrealID::Range(value)
//     }
// }


// /// # Range
// /// 这是一种SurrealDB的ID类型，范围类型
// /// ## example
// /// ```rust
// /// use surrealism::DefaultRes;
// /// use surrealism::db::{Range};
// ///
// /// //[tests\src\main.rs:10] range1.to_string() = "2..=8"
// /// // [tests\src\main.rs:11] range2.to_string() = "2.7..5.6"
// /// // [tests\src\main.rs:12] range3.to_string() = "4..89.5"
// /// #[tokio::main]
// /// async fn main() -> DefaultRes<()> {
// ///     let range1 = Range::new(2.into(),8.into(),true);
// ///     let range2 = Range::new(2.7.into(),5.6.into(),false);
// ///     let range3  = Range::new_from_str("4","89.5",false);
// ///     dbg!(range1.to_string());
// ///     dbg!(range2.to_string());
// ///     dbg!(range3.to_string());
// ///     Ok(())
// /// }
// /// ```
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct Range {
//     eq: bool,
//     beg: SurrealValue,
//     end: SurrealValue,
// }

// impl Range {
//     pub fn new(beg: SurrealValue, end: SurrealValue, is_eq: bool) -> Self {
//         Range {
//             eq: is_eq,
//             beg,
//             end,
//         }
//     }
//     pub fn new_from_str(beg: &str, end: &str, is_eq: bool) -> Self {
//         Range {
//             eq: is_eq,
//             beg: beg.into(),
//             end: end.into(),
//         }
//     }
// }

// impl Display for Range {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let res = if self.eq {
//             format!("{}..{}{}", self.beg.to_string(), EQ, self.end.to_string())
//         } else {
//             format!("{}..{}", self.beg.to_string(), self.end.to_string())
//         };
//         write!(f, "{}", res)
//     }
// }

// #[derive(Clone, Debug)]
// pub enum SurrealIDType {
//     Default,
//     Int,
//     Float,
//     Decimal,
//     /// UUID,ULID,RAND is Str too
//     Str,
//     Object,
//     Array,
//     Range,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_ids() -> Result<(), std::fmt::Error> {
//         Ok(())
//     }
// }