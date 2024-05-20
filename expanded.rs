#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use command_rpc::{crpc_main, print_ast};
use clap::{Args, Parser, Subcommand};
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub subcommand: SubcommandRustflixArgs,
}
#[automatically_derived]
impl ::core::fmt::Debug for RustflixArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "RustflixArgs",
            "subcommand",
            &&self.subcommand,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for RustflixArgs {
    #[inline]
    fn clone(&self) -> RustflixArgs {
        RustflixArgs {
            subcommand: ::core::clone::Clone::clone(&self.subcommand),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for RustflixArgs {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for RustflixArgs {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("cli_test");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("cli_test");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for RustflixArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = RustflixArgs {
            subcommand: {
                <SubcommandRustflixArgs as clap::FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        {
            #[allow(non_snake_case)]
            let subcommand = &mut self.subcommand;
            <SubcommandRustflixArgs as clap::FromArgMatches>::update_from_arg_matches_mut(
                subcommand,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for RustflixArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("RustflixArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("RustflixArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <SubcommandRustflixArgs as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true);
            __clap_app.version("0.1.0")
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("RustflixArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <SubcommandRustflixArgs as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app.version("0.1.0")
        }
    }
}
impl RustflixArgs {
    pub fn delegate(self) {
        self.subcommand.delegate();
    }
}
pub enum SubcommandRustflixArgs {
    SubcommandVideoCommand(rustflixArgs::VideoCommand),
    SubcommandViewCommand(rustflixArgs::ViewCommand),
    SubcommandUserCommand(rustflixArgs::UserCommand),
}
#[automatically_derived]
impl ::core::fmt::Debug for SubcommandRustflixArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SubcommandRustflixArgs::SubcommandVideoCommand(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SubcommandVideoCommand",
                    &__self_0,
                )
            }
            SubcommandRustflixArgs::SubcommandViewCommand(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SubcommandViewCommand",
                    &__self_0,
                )
            }
            SubcommandRustflixArgs::SubcommandUserCommand(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SubcommandUserCommand",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubcommandRustflixArgs {
    #[inline]
    fn clone(&self) -> SubcommandRustflixArgs {
        match self {
            SubcommandRustflixArgs::SubcommandVideoCommand(__self_0) => {
                SubcommandRustflixArgs::SubcommandVideoCommand(
                    ::core::clone::Clone::clone(__self_0),
                )
            }
            SubcommandRustflixArgs::SubcommandViewCommand(__self_0) => {
                SubcommandRustflixArgs::SubcommandViewCommand(
                    ::core::clone::Clone::clone(__self_0),
                )
            }
            SubcommandRustflixArgs::SubcommandUserCommand(__self_0) => {
                SubcommandRustflixArgs::SubcommandUserCommand(
                    ::core::clone::Clone::clone(__self_0),
                )
            }
        }
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for SubcommandRustflixArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
            .remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "subcommand-video-command"
                && !__clap_arg_matches.contains_id("")
            {
                return ::std::result::Result::Ok(
                    Self::SubcommandVideoCommand(
                        <rustflixArgs::VideoCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            if __clap_name == "subcommand-view-command"
                && !__clap_arg_matches.contains_id("")
            {
                return ::std::result::Result::Ok(
                    Self::SubcommandViewCommand(
                        <rustflixArgs::ViewCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            if __clap_name == "subcommand-user-command"
                && !__clap_arg_matches.contains_id("")
            {
                return ::std::result::Result::Ok(
                    Self::SubcommandUserCommand(
                        <rustflixArgs::UserCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The subcommand \'{0}\' wasn\'t recognized",
                                __clap_name,
                            ),
                        );
                        res
                    },
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::SubcommandVideoCommand(
                    ref mut __clap_arg,
                ) if "subcommand-video-command" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                Self::SubcommandViewCommand(
                    ref mut __clap_arg,
                ) if "subcommand-view-command" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                Self::SubcommandUserCommand(
                    ref mut __clap_arg,
                ) if "subcommand-user-command" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Subcommand for SubcommandRustflixArgs {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-video-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::VideoCommand as clap::Args>::augment_args(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-view-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::ViewCommand as clap::Args>::augment_args(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-user-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::UserCommand as clap::Args>::augment_args(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-video-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::VideoCommand as clap::Args>::augment_args_for_update(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-view-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::ViewCommand as clap::Args>::augment_args_for_update(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("subcommand-user-command");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <rustflixArgs::UserCommand as clap::Args>::augment_args_for_update(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "subcommand-video-command" == __clap_name {
            return true;
        }
        if "subcommand-view-command" == __clap_name {
            return true;
        }
        if "subcommand-user-command" == __clap_name {
            return true;
        }
        false
    }
}
impl SubcommandRustflixArgs {
    pub fn delegate(self) {
        match self {
            SubcommandRustflixArgs::SubcommandVideoCommand(command) => command.delegate(),
            SubcommandRustflixArgs::SubcommandViewCommand(command) => command.delegate(),
            SubcommandRustflixArgs::SubcommandUserCommand(command) => command.delegate(),
        }
    }
}
pub mod rustflixArgs {
    use command_rpc::{crpc_mod, crpc_fn};
    use clap::{Args, Parser, Subcommand};
    #[clap(author, version, about)]
    pub struct UserCommand {
        #[clap(subcommand)]
        pub subcommand: SubcommandUserCommand,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UserCommand {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UserCommand",
                "subcommand",
                &&self.subcommand,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UserCommand {
        #[inline]
        fn clone(&self) -> UserCommand {
            UserCommand {
                subcommand: ::core::clone::Clone::clone(&self.subcommand),
            }
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for UserCommand {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = UserCommand {
                subcommand: {
                    <SubcommandUserCommand as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?
                },
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            {
                #[allow(non_snake_case)]
                let subcommand = &mut self.subcommand;
                <SubcommandUserCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                    subcommand,
                    __clap_arg_matches,
                )?;
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Args for UserCommand {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("UserCommand"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("UserCommand")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 0usize] = [];
                                members
                            }),
                    );
                let __clap_app = <SubcommandUserCommand as clap::Subcommand>::augment_subcommands(
                    __clap_app,
                );
                let __clap_app = __clap_app
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_app.version("0.1.0")
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("UserCommand")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 0usize] = [];
                                members
                            }),
                    );
                let __clap_app = <SubcommandUserCommand as clap::Subcommand>::augment_subcommands(
                    __clap_app,
                );
                let __clap_app = __clap_app
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .subcommand_required(false)
                    .arg_required_else_help(false);
                __clap_app.version("0.1.0")
            }
        }
    }
    impl UserCommand {
        pub fn delegate(self) {
            self.subcommand.delegate();
        }
    }
    pub enum SubcommandUserCommand {
        SubcommandCreateUser(userCommand::CreateUser),
        SubcommandUpdateUser(userCommand::UpdateUser),
        SubcommandDeleteUser(userCommand::DeleteUser),
        SubcommandReadUser(userCommand::ReadUser),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SubcommandUserCommand {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SubcommandUserCommand::SubcommandCreateUser(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SubcommandCreateUser",
                        &__self_0,
                    )
                }
                SubcommandUserCommand::SubcommandUpdateUser(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SubcommandUpdateUser",
                        &__self_0,
                    )
                }
                SubcommandUserCommand::SubcommandDeleteUser(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SubcommandDeleteUser",
                        &__self_0,
                    )
                }
                SubcommandUserCommand::SubcommandReadUser(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SubcommandReadUser",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SubcommandUserCommand {
        #[inline]
        fn clone(&self) -> SubcommandUserCommand {
            match self {
                SubcommandUserCommand::SubcommandCreateUser(__self_0) => {
                    SubcommandUserCommand::SubcommandCreateUser(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                SubcommandUserCommand::SubcommandUpdateUser(__self_0) => {
                    SubcommandUserCommand::SubcommandUpdateUser(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                SubcommandUserCommand::SubcommandDeleteUser(__self_0) => {
                    SubcommandUserCommand::SubcommandDeleteUser(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                SubcommandUserCommand::SubcommandReadUser(__self_0) => {
                    SubcommandUserCommand::SubcommandReadUser(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for SubcommandUserCommand {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                .remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "subcommand-create-user"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(
                        Self::SubcommandCreateUser(
                            <userCommand::CreateUser as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "subcommand-update-user"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(
                        Self::SubcommandUpdateUser(
                            <userCommand::UpdateUser as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "subcommand-delete-user"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(
                        Self::SubcommandDeleteUser(
                            <userCommand::DeleteUser as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "subcommand-read-user"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(
                        Self::SubcommandReadUser(
                            <userCommand::ReadUser as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::InvalidSubcommand,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "The subcommand \'{0}\' wasn\'t recognized",
                                    __clap_name,
                                ),
                            );
                            res
                        },
                    ),
                )
            } else {
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::MissingSubcommand,
                        "A subcommand is required but one was not provided.",
                    ),
                )
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::SubcommandCreateUser(
                        ref mut __clap_arg,
                    ) if "subcommand-create-user" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::SubcommandUpdateUser(
                        ref mut __clap_arg,
                    ) if "subcommand-update-user" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::SubcommandDeleteUser(
                        ref mut __clap_arg,
                    ) if "subcommand-delete-user" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::SubcommandReadUser(
                        ref mut __clap_arg,
                    ) if "subcommand-read-user" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Subcommand for SubcommandUserCommand {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-create-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::CreateUser as clap::Args>::augment_args(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-update-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::UpdateUser as clap::Args>::augment_args(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-delete-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::DeleteUser as clap::Args>::augment_args(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-read-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::ReadUser as clap::Args>::augment_args(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(
            __clap_app: clap::Command,
        ) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-create-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::CreateUser as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-update-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::UpdateUser as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-delete-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::DeleteUser as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subcommand-read-user");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <userCommand::ReadUser as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "subcommand-create-user" == __clap_name {
                return true;
            }
            if "subcommand-update-user" == __clap_name {
                return true;
            }
            if "subcommand-delete-user" == __clap_name {
                return true;
            }
            if "subcommand-read-user" == __clap_name {
                return true;
            }
            false
        }
    }
    impl SubcommandUserCommand {
        pub fn delegate(self) {
            match self {
                SubcommandUserCommand::SubcommandCreateUser(command) => {
                    command.delegate()
                }
                SubcommandUserCommand::SubcommandUpdateUser(command) => {
                    command.delegate()
                }
                SubcommandUserCommand::SubcommandDeleteUser(command) => {
                    command.delegate()
                }
                SubcommandUserCommand::SubcommandReadUser(command) => command.delegate(),
            }
        }
    }
    pub mod userCommand {
        use command_rpc::crpc_fn;
    }
}
fn main() {
    ::core::panicking::panic("not yet implemented")
}
