use command_rpc::{crpc_main, print_ast};

// Macro for parsing the input
// macro_rules! parse {
//     // For the main function, that automatically parses the input and calls the right function - you have to pass the path to the crpc_main module
//     ($inp:expr) => {
//         let mut path = $inp.split("::").collect();
//         let mut module = path.last();
//         module = module[0].uppercase() + module[1..].to_string();
//         path.push(&module);
//         let path = path.join("::");
//         path!($path).delegate();
//     };
// }

#[crpc_main]
pub mod rustflixArgs {

    use command_rpc::{crpc_mod, crpc_fn};

    #[crpc_fn]
    pub fn videoCommand() {
        todo!()
    }

    #[crpc_fn]
    pub fn viewCommand() {
        todo!()
    }

    #[crpc_mod] 
    pub mod userCommand {

        use command_rpc::crpc_fn;

        #[crpc_fn]
        pub fn createUser(name: String, email: String) {
            todo!()
        }

        #[crpc_fn]
        pub fn updateUser(name: String, email: String) {
            todo!()
        }

        #[crpc_fn]
        pub fn deleteUser(name: String, email: String) {
            todo!()
        }

        #[crpc_fn]
        pub fn readUser(name: String, email: String) {
            todo!()
        }
    }
}

// Callback
// fn from_other() {
//     callback!(my_cli_backend::...::greet("John"));
// }

// #[print_ast]
fn main() {
    todo!()
}