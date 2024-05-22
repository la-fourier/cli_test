use command_rpc::crpc_main;

#[crpc_main]
pub mod rustflixArgs {

    use command_rpc::{crpc_fn, crpc_mod};

    // Test
    #[crpc_fn]
    /// Testkommenatr
    pub fn videoCommand(
        /// sdfj
        name: String,
        /// This is mail..
        email: Option<String>,
        chapters: Vec<i32>) {
        print!("Hello {}, your email is {:?}", name, email);
    }

    #[crpc_fn]
    pub fn viewCommand(test: i32) {
        print!("Hey, you are viewing {}", test);
    }

    #[crpc_fn]
    pub fn userCommand() {
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


fn main() {
    RustflixArgs::parse().delegate();
}
