// random

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

#[derive(Debug)]
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

    pub fn next_boolean(&mut self) -> bool {
        self.next_float() <= 0.5
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

// inout info

pub mod inout {

    pub type Point = crate::IPoint;
    use crate::CENTER;
    pub use crate::{Action, MonsterThreatState};

    #[derive(Debug)]
    pub struct Board {
        pub player: Player,
        pub opponent: Player,
        pub monster_list: Vec<Monster>,
        pub turn: usize,
    }

    impl Board {
        pub fn monster(&self, monster_id: i32) -> Option<&Monster> {
            for m in self.monster_list.iter() {
                if m.id == monster_id {
                    return Some(m);
                }
            }
            None
        }

        pub fn dump(&self) {
            eprintln!("----");
            eprintln!("turn: {}", self.turn);
            eprintln!("player:");
            eprintln!("  base: {:?}", self.player.base);
            for (hero_id, hero) in self.player.hero_list.iter().enumerate() {
                eprintln!("  hero {}: {:?}", hero_id, hero);
            }
            eprintln!("opponent:");
            eprintln!("  base: {:?}", self.opponent.base);
            for (hero_id, hero) in self.opponent.hero_list.iter().enumerate() {
                eprintln!("  hero {}: {:?}", hero_id, hero);
            }
            eprintln!("monster_list:");
            for monster in self.monster_list.iter() {
                eprintln!("  {:?}", monster);
            }
            eprintln!("----");
        }

        pub fn point_symmetry(&mut self) {
            self.player.base = self.player.base.point_symmetry(&CENTER);
            for hero in self.player.hero_list.iter_mut() {
                hero.pos = hero.pos.point_symmetry(&CENTER);
            }
            self.opponent.base = self.opponent.base.point_symmetry(&CENTER);
            for hero in self.opponent.hero_list.iter_mut() {
                hero.pos = hero.pos.point_symmetry(&CENTER);
            }
            for monster in self.monster_list.iter_mut() {
                monster.pos = monster.pos.point_symmetry(&CENTER);
                monster.v = monster.v.flip();
            }
        }
    }

    #[derive(Debug)]
    pub struct Player {
        pub health: i32,
        pub mana: i32,
        pub base: Point,
        pub hero_list: Vec<Hero>,
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                health: 0,
                mana: 0,
                hero_list: vec![],
                base: Point::new(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Hero {
        pub id: i32,
        pub pos: Point,
        pub shield_life: i32,
        pub is_controlled: bool,
    }

    #[derive(Debug)]
    pub struct Monster {
        pub id: i32,
        pub pos: Point,
        pub shield_life: i32,
        pub is_controlled: bool,
        pub health: i32,
        pub v: Point,
        pub threat_state: MonsterThreatState,
    }

    impl Monster {
        pub fn next_pos(&self) -> Point {
            self.pos + self.v
        }
    }
}

// simulator

use inout::Board;

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

pub trait Number:
    One
    + Zero
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Neg<Output = Self>
    + std::marker::Sized
    + Clone
    + Copy
    + PartialOrd
    + PartialEq
{
    fn two() -> Self;

    fn to_f64(self) -> f64;

    fn from_f64(val: f64) -> Self;

    fn min(self, val: Self) -> Self;

    fn max(self, val: Self) -> Self;
}

impl Number for i32 {
    fn two() -> Self {
        2
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(val: f64) -> Self {
        // FIXME: is round necessary ?
        val.round() as Self
    }

    fn min(self, val: Self) -> Self {
        if self < val {
            self
        } else {
            val
        }
    }

    fn max(self, val: Self) -> Self {
        if self > val {
            self
        } else {
            val
        }
    }
}
impl Number for f64 {
    fn two() -> Self {
        2.0
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn from_f64(val: f64) -> Self {
        val
    }

    fn min(self, val: Self) -> Self {
        if self < val {
            self
        } else {
            val
        }
    }

    fn max(self, val: Self) -> Self {
        if self > val {
            self
        } else {
            val
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point<T>
where
    T: Number,
{
    pub y: T,
    pub x: T,
}

impl Point<f64> {
    fn to<T>(&self) -> Point<T>
    where
        T: Number,
    {
        Point {
            y: T::from_f64(self.y),
            x: T::from_f64(self.x),
        }
    }

    fn rotate(&self, dir: f64) -> Point<f64> {
        let c = dir.cos();
        let s = dir.sin();

        let nx = c * self.x - s * self.y;
        let ny = s * self.x + c * self.y;
        Point { y: ny, x: nx }
    }
}

impl<T> Point<T>
where
    T: Number,
{
    pub fn flip(&self) -> Point<T> {
        Point { y: -self.y, x: -self.x }
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn to_f64(self) -> Point<f64> {
        Point {
            y: T::to_f64(self.y),
            x: T::to_f64(self.x),
        }
    }

    pub fn min(self, p: &Point<T>) -> Point<T> {
        Point {
            y: self.y.min(p.y),
            x: self.x.min(p.x),
        }
    }

    pub fn max(self, p: &Point<T>) -> Point<T> {
        Point {
            y: self.y.max(p.y),
            x: self.x.max(p.x),
        }
    }

    pub fn new() -> Point<T> {
        Point {
            y: T::zero(),
            x: T::zero(),
        }
    }

    pub fn distance2(&self, other: &Point<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: &Point<T>) -> T {
        T::from_f64(self.to_f64().distance2(&other.to_f64()).sqrt().floor())
    }

    pub fn norm2(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(&self) -> T {
        T::from_f64(self.to_f64().norm2().sqrt().floor())
    }

    pub fn point_symmetry(&self, center: &Point<T>) -> Point<T> {
        *center * T::two() - *self
    }

    pub fn in_range(&self, p: &Point<T>, radius: T) -> bool {
        return p.distance2(&self) <= radius * radius;
    }

    pub fn cross(&self, p: &Point<T>) -> T {
        self.x * p.y - self.y * p.x
    }
}

impl<T: Number> std::ops::Add<Self> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Number> std::ops::Sub<Self> for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Number> std::ops::Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Number> std::ops::Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub type IPoint = Point<i32>;
pub type FPoint = Point<f64>;

#[derive(Debug, Clone, Copy)]
struct Line<T: Number> {
    p1: Point<T>,
    p2: Point<T>,
}

impl Line<f64> {
    fn min(&self) -> Point<f64> {
        Point {
            y: self.p1.y.min(self.p2.y),
            x: self.p1.x.min(self.p2.x),
        }
    }

    fn max(&self) -> Point<f64> {
        Point {
            y: self.p1.y.max(self.p2.y),
            x: self.p1.x.max(self.p2.x),
        }
    }

    fn intersect(&self, l: &Line<f64>) -> Option<Point<f64>> {
        let d1 = self.p1 - self.p2;
        let d2 = l.p1 - l.p2;
        let d = d1.cross(&d2);
        if d == 0.0 {
            None
        } else {
            let c1 = self.p1.cross(&self.p2);
            let c2 = l.p1.cross(&l.p2);

            let xi = d2.x * c1 - d1.x * c2;
            let yi = d2.y * c1 - d1.y * c2;
            let p = Point { x: xi, y: yi } / d;

            let min_x = l.min().x.min(self.min().x);
            let max_x = l.max().x.max(self.max().x);

            if min_x <= xi && xi <= max_x {
                Some(p)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::Line;
    use crate::Point;

    #[test]
    fn test_intersect() {
        let l1 = Line::<f64> {
            p1: Point { x: 150.0, y: 200.0 },
            p2: Point { x: -150.0, y: 120.0 },
        };

        let l2 = Line::<f64> {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 0.0, y: 5000.0 },
        };

        if let Some(p) = l1.intersect(&l2) {
            assert!((p.y - 160.0).abs() <= 1e-5);
        } else {
            assert!(false);
        }
    }
}

impl<T: Number> Line<T> {
    fn to_f64(self) -> Line<f64> {
        Line {
            p1: self.p1.to_f64(),
            p2: self.p2.to_f64(),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub health: i32,
    pub mana: i32,
    pub base: IPoint,
    pub hero_list: Vec<Hero>,
    pub wild_mana: i32,
}

impl Player {
    pub fn new(base: IPoint) -> Player {
        Player {
            health: 3,
            mana: 0,
            base,
            hero_list: vec![],
            wild_mana: 0,
        }
    }

    // この player から p が見えるか
    pub fn visible(&self, p: &IPoint) -> bool {
        // 自陣から
        if self.base.in_range(p, VISIBLE_RADIUS_FROM_BASE) {
            return true;
        }
        for hero in self.hero_list.iter() {
            if hero.component.position.in_range(p, VISIBLE_RADIUS_FROM_HERO) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Hero {
    pub component: Component,
    pub action: Action,
    is_player: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MonsterThreatState {
    NotThreat,                 // nearBase == 0 && threatFor == 0
    PlayerThreatInTheFuture,   // nearBase == 0 && threatFor == 1
    PlayerThreat,              // nearBase == 1 && threatFor == 1
    OpponentThreatInTheFuture, // nearBase == 0 && threatFor == 2
    OpponentThreat,            // nearBase == 1 && threatFor == 2
}

impl MonsterThreatState {
    pub fn near_base(&self) -> bool {
        *self == MonsterThreatState::PlayerThreat || *self == MonsterThreatState::OpponentThreat
    }

    pub fn threat_player(&self) -> bool {
        *self == MonsterThreatState::PlayerThreat || *self == MonsterThreatState::PlayerThreatInTheFuture
    }

    pub fn threat_opponent(&self) -> bool {
        *self == MonsterThreatState::OpponentThreat || *self == MonsterThreatState::OpponentThreatInTheFuture
    }

    pub fn threat_level(&self) -> i64 {
        match *self {
            MonsterThreatState::NotThreat => 0,
            MonsterThreatState::PlayerThreatInTheFuture => 1,
            MonsterThreatState::PlayerThreat => 2,
            MonsterThreatState::OpponentThreatInTheFuture => -1,
            MonsterThreatState::OpponentThreat => -2,
        }
    }

    pub fn to_threat_state(near_base: i32, threat_for: i32) -> MonsterThreatState {
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

#[derive(Debug, Clone)]
pub struct Monster {
    pub component: Component,
    pub health: i32,
    threat_state: MonsterThreatState,
}

impl Monster {
    fn max_health(spawn_count: i32) -> i32 {
        (10.0 + spawn_count as f64 * 0.5) as i32
    }

    fn move_canceled(&self) -> bool {
        self.component.pushed || self.health == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ComponentType {
    Monster,
    PlayerHero,
    OpponentHero,
}

impl ComponentType {
    fn is_hero(&self) -> bool {
        *self == ComponentType::OpponentHero || *self == ComponentType::PlayerHero
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    pub id: i32,
    pub position: IPoint,
    pub velocity: IPoint,
    pub shield_life: i32,
    pub max_velocity: i32,
    // 現在のターンで反映されるべき
    control_move_target: Vec<IPoint>,
    // control によって、次の action で反映される動き
    next_control_move_target: Vec<IPoint>,
    pushed: bool,
    component_type: ComponentType,
}

impl Component {
    fn new(id: i32, position: IPoint, max_velocity: i32, component_type: ComponentType) -> Component {
        Component {
            id,
            position,
            velocity: Point { x: 0, y: 0 },
            shield_life: 0,
            control_move_target: vec![],
            next_control_move_target: vec![],
            max_velocity,
            pushed: false,
            component_type,
        }
    }

    fn next_pos(&self) -> IPoint {
        self.position + self.velocity
    }

    pub fn is_controlled(&self) -> bool {
        !self.control_move_target.is_empty()
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Wait {
        message: String,
    },
    Move {
        point: IPoint,
        message: String,
    },
    Wind {
        point: IPoint,
        message: String,
    },
    Shield {
        entity_id: i32,
        message: String,
    },
    Control {
        entity_id: i32,
        point: IPoint,
        message: String,
    },
}

#[derive(Debug)]
struct SpawnLocation {
    // 発生場所
    pos: FPoint,

    dir: FPoint,
}

impl SpawnLocation {
    fn new(pos: FPoint) -> SpawnLocation {
        SpawnLocation {
            pos,
            dir: if pos.y < MAX_Y as f64 / 2.0 {
                FPoint { x: 0.0, y: 1.0 }
            } else {
                FPoint { x: 0.0, y: -1.0 }
            },
        }
    }
}

#[derive(Debug)]
pub struct System {
    unique_id: i32,
    random: CachedRandom,
}

impl System {
    fn create_id(&mut self) -> i32 {
        let ret = self.unique_id;
        self.unique_id += 1;
        ret
    }
}

#[derive(Debug)]
pub struct Components {
    pub player_list: [Player; 2],
    pub monster_list: Vec<Monster>,
}

impl Components {
    const PLAYER_ID: usize = 0;
    const OPPONENT_ID: usize = 1;

    pub fn component_iter(&self) -> impl Iterator<Item = &Component> {
        self.player()
            .hero_list
            .iter()
            .map(|h| &h.component)
            .chain(self.opponent().hero_list.iter().map(|h| &h.component))
            .chain(self.monster_list.iter().map(|m| &m.component))
    }

    pub fn component_of(&self, id: i32) -> Option<&Component> {
        self.component_iter().filter(|c| c.id == id).next()
    }

    fn for_each_component_mut(&mut self, f: impl Fn(&mut Component)) {
        for player in self.player_list.iter_mut() {
            for hero in player.hero_list.iter_mut() {
                f(&mut hero.component);
            }
        }
        for monster in self.monster_list.iter_mut() {
            f(&mut monster.component);
        }
    }

    fn component_of_mut(&mut self, id: i32) -> Option<&mut Component> {
        for player in self.player_list.iter_mut() {
            for hero in player.hero_list.iter_mut() {
                if hero.component.id == id {
                    Some(&mut hero.component);
                }
            }
        }
        for monster in self.monster_list.iter_mut() {
            if monster.component.id == id {
                Some(&mut monster.component);
            }
        }
        None
    }

    pub fn component_len(&self) -> usize {
        self.player().hero_list.len() + self.opponent().hero_list.len() + self.monster_list.len()
    }

    fn find_component(&self, id: i32) -> bool {
        self.component_iter().any(|c| c.id == id)
    }

    pub fn player(&self) -> &Player {
        &self.player_list[Self::PLAYER_ID]
    }

    fn player_mut(&mut self) -> &mut Player {
        &mut self.player_list[Self::PLAYER_ID]
    }

    pub fn opponent(&self) -> &Player {
        &self.player_list[Self::OPPONENT_ID]
    }

    fn opponent_mut(&mut self) -> &mut Player {
        &mut self.player_list[Self::OPPONENT_ID]
    }
}

#[derive(Debug)]
pub struct Simulator {
    pub components: Components,
    pub turn: usize,
    spawn_location: Vec<SpawnLocation>,
    system: System,

    // FIXME: 何に使うかよくわからない
    activated_hero: Vec<i32>,
}

pub const MAX_X: i32 = 17630;
pub const MAX_Y: i32 = 9000;
pub const CENTER: IPoint = IPoint {
    x: MAX_X / 2,
    y: MAX_Y / 2,
};
pub const MAP_LIMIT: i32 = 800;
pub const MANA_TO_SPELL: i32 = 10;
pub const MAX_HERO_VELOCITY: i32 = 800;
pub const MAX_MONSTER_VELOCITY: i32 = 400;
pub const SHIELD_EFFECTIVE_TURN: i32 = 12;
pub const MANA_GAIN_TO_ATTACK: i32 = 2;
pub const HERO_ATTACK_RADIUS: i32 = 800;
pub const WIND_EFFECTIVE_RADIUS: i32 = 1280;
pub const CONTROL_EFFECTIVE_RADIUS: i32 = 2200;
pub const WIND_DISTANCE: i32 = 2200;
pub const VISIBLE_RADIUS_FROM_BASE: i32 = 6000;
pub const VISIBLE_RADIUS_FROM_HERO: i32 = 2200;
pub const MOB_SPAWN_MAX_DIRECTION_DELTA: f64 = 5.0 * std::f64::consts::PI / 12.0;

#[derive(Clone, Copy)]
struct ManaInfo {
    wild_mana: i32,
    all_mana: i32,
}

impl Simulator {
    pub fn new(seed: u64) -> Simulator {
        let player_base = Point { x: 0, y: 0 };
        let opponent_base = Point { x: MAX_X, y: MAX_Y };

        let mut ret = Simulator {
            system: System {
                unique_id: 0,
                random: CachedRandom::new(65535, seed),
            },
            components: Components {
                // opponent に対しても 点対称 に回してから渡すので、opponent_base ではない
                player_list: [Player::new(player_base), Player::new(opponent_base)],
                monster_list: vec![],
            },
            turn: 0,
            spawn_location: vec![
                SpawnLocation::new(Point {
                    y: (-MAP_LIMIT + 1) as f64,
                    x: (MAX_X / 2) as f64,
                }),
                SpawnLocation::new(Point {
                    y: (-MAP_LIMIT + 1) as f64,
                    x: (MAX_X / 2 + 4000) as f64,
                }),
            ],
            activated_hero: vec![],
        };

        for i in 0..3 {
            let hero = ret.create_hero(player_base, true);
            ret.components.player_mut().hero_list.push(hero);
            // Simulator の中で opponent_base 側も点対称に回して渡すので、opponent_base ではない
            let hero = ret.create_hero(opponent_base, false);
            ret.components.opponent_mut().hero_list.push(hero);
        }

        ret
    }

    fn create_hero(&mut self, pos: IPoint, is_player: bool) -> Hero {
        Hero {
            component: Component::new(
                self.system.create_id(),
                pos,
                MAX_HERO_VELOCITY,
                if is_player {
                    ComponentType::PlayerHero
                } else {
                    ComponentType::OpponentHero
                },
            ),
            action: Action::Wait { message: String::new() },
            is_player,
        }
    }

    pub fn finish_game(&self) -> bool {
        self.components.player().health == 0 || self.components.opponent().health == 0 || self.turn == 220
    }

    fn initialize(&mut self, player_action: Vec<Action>, opponent_action: Vec<Action>) {
        // Action のコピー
        // 死んでる mob を削除
        // mob の reset
        //   - winded 解除
        //   - 直前に行われた control を active に移動
        //     - active が空でなければ、control での移動先を計算
        //     - 複数 control の移動先を、最大 speed を 400 に normalize した平均を実数で取って移動
        //   - 次の control 予約を削除

        // clear controlled
        for player in self.components.player_list.iter_mut() {
            for hero in player.hero_list.iter_mut() {
                std::mem::swap(
                    &mut hero.component.control_move_target,
                    &mut hero.component.next_control_move_target,
                );
                hero.component.next_control_move_target.clear();
                hero.component.pushed = false;
            }
        }
        for monster in self.components.monster_list.iter_mut() {
            std::mem::swap(
                &mut monster.component.control_move_target,
                &mut monster.component.next_control_move_target,
            );
            monster.component.next_control_move_target.clear();
            monster.component.pushed = false;
        }

        // set hero action
        // fixme: avoid clone
        for hero_id in 0..3 {
            self.components.player_mut().hero_list[hero_id].action = player_action[hero_id].clone();
            self.components.opponent_mut().hero_list[hero_id].action = opponent_action[hero_id].clone();
        }

        self.activated_hero.clear();
    }

    fn do_control(&mut self) {
        for player_id in 0..2 {
            for hero_id in 0..3 {
                // control の場合
                if let Action::Control {
                    entity_id,
                    point,
                    message: _,
                } = self.components.player_list[player_id].hero_list[hero_id].action
                {
                    // マナが足りなかったら何もしない
                    if MANA_TO_SPELL <= self.components.player_list[player_id].mana
                        && self.components.find_component(entity_id)
                    {
                        {
                            let target = self.components.component_of(entity_id).unwrap();
                            // その player から見えなかったら違反
                            assert!(self.components.player_list[player_id].visible(&target.position));
                            // その hero の shield 半径外にいたら違反
                            assert!(self.components.player_list[player_id].hero_list[hero_id]
                                .component
                                .position
                                .in_range(&target.position, CONTROL_EFFECTIVE_RADIUS));
                        }

                        // マナ消費
                        self.components.player_list[player_id].mana -= MANA_TO_SPELL;

                        // control 先を設定
                        // 反映は次ターン
                        let target = self.components.component_of_mut(entity_id).unwrap();

                        if target.shield_life == 0 {
                            target.control_move_target.push(point);
                        }
                    }
                }
            }
        }
    }

    fn do_shield(&mut self) {
        for player_id in 0..2 {
            for hero_id in 0..3 {
                if let Action::Shield { entity_id, message: _ } =
                    self.components.player_list[player_id].hero_list[hero_id].action
                {
                    // マナが足りなかったら何もしない
                    if MANA_TO_SPELL <= self.components.player_list[player_id].mana
                        && self.components.find_component(entity_id)
                    {
                        {
                            // shield 適用先を設定
                            let target = self.components.component_of(entity_id).unwrap();

                            // その player から見えなかったら違反
                            assert!(self.components.player_list[player_id].visible(&target.position));
                            // その hero の shield 半径外にいたら違反
                            assert!(self.components.player_list[player_id].hero_list[hero_id]
                                .component
                                .position
                                .in_range(&target.position, CONTROL_EFFECTIVE_RADIUS));
                        }
                        // マナ消費
                        self.components.player_list[player_id].mana -= MANA_TO_SPELL;

                        // shield 適用先を設定
                        let target = self.components.component_of_mut(entity_id).unwrap();
                        // shield 有効の場合、shield 魔法自体も無効
                        if target.shield_life == 0 {
                            // 最後に decrease する分も足しておく
                            target.shield_life = SHIELD_EFFECTIVE_TURN + 1;
                        }
                    }
                }
            }
        }
    }

    /// WIND は実行したら即座に場所が反映される
    fn do_wind(&mut self) {
        let mut diff = std::collections::HashMap::<i32, Vec<IPoint>>::new();

        for player_id in 0..2 {
            for hero in self.components.player_list[player_id].hero_list.iter() {
                // wind
                if let Action::Wind { point, message: _ } = hero.action {
                    // マナが足りなかったら何もしない
                    if MANA_TO_SPELL <= self.components.player_list[player_id].mana {
                        // マナ消費
                        self.components.player_list[player_id].mana -= MANA_TO_SPELL;

                        // wind 適用相手を見つける (自分の hero 以外)
                        for component in self.components.player_list[1 - player_id]
                            .hero_list
                            .iter()
                            .map(|h| &h.component)
                            .chain(self.components.monster_list.iter().map(|m| &m.component))
                        {
                            if component
                                .position
                                .in_range(&hero.component.position, WIND_EFFECTIVE_RADIUS)
                                && component.shield_life == 0
                            {
                                assert!(point != hero.component.position);

                                let dir = ((point - hero.component.position).to_f64().normalize()
                                    * (WIND_DISTANCE as f64))
                                    .to::<i32>();
                                diff.entry(component.id).or_insert(vec![]).push(dir);
                            }
                        }
                    }
                }
            }
        }

        // wind component to move
        for (k, v) in diff.iter() {
            // FIXME: original では truncate な実装をしているが、それを忠実に再現していない
            let mut component = self.components.component_of_mut(*k).unwrap();
            component.pushed = true;
            let sum_diff = v.iter().fold(Point::new(), |p1, p2| p1 + *p2);

            let np = component.position + sum_diff;

            // base 範囲内から外へ出る動きをする場合 || hero は、外に出ずに停止する
            if component.component_type.is_hero() || Simulator::go_outside_around_base(&component.position, &np) {
                component.position = Simulator::snap_to_game_zone(np);
            } else {
                component.position = np;
                // base 近辺から外に出た monster
                if !component.component_type.is_hero() && Simulator::go_out_from_base(&component.position, &np) {
                    // ランダムに回転して方向が決まる
                    let angle = self.system.random.next_float() * std::f64::consts::PI * 2.0;
                    let vy = (angle.sin() * (component.max_velocity as f64)).round() as i32;
                    let vx = (angle.cos() * (component.max_velocity as f64)).round() as i32;
                    component.velocity = Point { y: vy, x: vx };
                }
            }
        }
    }

    fn calculate_real_target(pos: &IPoint, target: &IPoint, speed: i32) -> IPoint {
        if pos.in_range(&target, speed) {
            *target
        } else {
            ((*target - *pos).to_f64().normalize() * (speed as f64)).to::<i32>()
        }
    }

    fn go_out_from_base(p: &IPoint, np: &IPoint) -> bool {
        [Point { x: 0, y: 0 }, Point { x: MAX_X, y: MAX_Y }]
            .iter()
            .any(|base| base.in_range(&p, VISIBLE_RADIUS_FROM_BASE) && !base.in_range(&np, VISIBLE_RADIUS_FROM_BASE))
    }

    fn go_outside_around_base(p: &IPoint, np: &IPoint) -> bool {
        let ps: [FPoint; 6] = [
            Point { x: 0.0, y: 0.0 },
            Point {
                x: VISIBLE_RADIUS_FROM_BASE as f64,
                y: 0.0,
            },
            Point {
                x: 0.0,
                y: VISIBLE_RADIUS_FROM_BASE as f64,
            },
            Point {
                x: MAX_X as f64 - 1.0,
                y: MAX_Y as f64 - 1.0,
            },
            Point {
                x: (MAX_X - VISIBLE_RADIUS_FROM_BASE) as f64 - 1.0,
                y: MAX_Y as f64 - 1.0,
            },
            Point {
                x: MAX_X as f64 - 1.0,
                y: (MAX_Y - VISIBLE_RADIUS_FROM_BASE) as f64 - 1.0,
            },
        ];

        let target = Line::<f64> {
            p1: p.to_f64(),
            p2: np.to_f64(),
        };

        [
            Line::<f64> { p1: ps[0], p2: ps[1] },
            Line::<f64> { p1: ps[0], p2: ps[2] },
            Line::<f64> { p1: ps[3], p2: ps[4] },
            Line::<f64> { p1: ps[3], p2: ps[5] },
        ]
        .iter()
        .any(|l| -> bool { l.intersect(&target).is_some() })
    }

    // game board の外に出ずに止まる
    fn snap_to_game_zone(p: IPoint) -> IPoint {
        Point {
            y: p.y.clamp(0, MAX_Y - 1),
            x: p.x.clamp(0, MAX_X - 1),
        }
    }

    fn move_hero(&mut self) {
        // FIXME: control されたけど move している hero の影響は？
        for player in self.components.player_list.iter_mut() {
            for hero in player.hero_list.iter_mut() {
                if let Action::Move { point, message: _ } = hero.action {
                    hero.component.velocity =
                        ((point - hero.component.position).to_f64().normalize() * (MAX_HERO_VELOCITY as f64)).to();
                    // 外に出ないための対策
                    hero.component.position = Self::snap_to_game_zone(hero.component.next_pos());
                }
            }
        }
    }

    fn attack_monster(&mut self) -> [ManaInfo; 2] {
        let mut mana_gain = [ManaInfo {
            wild_mana: 0,
            all_mana: 0,
        }; 2];

        for monster in self.components.monster_list.iter_mut() {
            for (player_id, player) in self.components.player_list.iter_mut().enumerate() {
                for hero in player.hero_list.iter_mut() {
                    if monster
                        .component
                        .position
                        .in_range(&hero.component.position, HERO_ATTACK_RADIUS)
                    {
                        monster.health -= MANA_GAIN_TO_ATTACK;
                        mana_gain[player_id].all_mana += MANA_GAIN_TO_ATTACK;
                        if player
                            .base
                            .in_range(&monster.component.position, VISIBLE_RADIUS_FROM_BASE)
                        {
                            mana_gain[player_id].wild_mana += MANA_GAIN_TO_ATTACK;
                        }
                    }
                }
            }
        }
        mana_gain
    }

    fn create_monster(&mut self, position: FPoint, velocity: IPoint) {
        let mut monster = Monster {
            component: Component::new(
                self.system.create_id(),
                position.to::<i32>(),
                MAX_MONSTER_VELOCITY,
                ComponentType::Monster,
            ),
            health: Monster::max_health(self.turn as i32 / 5),
            threat_state: MonsterThreatState::NotThreat, // FIXME: assign appropreate state
        };
        monster.component.velocity = velocity;
        self.components.monster_list.push(monster);

        // create flipped monster
        let position = position.point_symmetry(&CENTER.to_f64());
        let velocity = velocity.flip();
        let mut monster = Monster {
            component: Component::new(
                self.system.create_id(),
                position.to::<i32>(),
                MAX_MONSTER_VELOCITY,
                ComponentType::Monster,
            ),
            health: Monster::max_health(self.turn as i32 / 5),
            threat_state: MonsterThreatState::NotThreat, // FIXME: assign appropreate state
        };
        monster.component.velocity = velocity;
        self.components.monster_list.push(monster);
    }

    fn adjust_monster(&mut self) {
        // remove dead monster
        self.components.monster_list.retain(|m| {
            m.health > 0
                && -MAP_LIMIT + 1 <= m.component.position.x
                && m.component.position.x <= MAX_X + MAP_LIMIT - 1
                && -MAP_LIMIT + 1 <= m.component.position.y
                && m.component.position.y <= MAX_Y + MAP_LIMIT - 1
        });

        let sudden_death = self.turn >= 200;

        // appear new monster
        if sudden_death || self.turn % 5 == 0 {
            for loc_id in 0..self.spawn_location.len() {
                if sudden_death {
                    let mut tx = self.system.random.next_int_range(0, VISIBLE_RADIUS_FROM_BASE as u32) as i32;
                    let mut ty = self.system.random.next_int_range(0, VISIBLE_RADIUS_FROM_BASE as u32) as i32;
                    if self.system.random.next_boolean() {
                        tx = MAX_X - tx;
                        ty = MAX_Y - ty;
                    }
                    let target = FPoint {
                        x: tx as f64,
                        y: ty as f64,
                    };

                    let position = self.spawn_location[loc_id].pos;
                    let velocity = ((target - position).normalize() * MAX_MONSTER_VELOCITY as f64).to::<i32>();
                    self.create_monster(position, velocity);
                } else {
                    let direction_delta = self
                        .system
                        .random
                        .next_float_range(-MOB_SPAWN_MAX_DIRECTION_DELTA, MOB_SPAWN_MAX_DIRECTION_DELTA);

                    let position = self.spawn_location[loc_id].pos;
                    let velocity = (self.spawn_location[loc_id].dir.rotate(direction_delta)
                        * MAX_MONSTER_VELOCITY as f64)
                        .to::<i32>();
                    self.create_monster(position, velocity);
                }
            }
        }
    }

    fn is_in_gameboard(p: &Point<i32>) -> bool {
        -MAP_LIMIT <= p.x && p.x <= MAX_X + MAP_LIMIT && -MAP_LIMIT <= p.y && p.x <= MAX_Y + MAP_LIMIT
    }

    fn move_monster(&mut self) {
        for m in self.components.monster_list.iter_mut() {
            // 既に盤外に出ていたら無視（initialize で実際に削除）
            if !Simulator::is_in_gameboard(&m.component.position) {
                continue;
            }

            // 移動キャンセル出ていなかったら、以下
            if !m.move_canceled() {
                // 1. control されていたら、その通りに動く
                // FIXME: f64 で計算してない
                if m.component.is_controlled() {
                    let averaged_target = m
                        .component
                        .control_move_target
                        .iter()
                        .map(|p| Self::calculate_real_target(&m.component.position, &p, MAX_MONSTER_VELOCITY))
                        .fold(Point::new(), |l, r| l + r)
                        / m.component.control_move_target.len() as i32;
                } else {
                    // 2. そうでなければ、設定された velocity を基に動く
                    m.component.position = m.component.next_pos();
                }
            }

            m.component.position = m.component.next_pos();
        }
    }

    fn countdown_shield(&mut self) {
        self.components
            .for_each_component_mut(|c: &mut Component| c.shield_life = Number::max(0, c.shield_life - 1));
    }

    // original code の gameTurn 相当（ただし、各プレイヤーの action は事前に貰っているものとする）
    pub fn next_state(&mut self, player_action: Vec<Action>, opponent_action: Vec<Action>) {
        // 0. (self) clear hero state
        self.initialize(player_action, opponent_action);

        // 1. (Wait for both players to output 3 commands.)

        // 2. CONTROL spells are applied to the targets and will only be effective on the next turn, after the next batch of commands.
        self.do_control();

        // 3. SHIELD spells are applied to the targets and will only be effective on the next turn, after the next batch of commands.
        // Does not protect from a spell from this same turn.
        self.do_shield();

        // 4. MOVE all heroes.
        self.move_hero();

        // 5. Heroes attack monsters in range and produce mana for each hit.
        let mana_gain = self.attack_monster();

        // 6. WIND spells are applied to entities in range.
        self.do_wind();

        // 7. MOVE all monsters according to their current speed, unless they were pushed by a wind on this turn.
        self.move_monster();

        // 8. SHIELD countdowns are decremented.
        self.countdown_shield();

        // 9. New monsters appear. Dead monsters are removed.
        self.adjust_monster();

        for player_id in 0..2 {
            self.components.player_list[player_id].mana += mana_gain[player_id].all_mana;
        }

        self.turn += 1;
    }

    /// player_id にとって visible な情報だけを取得して返す
    /// オリジナルコードの sendGameStateFor()
    pub fn to_board(&self, player_id: i32) -> inout::Board {
        let mut board = inout::Board {
            player: inout::Player::new(),
            opponent: inout::Player::new(),
            monster_list: vec![],
            turn: self.turn,
        };

        // player hero
        board.player.hero_list.clear();
        for hero in self.components.player_list[player_id as usize].hero_list.iter() {
            let inout_hero = inout::Hero {
                id: hero.component.id,
                pos: hero.component.position,
                shield_life: hero.component.shield_life,
                is_controlled: hero.component.is_controlled(),
            };
            board.player.hero_list.push(inout_hero);
        }
        board.player.base = IPoint {
            x: player_id * MAX_X,
            y: player_id * MAX_Y,
        };

        // opponent hero
        board.opponent.hero_list.clear();
        for op_hero in self.components.player_list[1 - player_id as usize].hero_list.iter() {
            if self.components.player_list[player_id as usize].visible(&op_hero.component.position) {
                board.opponent.hero_list.push(inout::Hero {
                    id: op_hero.component.id,
                    pos: op_hero.component.position,
                    shield_life: op_hero.component.shield_life,
                    is_controlled: op_hero.component.is_controlled(),
                })
            }
        }
        board.opponent.base = IPoint {
            x: (1 - player_id) * MAX_X,
            y: (1 - player_id) * MAX_Y,
        };

        // monster
        board.monster_list.clear();
        for m in self.components.monster_list.iter() {
            if self.components.player_list[player_id as usize].visible(&m.component.position) {
                let mut monster = inout::Monster {
                    id: m.component.id,
                    pos: m.component.position,
                    shield_life: m.component.shield_life,
                    is_controlled: m.component.is_controlled(),
                    health: m.health,
                    v: m.component.velocity,
                    threat_state: MonsterThreatState::NotThreat,
                };
                monster.threat_state = self.decide_monster_threat(player_id, &m);

                board.monster_list.push(monster);
            }
        }
        // turn
        board.turn = self.turn;

        board
    }

    fn decide_monster_threat(&self, player_id: i32, m: &Monster) -> MonsterThreatState {
        if m.component.velocity.norm2() == 0 {
            MonsterThreatState::NotThreat
        } else if self.components.player_list[player_id as usize]
            .base
            .in_range(&m.component.position, VISIBLE_RADIUS_FROM_BASE)
        {
            MonsterThreatState::PlayerThreat
        } else if self.components.player_list[1 - player_id as usize]
            .base
            .in_range(&m.component.position, VISIBLE_RADIUS_FROM_BASE)
        {
            MonsterThreatState::OpponentThreat
        } else {
            for turn in 1.. {
                let p = m.component.position + m.component.velocity * turn;
                if self.components.player_list[player_id as usize]
                    .base
                    .in_range(&p, VISIBLE_RADIUS_FROM_BASE)
                {
                    return MonsterThreatState::PlayerThreatInTheFuture;
                } else if self.components.player_list[1 - player_id as usize]
                    .base
                    .in_range(&p, VISIBLE_RADIUS_FROM_BASE)
                {
                    return MonsterThreatState::OpponentThreatInTheFuture;
                } else if !(0 <= p.x && p.x < MAX_X && 0 <= p.y && p.y < MAX_Y) {
                    return MonsterThreatState::NotThreat;
                }
            }
            panic!("unintentional code");
        }
    }
}
