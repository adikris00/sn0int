use crate::errors::*;
use crate::shell::Shell;

pub trait Cmd: structopt::StructOpt + Sized {
    fn run(self, rl: &mut Shell) -> Result<()>;

    #[inline]
    fn run_str(rl: &mut Shell, args: &[String]) -> Result<()> {
        let args = Self::from_iter_safe(args)?;
        args.run(rl)
    }
}

pub mod activity_cmd;
pub mod add_cmd;
pub mod autonoscope_cmd;
pub mod autoscope_cmd;
pub mod delete_cmd;
pub mod export_cmd;
pub mod fsck_cmd;
pub mod help_cmd;
pub mod run_cmd;
pub mod use_cmd;
pub mod select_cmd;
pub mod keyring_cmd;
pub mod mod_cmd;
pub mod noscope_cmd;
pub mod set_cmd;
pub mod scope_cmd;
pub mod target_cmd;
pub mod quickstart_cmd;
pub mod workspace_cmd;
