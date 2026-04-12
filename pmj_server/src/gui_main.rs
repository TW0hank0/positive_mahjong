use iced;

use pmj_server_lib::{self, gamemodes};
use pmj_shared;

fn main() -> iced::Result {
    let config = pmj_server_lib::shared::read_server_config();
    match config.gamemode {
        pmj_shared::shared::GameModes::Base => {
            gamemodes::gui_base::main()?;
        }
        pmj_shared::shared::GameModes::V1Simple => {
            println!("還未支援！");
        }
        pmj_shared::shared::GameModes::V2Better => {
            println!("還未支援！");
        }
    }
    Ok(())
}
