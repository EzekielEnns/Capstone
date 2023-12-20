#![allow(dead_code)]
//good reffrence
//https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/

#[derive(Clone, Copy)]
struct St {
    h: i32,
    s: i32,
}

//stats
impl St {
    const fn default() -> Self {
        St { h: 0, s: 0 }
    }
    fn rest(&mut self) {}
}
/*
game breaks down into a few phases
ATK
    sum up per target
    apply modifyer
DEF
ATK
DEF
RST
    itterate lifespan

let a, b: side
for {
    wait for input
    let stats,aSide = a.attack()
        //applies modifyers returns a status
    //set side a info
    //wait for input
    b.deffend(stats) -> side b
        //apply new stats to side b
    let stats,bSide b.attack()
    //set side b info
    a.deffend(stats) -> side a
        //apply new stats to side a
}

this wont be a loop
*/

struct Side<'a> {
    stats: St,
    //TODO accomidate multiple players
    attacks: [(&'a Abi<'a>, &'a mut State); 4],
    deffense: [(&'a Abi<'a>, &'a mut State); 4],
    turn: Vec<usize>,
}

impl<'a> Side<'a> {
    fn attack(&mut self) -> St {
        //itterate over turn, update counters
        //update moves/
        St::default()
    }
}

struct State {
    uses: i32,
    combo_index: usize, //negative value indicates out of combo
}

//ability
//TODO fix reffrence issues here / abstraction neeeds fixing

struct Abi<'a> {
    name: &'a str,
    desc: &'a str,

    cost: St,
    effect: St,

    modifyer: bool, //will effect be modifying dmg
    max_uses: Option<i32>,

    //TODO set as enum
    combo: [usize; 4],

    //what the ability will cost
    loss: fn(&Abi, St) -> St,
    //what the ability will do
    activate: fn(&Abi, St) -> St,
}

fn def_loss(a: &Abi, _: St) -> St {
    a.cost
}

fn def_act(a: &Abi, _: St) -> St {
    a.effect
}

impl<'a> Abi<'a> {
    const fn default() -> Self {
        Abi {
            name: "",
            desc: "",
            modifyer: false,
            cost: St::default(),
            effect: St::default(),
            loss: def_loss,
            activate: def_act,
            combo: [0; 4],
            max_uses: None,
        }
    }
}

static MOVES: &[(&'static str, Abi)] = &[
    (
        "slash",
        Abi {
            name: "slash",
            desc: "a quick slice of 4 damage",
            cost: St { h: 0, s: 2 },
            effect: St { h: 5, s: 0 },
            combo: [0, 0, 1, 0],
            ..Abi::default()
        },
    ),
    (
        "spin",
        Abi {
            name: "Spinning Strike",
            desc: "WEEE",
            cost: St { h: 0, s: 2 },
            effect: St { h: 10, s: 0 },
            ..Abi::default()
        },
    ),
];

// static OF_MOVES:[Abi;4] = [
//     Abi {
//         name: "slash",
//         desc: "a quick slice of 4 damage",
//         cost: St { h: 0, s:2  ,..St::default()},
//         effect: St { h: 5, s:0 ,..St::default() },
//         combo: Some(
//             (2,&mut Abi{
//                 name: "Spinning Attack",
//                 desc: "hurl yourself to the enemey gettem good",
//                 cost: St { h: 0, s:4  ,..St::default()},
//                 effect: St { h: 13, s:0 ,..St::default() },
//                 ..Abi::default()
//             })
//         ),
//         ..Abi::default()
//     },
//     Abi {
//         name: "Thrust",
//         desc: "drain your enemy of 3 stamina, dealing 10 dmg",
//         cost: St { h: 0, s: 6 ,..St::default() },
//         effect: St {h:20,s:3, ..St::default()},
//         combo : Some(
//             (3,&mut Abi{
//                 name:"Pommel Strike",
//                 desc: "Walk your enemey upside the head with sword 10 dmg, 20 stamina",
//                 cost: St { h: 0, s: 6 ,..St::default() },
//                 effect: St {h:10,s:20, ..St::default()},
//                 ..Abi::default()
//              })
//         ),
//         ..Abi::default()
//     },
//     Abi {
//         name: "Feint",
//         desc: "Trick your oppenet into loosing 5 stamina",
//         cost: St { h: 0, s: 4, ..St::default() },
//         effect: St {h:0,s:5, ..St::default()},
//         total: Some(1),
//         ..Abi::default()
//     },

//     Abi {
//         name: "Hooking",
//         desc: "Attempt to apply disarmed to your oppoenet",
//         cost: St { h: 0, s: 4, ..St::default()  },
//         effect: St {h:0,s:0, d:1,..St::default() },
//         total:Some(1),
//         ..Abi::default()
//     },
// ];

// static DF_MOVES: [Abi;3] = [
//     Abi {
//         name: "Parry",
//         desc: "Chance to negate some attacks next turn",
//         cost: St { h: 0, s: 5, ..St::default() },
//         effect: St { h:0, s:0, ..St::default()},
//         total: Some(1),
//         modifyer:true,
//        ..Abi::default()
//     },
//     Abi {
//         name: "Block",
//         desc: "Use Your stamina to block incoming damage",
//         cost: St { h: 0, s: 1, ..St::default() },
//         effect: St { h:1, s:0, ..St::default()},
//         modifyer: true,
//         total: Some(1),
//         ..Abi::default()
//     },
//     Abi {
//         name: "Dodge",
//         desc: "Chance to negate all damge",
//         cost: St { h: 0, s: 5, ..St::default() },
//         effect: St { h:0, s:0, ..St::default()},
//         modifyer: true,
//         total: Some(1),
//         ..Abi::default()
//     },
// ]

// static TEST: Abi = Abi {
//     name: "slash",
//     desc: "dose 5 damage to your opponent",
//     cost: St { h: 0, s: 1 },
//     effect: St { h: 5, s: 0 },
//     loss: slash,
//     ..Abi::default()
// };

const WIDTH: usize = 90;
const HEIGHT: usize = 30;
#[no_mangle]
pub unsafe extern "C" fn get_height() -> usize {
    HEIGHT
}
#[no_mangle]
pub unsafe extern "C" fn get_width() -> usize {
    WIDTH
}

static mut WORLD: Game = Game {
    player: Entity {
        pos: Pos {
            y: HEIGHT as u32 / 2,
            x: WIDTH as u32 / 2,
        },
        value: b'@',
        update: b'0',
    },
    level: [b'.'; HEIGHT * WIDTH], //current sim
    map: [b'.'; HEIGHT * WIDTH],   //bases of sim
};
#[no_mangle]
pub unsafe extern "C" fn tick() -> *const u8 {
    //there should be a way to do this better
    WORLD.level = WORLD.map;
    ply_update();

    WORLD.level.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn plyMove(v: u8) {
    WORLD.player.update |= v;
}

pub unsafe fn ply_update() {
    let update = WORLD.player.update;
    if update & 0b10000000 != 0 {
        WORLD.player.pos.x -= if WORLD.player.pos.x == 0 { 0 } else { 1 };
    }
    if update & 0b01000000 != 0 {
        WORLD.player.pos.y += if WORLD.player.pos.y == HEIGHT as u32 - 1 {
            0
        } else {
            1
        };
    }
    if update & 0b00100000 != 0 {
        WORLD.player.pos.y -= if WORLD.player.pos.y == 1 { 0 } else { 1 };
    }
    if update & 0b00010000 != 0 {
        WORLD.player.pos.x += if WORLD.player.pos.x == WIDTH as u32 - 1 {
            0
        } else {
            1
        };
    }
    WORLD.player.update = 0;
    WORLD.level[Game::get_index(&WORLD.player.pos)] = WORLD.player.value;
}

#[repr(C)]
pub struct Game {
    player: Entity,
    level: [u8; WIDTH * HEIGHT],
    map: [u8; WIDTH * HEIGHT],
}

impl Game {
    fn get_index(p: &Pos) -> usize {
        return (p.y * WIDTH as u32 + p.x) as usize;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    y: u32,
    x: u32,
}

pub struct Entity {
    pos: Pos,
    value: u8,
    update: u8,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            println!("hi");
            tick();
        }
    }
}
