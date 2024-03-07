use serde::Serialize;
// use surrealdb::opt::auth::Scope;

#[derive(Debug, Serialize)]
pub struct Credentials<'a> {
    botname: &'a str,
    secret: &'a str,
}

// pub fn get_scope<'a>(
//     namespace: &'a str,
//     database: &'a str,
//     scope: &'a str,
//     params: Credentials<'a>,
// ) -> Scope<'a, Credentials<'a>> {
//     Scope {
//         namespace,
//         database,
//         scope,
//         params: Credentials {
//             botname: params.botname,
//             secret: params.secret,
//         }
//     }
// }
