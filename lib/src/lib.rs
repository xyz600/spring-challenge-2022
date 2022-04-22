// library to randomizer

pub struct XorShift {
    pub state: [u64; 2],
}

impl XorShift {
    pub fn new(seed: u64) -> Self {
        let mut ret = XorShift {
            state: [seed, seed * 195 + 1],
        };
        for _ in 0..128 {
            ret.next();
        }
        ret
    }
    pub fn next(&mut self) -> u64 {
        let mut s1 = self.state[0];
        let s0 = self.state[1];
        self.state[0] = s0;
        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        self.state[1] = s1;

        // avoid overflow for debug build
        if let Some(v) = s0.checked_add(s1) {
            v
        } else {
            s0 - (std::u64::MAX - s1) - 1
        }
    }

    pub fn next_float(&mut self) -> f64 {
        return (self.next() as f64) / (std::u64::MAX as f64);
    }
}

pub struct CachedRandom {
    int_table: Vec<u32>,
    uniform_table: Vec<f64>,
    log_table: Vec<f64>,
    index: usize,
}

impl CachedRandom {
    pub fn new(size: usize, seed: u64) -> CachedRandom {
        let mut ret = CachedRandom {
            int_table: vec![],
            uniform_table: vec![],
            log_table: vec![],
            index: 0,
        };

        let mut rand = XorShift::new(seed);

        for _ in 0..size {
            let val = (rand.next() >> 32) as u32;
            ret.int_table.push(val);

            let fval = (val as f64) / (std::u32::MAX as f64);
            ret.uniform_table.push(fval);

            // add eps to avoid log(0)
            let log_fval = (fval + 1e-20).log(std::f64::consts::E);
            ret.log_table.push(log_fval);
        }

        ret
    }

    pub fn next_int(&mut self) -> u32 {
        let ret = self.int_table[self.index];
        self.update();
        ret
    }

    // FIXME: 高速化
    pub fn next_int64(&mut self) -> u64 {
        let v1 = self.next_int();
        let v2 = self.next_int();
        ((v1 as u64) << 32) + (v2 as u64)
    }

    pub fn next_int_range(&mut self, left: u32, right: u32) -> u32 {
        (((right - left) as u64) * self.next_int() as u64 >> 32) as u32 + left
    }

    pub fn next_float(&mut self) -> f64 {
        let ret = self.uniform_table[self.index];
        self.update();
        ret
    }

    pub fn next_float_range(&mut self, left: f64, right: f64) -> f64 {
        self.next_float() * (right - left) + left
    }

    pub fn next_log_float(&mut self) -> f64 {
        let ret = self.log_table[self.index];
        self.update();
        ret
    }

    fn len(&self) -> usize {
        self.int_table.len()
    }

    fn update(&mut self) {
        self.index += 1;
        if self.index == self.len() {
            self.index = 0;
        }
    }
}

// game implementation

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    y: i32,
    x: i32,
}

impl Point {
    fn new() -> Point {
        Point { y: 0, x: 0 }
    }

    fn distance2(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.distance2(other) as f64).sqrt().ceil() as i32
    }

    fn norm2(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    fn norm(&self) -> i32 {
        (self.norm2() as f64).sqrt().ceil() as i32
    }

    fn point_symmetry(&self, center: &Point) -> Point {
        *center * 2 - *self
    }
}

impl std::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

const MAP_LIMIT: i32 = 800;
const MAX_X: i32 = 17630;
const MAX_Y: i32 = 9000;
const DETECT_BASE_RADIUS: i32 = 5000;
const WIND_RADIUS: i32 = 1280;
const MAX_PLAYER_VELOCITY: i32 = 800;
const MAX_MONSTER_VELOCITY: i32 = 400;
const THREASHOLD_BASE_DAMAGE_RADIUS: i32 = 300;
const PLAYER_DAMAGE: i32 = 2;
const VELOCITY_DIFF: i32 = MAX_PLAYER_VELOCITY - MAX_MONSTER_VELOCITY;
const HERO_SIZE: i32 = 3;
const CENTER: Point = Point {
    x: MAX_X / 2,
    y: MAX_Y / 2,
};

pub struct Player {
    health: i32,
    mana: i32,
    base: Point,
    hero_list: Vec<Hero>,
}

impl Player {
    fn new() -> Player {
        Player {
            health: 0,
            mana: 0,
            hero_list: vec![],
            base: Point::new(),
        }
    }
}

pub struct Hero {
    id: i32,
    pos: Point,
    shield_life: i32,    // not use
    is_controlled: bool, // not use
}

