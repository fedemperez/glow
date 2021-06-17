use std::collections::BTreeMap;

use nalgebra::{Vector3, vector};

use crate::chunks::WorldView;
use crate::blocks::{Block, BlockClass};

pub fn get_stair_shape(props: &BTreeMap<String, String>, view: &WorldView)
    -> String
{
    let behind = dir_from_facing(&props["facing"]);
    let other = view.get(behind.x, behind.y, behind.z);
    if can_connect(&props, other) {
        if other.props["facing"] == get_right(&props["facing"]) {
            if get_side(props, view, false) {
                "straight"
            } else {
                "outer_right"
            }
        } else {
            if get_side(props, view, true) {
                "straight"
            } else {
                "outer_left"
            }
        }
    } else {
        let front = behind * -1;
        let other = view.get(front.x, front.y, front.z);
        if can_connect(&props, other) {
            if other.props["facing"] == get_right(&props["facing"]) {
                if get_side(props, view, true) {
                    "straight"
                } else {
                    "inner_right"
                }
            } else {
                if get_side(props, view, false) {
                    "straight"
                } else {
                    "inner_left"
                }
            }
        } else {
            "straight"
        }
    }.into()
}

fn dir_from_facing(facing: &str) -> Vector3<i32> {
    match facing {
        "south" => vector!(0, 0, 1),
        "west" => vector!(-1, 0, 0),
        "north" => vector!(0, 0, -1),
        _east => vector!(1, 0, 0),
    }.into()
}

fn get_right(facing: &str) -> String {
    match facing {
        "south" => "west",
        "west" => "north",
        "north" => "east",
        _east => "south",
    }.into()
}

fn get_side(props: &BTreeMap<String, String>, view: &WorldView, right: bool) 
    -> bool
{
    let dir_right = dir_from_facing(
        &get_right(&props["facing"]));
    let dir = if right { dir_right } else { dir_right * -1 };
    is_in_line(props, view.get(dir.x, dir.y, dir.z))
}

fn is_in_line(props: &BTreeMap<String, String>, with: &Block) -> bool {
    with.btype.class == BlockClass::StairsBlock && 
    with.props["half"] == props["half"] &&
    props["facing"] == with.props["facing"]
}

fn can_connect(props: &BTreeMap<String, String>, to: &Block)
    -> bool
{
    to.btype.class == BlockClass::StairsBlock && 
    to.props["half"] == props["half"] &&
    is_perpendicular(&props["facing"], &to.props["facing"])
}

fn is_perpendicular(a: &str, b: &str) -> bool {
    if a == "north" || a == "south" {
        b == "east" || b == "west"
    } else {
        b == "north" || b == "south"
    }
}
