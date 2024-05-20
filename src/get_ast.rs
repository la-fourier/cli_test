use clap::{
    Args,
    delegater,
    Subcommand,
};

pub trait Delegate {
    fn delegate(self);
}

#[derive(Debug, Args)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    // /// First arg
    // pub first_arg: String,
    // /// Sec arg
    // pub sec_arg: String,
    // /// 3rd arg
    // pub third_arg: String,

    #[clap(subcommand)]
    pub entity_type: EntityType,
}

impl Delegate for RustflixArgs {
    fn delegate(self) {
        self.entity_type.delegate();
    }
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    User(UserCommand),
    Video(VideoCommand),
    View(ViewCommand),
}

#[print_ast]
impl Delegate for EntityType {
    fn delegate(self) {
        match self {
            EntityType::User(user_command) => user_command.delegate(),
            EntityType::Video(video_command) => video_command.delegate(),
            EntityType::View(view_command) => view_command.delegate(),
        }
    }
}

#[print_ast]
#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

impl Delegate for UserCommand {
    fn delegate(self) {
        self.subcommand.delegate();
    }
}

#[derive(Debug, Args)]
pub struct VideoCommand {
}

impl Delegate for VideoCommand {
    fn delegate(self) {
        println!("Video command");
    }
}

#[derive(Debug, Args)]
pub struct ViewCommand {
}

impl Delegate for ViewCommand {
    fn delegate(self) {
        println!("View command");
    }
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
    Read(ReadUser),
}

impl Delegate for UserSubcommand {
    fn delegate(self) {
        match self {
            UserSubcommand::Create(create_user) => create_user.delegate(),
            UserSubcommand::Update(update_user) => update_user.delegate(),
            UserSubcommand::Delete(delete_user) => delete_user.delegate(),
            UserSubcommand::Read(read_user) => read_user.delegate(),
        }
    }
}

#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

impl Delegate for CreateUser {
    fn delegate(self) {
        println!("Create user {:?} with mail {:?}", self.name, self.email);
    }
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

impl Delegate for UpdateUser {
    fn delegate(self) {
        println!("Update user {:?} with mail {:?}", self.name, self.email);
    }
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub name: String,
    pub email: String,
}

impl Delegate for DeleteUser {
    fn delegate(self) {
        println!("Delete user {:?} with mail {:?}", self.name, self.email);
    }
}

#[derive(Debug, Args)]
pub struct ReadUser {
    pub name: String,
    pub email: String,
}

impl Delegate for ReadUser {
    fn delegate(self) {
        println!("Read user {:?} with mail {:?}", self.name, self.email);
    }
}