#[derive(Clone, Copy, PartialEq)]
pub enum MonsterThreatState {
    NotThreat,                 // nearBase == 0 && threatFor == 0
    PlayerThreatInTheFuture,   // nearBase == 0 && threatFor == 1
    PlayerThreat,              // nearBase == 1 && threatFor == 1
    OpponentThreatInTheFuture, // nearBase == 0 && threatFor == 2
    OpponentThreat,            // nearBase == 1 && threatFor == 2
}

impl MonsterThreatState {
    fn near_base(&self) -> bool {
        *self == MonsterThreatState::PlayerThreat || *self == MonsterThreatState::OpponentThreat
    }

    fn threat_player(&self) -> bool {
        *self == MonsterThreatState::PlayerThreat || *self == MonsterThreatState::PlayerThreatInTheFuture
    }

    fn threat_opponent(&self) -> bool {
        *self == MonsterThreatState::OpponentThreat || *self == MonsterThreatState::OpponentThreatInTheFuture
    }

    fn threat_level(&self) -> i64 {
        match *self {
            MonsterThreatState::NotThreat => 0,
            MonsterThreatState::PlayerThreatInTheFuture => 1,
            MonsterThreatState::PlayerThreat => 2,
            MonsterThreatState::OpponentThreatInTheFuture => -1,
            MonsterThreatState::OpponentThreat => -2,
        }
    }

    fn to_threat_state(near_base: i32, threat_for: i32) -> MonsterThreatState {
        if threat_for == 0 {
            MonsterThreatState::NotThreat
        } else if threat_for == 1 {
            if near_base == 1 {
                MonsterThreatState::PlayerThreat
            } else {
                MonsterThreatState::PlayerThreatInTheFuture
            }
        } else if threat_for == 2 {
            if near_base == 1 {
                MonsterThreatState::OpponentThreat
            } else {
                MonsterThreatState::OpponentThreatInTheFuture
            }
        } else {
            panic!("unknown threat type");
        }
    }
}

pub struct Monster {
    id: i32,
    pos: Point,
    shield_life: i32,    // not use
    is_controlled: bool, // not use
    health: i32,
    v: Point,
    threat_state: MonsterThreatState,
}

impl Monster {
    fn next_pos(&self) -> Point {
        self.pos + self.v
    }
}

#[derive(Clone)]
pub enum Action {
    Wait { message: String },
    Move { point: Point, message: String },
    Wind { point: Point, message: String },
}

pub struct GameBoard {
    random: CachedRandom,
    spawn_list: Vec<Point>,
    player: Player,
    opponent: Player,
    monster_list: Vec<Monster>,
    turn: usize,
    unique_id: usize,
}

impl GameBoard {
    pub fn new(seed: u64) -> GameBoard {
        let mut ret = GameBoard {
            random: CachedRandom::new(2usize.pow(16) - 1, seed),
            spawn_list: vec![
                Point {
                    x: MAX_X / 2,
                    y: -MAP_LIMIT + 1,
                },
                Point {
                    x: MAX_X / 2 + 4000,
                    y: -MAP_LIMIT + 1,
                },
            ],
            player: Player {
                health: 3,
                mana: 0,
                base: Point { x: 0, y: 0 },
                hero_list: vec![],
            },
            opponent: Player {
                health: 3,
                mana: 0,
                base: Point { x: MAX_X, y: MAX_Y },
                hero_list: vec![],
            },
            monster_list: vec![],
            turn: 0,
            unique_id: 0,
        };
        ret.spawn_list.push(ret.spawn_list[0].point_symmetry(&CENTER));
        ret.spawn_list.push(ret.spawn_list[1].point_symmetry(&CENTER));

        // player hero の追加
        for hero_id in 0..HERO_SIZE {
            let pos = Point { x: 0, y: 0 };
            ret.player.hero_list.push(Hero {
                id: hero_id,
                pos,
                shield_life: 0,
                is_controlled: false,
            });
        }

        // opponent hero の追加
        for hero_id in 0..HERO_SIZE {
            let pos = Point { x: 0, y: 0 };
            ret.opponent.hero_list.push(Hero {
                id: hero_id,
                pos: pos.point_symmetry(&CENTER),
                shield_life: 0,
                is_controlled: false,
            });
        }
        ret
    }

    pub fn next_state(&mut self, hero_actions1: Vec<Action>, hero_actions2: Vec<Action>) {
        println!("{}", 0);
    }
}
