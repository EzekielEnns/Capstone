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
*/

//ability
struct Abi<'a> {
    name: &'a str,
    desc: &'a str,

    cost: St,
    effect: St,

    lifespan: i32,  //how long effect will be added in rst phase
    modifyer: bool, //will effect be modifying dmg

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
            cost: St::default(),
            effect: St::default(),
            loss: def_loss,
            activate: def_act,
        }
    }
}

fn slash(_: &Abi, _: St) -> St {
    St { h: 5, s: 0 }
}

static TEST: Abi = Abi {
    name: "slash",
    desc: "dose 5 damage to your opponent",
    cost: St { h: 0, s: 1 },
    effect: St { h: 5, s: 0 },
    loss: slash,
    ..Abi::default()
};

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
