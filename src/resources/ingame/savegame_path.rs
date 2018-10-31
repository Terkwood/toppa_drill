use std::{
    fs,
    path::*,
};

pub struct SavegamePaths {
    pub savegame_dir_path: PathBuf,
    pub planet_file_path: PathBuf,
    pub chunk_dir_path: PathBuf,
}

impl SavegamePaths {
    pub fn init(base_path: &'static str, game_name: &'static str) -> SavegamePaths {
        // Directory of all savegames
        let dir_path = Path::new("savegames");

        // Directory of this savegame
        let mut savegame_dir_path = PathBuf::from(base_path);
        savegame_dir_path.push(dir_path);
        savegame_dir_path.push(Path::new(game_name));
        #[cfg(feature = "debug")]
        debug!("savegame_dir_path: {:?}", savegame_dir_path.clone());

        // Filepath for the serialized planet
        let mut planet_file_path = PathBuf::new();
        planet_file_path.push(savegame_dir_path.clone());
        planet_file_path.push(Path::new("session_data"));
        planet_file_path.set_extension("ron");

        // Directory-path for the serialized chunks, need to append the individual chunks Id
        let mut chunk_dir_path = PathBuf::new();
        chunk_dir_path.push(savegame_dir_path.clone());
        chunk_dir_path.push(Path::new("chunks"));

        // NOTE: Maybe replace all these file operations with walk_dir crate?
        let mut dir_exists = dir_path.is_dir();
        if !dir_exists {
            if let Ok(_) = fs::create_dir(dir_path) {
                #[cfg(feature = "debug")]
                debug!("Savegame dir has been created at {:?}.", dir_path);
            } else {
                error!("Failed to create savegame dir at {:?}", dir_path);
            }
        }

        dir_exists = savegame_dir_path.exists();
        if dir_exists {
            #[cfg(feature = "debug")]
            debug!("Overwriting old savegame: {:?}.", game_name);
            for entry_result in fs::read_dir(savegame_dir_path.clone()).unwrap() {
                if let Ok(entry) = entry_result {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        for sub_entry_res in fs::read_dir(entry_path.clone()).unwrap() {
                            if let Ok(sub_entry) = sub_entry_res {
                                if sub_entry.path().is_file() {
                                    if let Err(e) = fs::remove_file(sub_entry.path()) {
                                        error!(
                                            "Error removing file '{:?}': {:?}",
                                            sub_entry.path(),
                                            e
                                        );
                                    }
                                } else {
                                    error!("Found unexpected directory inside the savegame's chunk directory!");
                                }
                            }
                        }
                    } else if entry_path.is_file() {
                        if let Err(e) = fs::remove_file(entry_path.clone()) {
                            error!("Error removing file '{:?}': {:?}", entry_path, e);
                        }
                    } else {
                        error!(
                            "Error removing dir '{:?}' entry '{:?}!",
                            savegame_dir_path.clone(),
                            entry_path
                        );
                    }
                } else {
                    error!("Error reading dir '{:?}' entry!", savegame_dir_path.clone());
                }
            }
        } else {
            if let Ok(_) = fs::create_dir_all(chunk_dir_path.clone()) {
            } else {
                error!(
                    "Failed to create savegame '{:?}'s dir at {:?}",
                    game_name,
                    savegame_dir_path.clone()
                );
            }
        }

        SavegamePaths {
            savegame_dir_path,
            planet_file_path,
            chunk_dir_path,
        }
    }
}
