#[derive(Debug, Serializable, Deserializable)]
pub struct HubStatus {
    in_custom_hub: bool,
    in_custom_hub_name: String,
    in_custom_hub_truncated: String,
    in_custom_hub_icon_name: String,
    gate_key_version: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct Player {
    name: String,
    character_name: String,
    texture_body: i32,
    accessory1: i32,
    accessory1_texture: i32,
    accessory2: i32,
    accessory2_texture: i32,
    size: Scale<f32, 3>,
    voice: i32,
    pitch: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct InventoryItem {
    item: i32,
    id: i32,
    texture: i32,
    subtext: String,
    help_text: String
}

#[derive(Debug, Serializable, Deserializable)]
pub struct Inventory {
    size: i32,
    number_of_inventory_items: i32,
    items: [InventoryItem; 100],
    coins: i32,
    coins_collected: i32,
    gems: i32,
    stars: i32,
    score: i32,
}

impl Inventory {
    fn decrypt_field(arg: i32) -> i32 {
        todo!()
    }

    fn decrypt_field(arg: i32) -> i32 {
        ((arg - 11) / 5).sqrt() - 3
    }

    pub fn encrypt(&mut self) {
        todo!()
    }

    pub fn decrypt(&mut self) {
        self.coins = self.decrypt_field(self.coins);
        self.stars = self.decrypt_field(self.stars);
    }
}

#[derive(Debug, Serializable, Deserializable)]
pub struct Widescreen {
    fit_for_widescreen_global: i32,
    fit_for_widescreen_global_hub: i32,
    future: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct Command {
    id: i32,
    data1: i32,
    data2: i32,
    data3: i32,
    data4: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct AdventureWonCommand {
    level: i32,
    command: Command
}

#[derive(Debug, Serializable, Deserializable)]
pub struct AdventureExit {
    won_level: i32,
    won_pos: Pos<i32, 2>,
    lost_level: i32,
    lost_pos: Pos<i32, 2>,
    goal: i32,
    won_commands: [AdventureWonCommand; 3]
}

#[derive(Debug, Serializable, Deserializable)]
pub struct CurrentAdventure {
    gems: i32,
    coins: i32,
    time: i32,
    score: i32,
    adventure_timer: i32,
    level: i32,
    status: i32,
    number: i32,
    name: String,
    exit: AdventureExit,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct AdventureCompletion {
    completed: i32,
    time: i32,
    gems: i32,
    gems_total: i32,
    coins: i32,
    coins_total: i32,
    score: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GlobalData {
    player_object: i32,
    stinker_object: i32,
    camera_focus_object: i32,
    level_timer: i32,
    player_control_mode: i32,
    last_player_control: i32,
    player_talk_to_goal_object: i32,
    game_mode: i32,
    old_game_mode: i32,
    move_cursor_new_target: i32,
    mouse_held: i32,
    delay_command: Command,
    spell_active: i32,
    current_spell: i32,
    current_charm: i32,
    current_spell_power: i32,
    current_light_power: i32,
    used_inventory_once: i32,
    global_grow_flower_counter: i32,
    global_floing_bubble_counter: i32,
    player_lava_timer: i32,
    indigo_active: i32,
    future: [i32; 4],
    wa3_blue_flower: i32,
    wa3_blue_flower_status: i32,
    custom_map_name: String
}

#[derive(Debug, Serializable, Deserializable)]
pub struct CurrentAdventureCounts {
    wee_stinkers: i32,
    wee_stinkers_following: i32,
    wee_stinkers_following_last: i32,
    scritters: i32,
    gems: i32,
    bricks: i32,
    fireflowers: i32,
    crabs: i32,
    baby_boomers: i32,
    z_bots: i32,
    future: [i32; 7]
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplayAdventure {
    current: i32,
    pre_level: i32,
    pre_pos: Pos<i32, 2>
}

#[derive(Debug, Serializable, Deserializable)]
pub struct LevelMusic {
    volume: i32,
    pitch: i32
}

#[derive(Debug, Serializable, Deserializable)]
pub struct PlayerSave {
    hub_status: HubStatus,
    player: Player,
    inventory: Inventory,
    shards_are_active: bool,
    widescreen: Widescreen,
    inventory_b4: Inventory,
    light_power_b4: i32,
    current_adventure: CurrentAdventure,
    adventure_completions: [AdventureCompletion; 500],
    counts: CurrentAdventureCounts,
    global_data: GlobalData,
    future_strings: [String; 4],
    future_floats: [f32; 10],
    master_askabout_active: [bool; 1000],
    map_piece_found: [bool; 8],
    mystery_number: [i32; 4],
    mystery_number_pos: i32,
    replay_adventure: ReplayAdventure,
    level_music_custom: LevelMusic,
    custom_shard_enabled: bool,
    custom_glyph_enabled: bool,
    // TODO: more
}

impl Player {
    pub fn create_player_dirs() {
        todo!();
    }

    pub fn new(player_name: String) {

    }
}

pub fn create_new_player(player_name: &str) {
    
}