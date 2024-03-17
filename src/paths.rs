pub const ICON: &str = "resources/icon/icon_32.png";

pub mod loading {
    pub const ICON: &str = "resources/loading/loading_512.png";
}

pub mod main_menu {
    pub const PRLX_TREES_FAR: &str = "resources/main_menu/parallax/woods-far-trees.png";
    pub const PRLX_TREES_MID: &str = "resources/main_menu/parallax/woods-mid-trees.png";
    pub const PRLX_TREES_CLO: &str = "resources/main_menu/parallax/woods-close-trees.png";
    pub const LOGO: &str = "resources/main_menu/LOGO_WHITE.png";
}

pub mod player {
    pub mod basic {
        pub const IDLE: &str = "resources/player/basic/idle_48x48.png";
        pub const RUN: &str = "resources/player/basic/run_48x48.png";
        pub const JUMP: &str = "resources/player/basic/jump_48x48.png";
        pub const FALL: &str = "resources/player/basic/jump_48x48.png";
        pub const CROUCH_IDLE: &str = "resources/player/basic/crouch_idle_48x48.png";
        pub const CROUCH_WALK: &str = "resources/player/basic/crouch_walk_48x48.png";
    }

    pub mod advn {
        pub const ATLAS: &str = "resources/player/adventurer/SpriteSheetOrdered.png";
        pub const IDLE: &str = "resources/player/adventurer/idle.png";
        pub const WALK: &str = "resources/player/adventurer/walk.png";
        pub const RUN: &str = "resources/player/adventurer/run.png";
        pub const JUMP: &str = "resources/player/adventurer/jump.png";
        pub const FALL: &str = "resources/player/adventurer/fall.png";
        pub const CRID: &str = "resources/player/adventurer/crouch_idle.png";
        pub const CRWK: &str = "resources/player/adventurer/crouch_walk.png";
        pub const WSLD: &str = "resources/player/adventurer/wall_slide.png";
    }
}

pub mod world {
    pub const TILESET: &str = "resources/tiles/mainlev_build.png";
}

pub const MAPPING: &str = "resources/glfw/gamecontrollerdb.txt";
