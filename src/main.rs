pub mod cmd;
pub use cmd::*;
pub mod dialog;
pub use dialog::*;
pub mod object;
pub use object::*;
pub mod serde_blitz3d;
pub mod tile_logic;
pub use tile_logic::*;

fn main() {
    make_funny_password_dia();

    //make_item_maze_check_dia();

    //make_mechanism_piece_wop();

    //let spellball = Object::from_wlv("13.wlv", 0x16504).unwrap();
    //println!("{:#?}", spellball);

    //let glovecharge = Object::from_wop("GloveCharge.wop").unwrap();
    //println!("{:#?}", glovecharge);
}

pub fn make_spellball_wop() {
    let mut spellball = Object::from_wlv("SaveLevel.wlv", 0x3889A).unwrap();
    spellball.set_logic(ObjectLogic::spellball(1));
    spellball.set_model_name("!None".to_owned());
    spellball.set_texture_name("!None".to_owned());
    spellball.set_tile_pos(0, 0);
    spellball.set_2d_pos(0.0, 0.0);
    spellball.set_2d_delta(0.0, 0.0);
    spellball.set_timer(0);
    spellball.set_status(0);
    spellball.zero_data();
    spellball.set_data(0, -1);
    spellball.set_data(1, -1);
    spellball.set_object_type_collision(ObjectTypeCollision::NONE);
    spellball.to_wop("Spellball").unwrap();

    println!("{:#?}", spellball);
}

pub fn make_mechanism_piece_wop() {
    let mut obj = Object::new("!None".to_owned(), "!None".to_owned(), ObjectLogic::NONE);
    obj.set_tile_type_collision(TileTypeCollision::all_floors_and_06_07());
    obj.set_object_type_collision(ObjectTypeCollision::NONE);
    obj.set_movement_type(MovementType::NORTH_LEFT);
    obj.set_movement_speed(1000);
    obj.set_button_push(1);
    obj.to_wop("Mechanism Piece").unwrap();
}

pub fn make_item_maze_check_dia() {
    let mut dialog = Dialog::default();
    dialog.add_empty_interchange();
    dialog.add_empty_interchange();
    let interchange_item_check = Interchange::new(
        Body::one_liner(String::from("{} LEFT...")),
        vec![Reply::consume_item(1, String::from("(OK)"), Cmd::none())]
    );
    let interchange_item_missing = Interchange::new(
        Body::one_liner(String::from("...")),
        vec![Reply::continue_to(1, String::from("What?"), Cmd::none())]
    );
    dialog.add_item_consumption_sequence(22, interchange_item_check, interchange_item_missing);
    dialog.to_dia("1").unwrap();
}

pub fn make_funny_password_dia() {
    let mut dialog = Dialog::default();
    dialog.add_empty_interchange();
    dialog.add_password_entry_sequence(
        Body::one_liner(String::from("...")),
        &["1", "2", "3", "4", "5", "6"],
        &[2, 1, 0, 1, 2]);
    dialog.to_dia("1").unwrap();
}