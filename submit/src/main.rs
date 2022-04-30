// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

use std::{collections::HashSet, time::Instant};

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

macro_rules! input_old {
    (line_num: $n:expr, $($t:tt)+) => {
        let content = ::std::io::BufRead::lines(::std::io::stdin().lock())
            .take($n)
            .map(|line| line.unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        let source = content.as_str();
        input! {
            source = source,
            $($t)*
        }
    };
}

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

#[derive(Debug)]
struct Board {
    player: Player,
    opponent: Player,
    monster_list: Vec<Monster>,
    turn: usize,
}

impl Board {
    fn monster(&self, monster_id: i32) -> Option<&Monster> {
        for m in self.monster_list.iter() {
            if m.id == monster_id {
                return Some(m);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Player {
    health: i32,
    mana: i32,
    base: IPoint,
    hero_list: Vec<Hero>,
}

impl Player {
    fn new() -> Player {
        Player {
            health: 0,
            mana: 0,
            hero_list: vec![],
            base: IPoint::new(),
        }
    }
}

#[derive(Debug)]
struct Hero {
    id: i32,
    pos: IPoint,
    shield_life: i32,    // not use
    is_controlled: bool, // not use
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum MonsterThreatState {
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

#[derive(Debug)]
struct Monster {
    id: i32,
    pos: IPoint,
    shield_life: i32,    // not use
    is_controlled: bool, // not use
    health: i32,
    v: IPoint,
    threat_state: MonsterThreatState,
}

impl Monster {
    fn next_pos(&self) -> IPoint {
        self.pos + self.v
    }
}

#[derive(Clone)]
enum Action {
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

#[derive(PartialEq, Copy, Clone)]
struct CollectManaInfo {
    home: IPoint,
}

impl CollectManaInfo {
    fn calculate_home_to_collect_mana(base_pos: &IPoint, hero_id: usize) -> IPoint {
        if hero_id == 0 {
            IPoint {
                x: MAX_X * 6 / 10,
                y: MAX_Y * 6 / 10,
            }
        } else if hero_id == 1 {
            IPoint {
                x: MAX_X * 4 / 10,
                y: MAX_Y * 4 / 10,
            }
        } else {
            let rad = std::f64::consts::PI / 4.0;
            let radius = DETECT_BASE_RADIUS as f64;
            let dx = (rad.cos() * radius) as i32;
            let dy = (rad.sin() * radius) as i32;
            *base_pos + IPoint { x: dx, y: dy }
        }
    }

    fn new(base_pos: &IPoint, hero_id: usize) -> CollectManaInfo {
        CollectManaInfo {
            home: Self::calculate_home_to_collect_mana(base_pos, hero_id),
        }
    }

    fn action(&mut self, board: &Board, hero_id: usize, solver: &mut SolverState) -> Action {
        let hero = &board.player.hero_list[hero_id];
        if hero_id == 2 {
            // 相手 hero, monster, 自分 hero 全員 WIND 圏内にいる場合は、wind!
            if solver.can_spell(board, true) {
                for op_hero in board.opponent.hero_list.iter() {
                    for monster in board.monster_list.iter() {
                        if hero.pos.distance(&op_hero.pos) <= WIND_RADIUS
                            && hero.pos.distance(&monster.pos) <= WIND_RADIUS
                            && op_hero.pos.distance(&monster.pos) <= WIND_RADIUS
                        {
                            // FIXME: 相手が1手で WIND を使える条件をもっと正確に書く
                            let point = hero.pos * 2 - board.player.base;
                            solver.spell_count += 1;
                            return Action::Wind {
                                point,
                                message: format!("[m1]im wind"),
                            };
                        }
                    }
                }
            }

            let candidate = board
                .monster_list
                .iter()
                .filter(|m| m.pos.distance(&board.player.base) <= DETECT_BASE_RADIUS)
                .min_by_key(|m| m.pos.distance(&board.player.base));

            if let Some(monster) = candidate {
                let around_alive = board
                    .monster_list
                    .iter()
                    .filter(|m| m.pos.distance(&hero.pos) <= WIND_RADIUS && m.health > 2)
                    .count()
                    > 0;
                // 既に monster に追いついていて、1撃じゃ死なない monster に goal されそうなら、 WIND!
                if around_alive
                    && monster.pos.distance(&board.player.base) <= THREASHOLD_BASE_DAMAGE_RADIUS + MAX_MONSTER_VELOCITY
                    && monster.pos.distance(&hero.pos) <= WIND_RADIUS
                    && solver.can_spell(board, true)
                {
                    let point = hero.pos * 2 - board.player.base;
                    solver.spell_count += 1;
                    Action::Wind {
                        point,
                        message: format!("[m1]wind"),
                    }
                } else if hero.shield_life == 0 && solver.can_spell(board, true) && solver.is_opponent_speller[hero_id]
                {
                    solver.spell_count += 1;
                    return Action::Shield {
                        entity_id: hero.id,
                        message: format!("[m1]shield self!"),
                    };
                } else if hero.pos.distance(&monster.pos) <= ATTACK_HIT_RADIUS {
                    // 攻撃が当たるなら、マナの収集効率が良い場所を見つける
                    let candidate = self.enumerate_multiple_hit_with_target(board, hero_id, monster);
                    assert!(!candidate.is_empty());

                    Action::Move {
                        point: candidate[0].1,
                        message: format!("[m1]{}attack", candidate[0].0),
                    }
                } else {
                    let (turn, point) = self.shortest_move(&monster, &hero.pos);
                    Action::Move {
                        point,
                        message: format!("[m1]shortest"),
                    }
                }
            } else {
                Action::Move {
                    point: self.home,
                    message: format!("[m1]go home"),
                }
            }
        } else {
            // hero_id = 0
            // マナを集める

            let point = self.home;
            let candidate = self.enumerate_multiple_hit(board, hero_id);

            if hero.shield_life == 0 && solver.can_spell(board, false) && solver.is_opponent_speller[hero_id] {
                solver.spell_count += 1;
                return Action::Shield {
                    entity_id: hero.id,
                    message: format!("[m2]shield self!"),
                };
            } else if candidate.is_empty() {
                // 候補がなければ自分の home に向かう
                Action::Move {
                    point,
                    message: format!("[m2]go home"),
                }
            } else {
                // FIXME: 探索して、数手分で最もマナが稼げる所に移る

                // 即時効果が発揮できる場所なら、効果が高い順に移動する
                for (hit, pos) in candidate.iter() {
                    if hero.pos.distance(&pos) <= MAX_PLAYER_VELOCITY {
                        return Action::Move {
                            point: *pos,
                            message: format!("[m2]{}attack", hit),
                        };
                    }
                }

                // hit がたくさんある、一番近いところにとりあえず移っておく
                let target = candidate
                    .iter()
                    .filter(|p| p.0 == candidate[0].0)
                    .map(|(_, p)| p)
                    .min_by_key(|p| p.distance2(&hero.pos))
                    .unwrap();

                Action::Move {
                    point: *target,
                    message: format!("[m2]move only"),
                }
            }
        }
    }

    fn enumerate_multiple_hit(&self, board: &Board, hero_id: usize) -> Vec<(usize, IPoint)> {
        let mut ret = vec![];

        let enable = |p: &IPoint| board.player.base.distance(p) > DETECT_BASE_RADIUS + 3000;

        // 3点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                for m3 in board.monster_list.iter().filter(|m| m.id != m1.id && m.id != m2.id) {
                    if m1.pos.distance(&m2.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m1.pos.distance(&m3.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m2.pos.distance(&m3.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                    {
                        // m1, m2, m3 間の各距離が全部 3/2R 以内なら、3点の重心に行くと3体に当たる
                        let middle = (m1.pos + m2.pos + m3.pos) / 3;
                        if enable(&middle) {
                            ret.push((3, middle));
                        }
                    }
                }
            }
        }

        // 2点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                if m1.pos.distance(&m2.pos) <= 2 * ATTACK_HIT_RADIUS {
                    let middle = (m1.pos + m2.pos) / 2;
                    if enable(&middle) {
                        ret.push((2, middle));
                    }
                }
            }
        }

        // 1点 hit
        for m1 in board.monster_list.iter() {
            if enable(&m1.pos) {
                ret.push((1, m1.pos));
            }
        }
        ret
    }

    fn shortest_move(&self, m: &Monster, pos: &IPoint) -> (i32, IPoint) {
        // FIXME: 共通化
        (1, m.next_pos())
    }

    /// 攻撃したい target は含んだ状態で、可能な限り hit が多い位置を探索する
    fn enumerate_multiple_hit_with_target(
        &self,
        board: &Board,
        hero_id: usize,
        target: &Monster,
    ) -> Vec<(usize, IPoint)> {
        let mut ret = vec![];

        // 3点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                for m3 in board.monster_list.iter().filter(|m| m.id != m1.id && m.id != m2.id) {
                    if m1.pos.distance(&m2.next_pos()) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m1.pos.distance(&m3.next_pos()) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m2.pos.distance(&m3.next_pos()) <= 3 * ATTACK_HIT_RADIUS / 2
                        && (m1.id == target.id || m2.id == target.id || m3.id == target.id)
                    {
                        // m1, m2, m3 間の各距離が全部 3/2R 以内なら、3点の重心に行くと3体に当たる
                        let middle = (m1.next_pos() + m2.next_pos() + m3.next_pos()) / 3;
                        ret.push((3, middle));
                    }
                }
            }
        }

        // 2点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                if m1.pos.distance(&m2.pos) <= 2 * ATTACK_HIT_RADIUS && (m1.id == target.id || m2.id == target.id) {
                    let middle = (m1.pos + m2.pos) / 2;
                    ret.push((2, middle));
                }
            }
        }

        // 1点 hit
        for m1 in board.monster_list.iter() {
            if m1.id == target.id {
                ret.push((1, m1.pos));
            }
        }
        ret
    }
}

#[derive(Clone)]
struct AttackerGoHomeInfo {
    controlled_monster_id: HashSet<i32>,
}

impl AttackerGoHomeInfo {
    fn new() -> AttackerGoHomeInfo {
        AttackerGoHomeInfo {
            controlled_monster_id: HashSet::new(),
        }
    }
}

#[derive(Clone)]
enum AttackerInnerState {
    GoHome(AttackerGoHomeInfo),
    Attack,
}

#[derive(Clone)]
struct AttackerInfo {
    home: [IPoint; 2],
    idle_counter: usize,
    home_shifted: bool,
    inner_state: AttackerInnerState,
}

impl AttackerInfo {
    fn new() -> AttackerInfo {
        AttackerInfo {
            home: [
                IPoint {
                    x: MAX_X - 5000,
                    y: MAX_Y - 1000,
                },
                IPoint {
                    x: MAX_X - 5000,
                    y: MAX_Y - 2000,
                },
            ],
            idle_counter: 0,
            home_shifted: false,
            inner_state: AttackerInnerState::GoHome(AttackerGoHomeInfo::new()),
        }
    }

    fn action(&mut self, board: &Board, hero_list: &Vec<usize>, solver: &mut SolverState) -> Vec<(usize, Action)> {
        // FIXME: 実装
        // - 速攻が成立するかを判定したい
        //   - 3人の hero の所在がすべて見えている
        //   - ソロのドリブルを相手が止める手段があるか否かを判定

        let ret = match &mut self.inner_state {
            AttackerInnerState::GoHome(info) => {
                // 所定の位置に移動する
                // 移動するついでに monster を見かけたら、hero[0] の home に向けて control しておく
                let mut ret = vec![];
                for hero_id in hero_list.iter() {
                    let hero = &board.player.hero_list[*hero_id];
                    if let Some(m) = board
                        .monster_list
                        .iter()
                        .filter(|m| {
                            m.pos.in_range(&hero.pos, CONTROL_RADIUS)
                                && !info.controlled_monster_id.contains(&m.id)
                                && hero.pos.distance2(&board.opponent.base) < m.pos.distance2(&board.opponent.base)
                        })
                        .next()
                    {
                        info.controlled_monster_id.insert(m.id);
                        ret.push((
                            *hero_id,
                            Action::Control {
                                entity_id: m.id,
                                point: self.home[0],
                                message: format!("[at]contorl for future"),
                            },
                        ));
                    } else {
                        ret.push((
                            *hero_id,
                            Action::Move {
                                point: self.home[*hero_id],
                                message: format!("[at]go home"),
                            },
                        ));
                    }
                }
                ret
            }
            AttackerInnerState::Attack => {
                // - 以下の行動列が敵に邪魔されうるか否かを判定したい
                //   - 味方 hero 1 が前方に monster を打つ
                //   - 味方 hero 2 が所定の場所に (move / wind / control)
                //   - 味方 hero 1 & 2 が WIND !
                // - 自由変数
                //   1. どの monster を選択するか
                //     - hero 1 との距離が WIND 圏内
                //     - 敵 base からの距離が 6900 以内
                //   2. hero 1 が初手でどの位置に monster を投げるか
                //     - hero 2 が1手の move で 2手目の WIND を打てる範囲内
                //     - hero 1 が2手目の WIND を打てる範囲内(hero2からの距離が WIND_RAD + HERO_VELOCITY 以内)
                //     - 敵 base からの距離が 4700 以内
                //     - 敵味方関係なく hero の攻撃範囲外である
                //     - [memo] hero 1 がどこに移動するかについては、上記を満たせば適当な場所に移動すればよさそう
                // - 原理的に間に合うか否かの判定
                //   - 1手目を防がれなければ2手目を防ぎようがない
                //   - ということは、1手目を防げない程遠い位置にいれば OK
                //     - 1手目の hero の WIND 圏外にいる
                // - 探索アルゴリズム
                //
                vec![
                    (
                        0,
                        Action::Move {
                            point: self.home[0],
                            message: format!("go home"),
                        },
                    ),
                    (
                        1,
                        Action::Move {
                            point: self.home[1],
                            message: format!("go home"),
                        },
                    ),
                ]
            }
        };

        if let AttackerInnerState::GoHome(_) = self.inner_state {
            if hero_list
                .iter()
                .all(|hero_id| self.home[*hero_id].distance2(&board.player.hero_list[*hero_id].pos) == 0)
            {
                self.inner_state = AttackerInnerState::Attack;
            }
        }

        ret
    }
}

#[derive(Copy, Clone)]
struct DefenderInfo {
    home: IPoint,
}

impl DefenderInfo {
    fn new() -> DefenderInfo {
        DefenderInfo {
            home: IPoint { y: 3000, x: 3000 },
        }
    }
    fn action(&mut self, board: &Board, hero_id: usize, solver: &mut SolverState) -> Action {
        let hero = &board.player.hero_list[hero_id];

        // base に一番近いやつを殴り続ける
        let candidate = board
            .monster_list
            .iter()
            .filter(|m| m.pos.distance(&board.player.base) <= DETECT_BASE_RADIUS)
            .min_by_key(|m| m.pos.distance(&board.player.base));

        if let Some(monster) = candidate {
            let around_alive = board
                .monster_list
                .iter()
                .filter(|m| m.pos.distance(&hero.pos) <= WIND_RADIUS && m.health > 2)
                .count()
                > 0;
            // 既に monster に追いついていて、1撃じゃ死なない monster に goal されそうなら、 WIND!
            if around_alive
                && monster.pos.distance(&board.player.base) <= THREASHOLD_BASE_DAMAGE_RADIUS + MAX_MONSTER_VELOCITY
                && solver.can_spell(board, true)
            {
                let point = hero.pos * 2 - board.player.base;
                solver.spell_count += 1;
                Action::Wind {
                    point,
                    message: format!("[def]wind"),
                }
            } else if hero.shield_life == 0 && solver.can_spell(board, true) && solver.is_opponent_speller[hero_id] {
                solver.spell_count += 1;
                Action::Shield {
                    entity_id: hero.id,
                    message: format!("shield self!"),
                }
            } else if hero.pos.distance(&monster.pos) <= ATTACK_HIT_RADIUS {
                // 攻撃が当たるなら、マナの収集効率が良い場所を見つける
                let candidate = self.enumerate_multiple_hit_with_target(board, hero_id, monster);
                assert!(!candidate.is_empty());

                Action::Move {
                    point: candidate[0].1,
                    message: format!("[def]{}attack", candidate[0].0),
                }
            } else {
                let (_, point) = self.shortest_move(&monster, &hero.pos);
                Action::Move {
                    point,
                    message: format!("[def]shortest"),
                }
            }
        } else {
            Action::Move {
                point: self.home,
                message: format!("[def]go home"),
            }
        }
    }

    fn shortest_move(&self, m: &Monster, pos: &IPoint) -> (i32, IPoint) {
        // FIXME: 共通化
        (1, m.next_pos())
    }

    /// 攻撃したい target は含んだ状態で、可能な限り hit が多い位置を探索する
    fn enumerate_multiple_hit_with_target(
        &self,
        board: &Board,
        hero_id: usize,
        target: &Monster,
    ) -> Vec<(usize, IPoint)> {
        let mut ret = vec![];

        // 3点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                for m3 in board.monster_list.iter().filter(|m| m.id != m1.id && m.id != m2.id) {
                    if m1.pos.distance(&m2.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m1.pos.distance(&m3.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                        && m2.pos.distance(&m3.pos) <= 3 * ATTACK_HIT_RADIUS / 2
                        && (m1.id == target.id || m2.id == target.id || m3.id == target.id)
                    {
                        // m1, m2, m3 間の各距離が全部 3/2R 以内なら、3点の重心に行くと3体に当たる
                        let middle = (m1.pos + m2.pos + m3.pos) / 3;
                        ret.push((3, middle));
                    }
                }
            }
        }

        // 2点 hit
        for m1 in board.monster_list.iter() {
            for m2 in board.monster_list.iter().filter(|m| m.id != m1.id) {
                if m1.pos.distance(&m2.pos) <= 2 * ATTACK_HIT_RADIUS && (m1.id == target.id || m2.id == target.id) {
                    let middle = (m1.pos + m2.pos) / 2;
                    ret.push((2, middle));
                }
            }
        }

        // 1点 hit
        for m1 in board.monster_list.iter() {
            if m1.id == target.id {
                ret.push((1, m1.pos));
            }
        }
        ret
    }
}

#[derive(Clone)]
enum HeroState {
    CollectMana(CollectManaInfo),
    Attacker(AttackerInfo),
    Defender(DefenderInfo),
}

#[derive(Clone, Debug)]
enum OpponentStrategyType {
    NotEstimated,
    CompletelyDefense,
}

#[derive(Clone, Debug)]
struct SolverState {
    // 相手が自分の hero に対して一度でも妨害呪文をかけてきたか
    is_opponent_speller: [bool; 3],
    spell_count: i32,
    // mid fielder が何回 control したか
    // これを見て attacker が妨害工作をするタイミングを決める
    midfielder_countrol_count: i32,

    strategy_changed: bool,

    previous_position: Vec<IPoint>,
}

impl SolverState {
    fn can_spell(&self, board: &Board, has_priority: bool) -> bool {
        if has_priority {
            board.player.mana - self.spell_count * 10 >= 10
        } else {
            board.player.mana - (2 + self.spell_count) * 10 >= 10
        }
    }
}

#[derive(Clone)]
struct HeroGroupState {
    hero_list: Vec<usize>,
    hero_state: HeroState,
}

impl HeroGroupState {
    fn action(&self, board: &Board) -> Vec<(usize, Action)> {
        vec![]
    }
}

#[derive(Clone)]
struct Solver {
    hero_state: Vec<HeroGroupState>,
    solver_state: SolverState,
}

const MAX_X: i32 = 17630;
const MAX_Y: i32 = 9000;
const DETECT_BASE_RADIUS: i32 = 5000;
const WIND_RADIUS: i32 = 1280;
const CONTROL_RADIUS: i32 = 2200;
const SHIELD_RADIUS: i32 = 2200;
const MAX_PLAYER_VELOCITY: i32 = 800;
const MAX_MONSTER_VELOCITY: i32 = 400;
const THREASHOLD_BASE_DAMAGE_RADIUS: i32 = 300;
const PLAYER_DAMAGE: i32 = 2;
const VELOCITY_DIFF: i32 = MAX_PLAYER_VELOCITY - MAX_MONSTER_VELOCITY;
const ATTACK_HIT_RADIUS: i32 = 800;
const HERO_RECOGNIZABLE_RADIUS: i32 = 2200;

impl Solver {
    fn new(base_pos: &IPoint, hero_size: usize) -> Solver {
        Solver {
            hero_state: (0..hero_size)
                .map(|hero_id| HeroGroupState {
                    hero_list: vec![hero_id],
                    hero_state: HeroState::CollectMana(CollectManaInfo::new(base_pos, hero_id)),
                })
                .collect::<Vec<_>>(),
            solver_state: SolverState {
                is_opponent_speller: [false; 3],
                spell_count: 0,
                midfielder_countrol_count: 0,
                strategy_changed: false,
                previous_position: vec![IPoint::new(); 3],
            },
        }
    }

    fn solve(&mut self, board: &Board) -> Vec<Action> {
        let start = Instant::now();

        if board.turn == 1 {
            for (hero_id, hero) in board.player.hero_list.iter().enumerate() {
                self.solver_state.previous_position[hero_id] = hero.pos;
            }
        }

        eprintln!("{:?}", self.solver_state);
        eprintln!("self hero");
        for h in board.player.hero_list.iter() {
            eprintln!("{:?}", h);
        }
        eprintln!("opponent hero");
        for h in board.opponent.hero_list.iter() {
            eprintln!("{:?}", h);
        }
        eprintln!("monster");
        for m in board.monster_list.iter() {
            eprintln!("{} {:?}", m.pos.distance(&board.player.hero_list[2].pos), m);
        }

        self.solver_state.spell_count = 0;

        for (hero_id, hero) in board.player.hero_list.iter().enumerate() {
            // WIND を使われているかは、直前の場所との距離で判断
            if hero.is_controlled
                || !self.solver_state.previous_position[hero_id].in_range(&hero.pos, WIND_RADIUS - 100)
            {
                eprintln!(
                    "prev: {:?}, next: {:?}",
                    self.solver_state.previous_position[hero_id], hero.pos
                );
                self.solver_state.is_opponent_speller[hero_id] = true;
            }
        }

        let mut ret = vec![
            Action::Wait {
                message: "empty".to_string()
            };
            3
        ];

        for group_id in 0..self.hero_state.len() {
            let action_list = match &mut self.hero_state[group_id] {
                HeroGroupState {
                    hero_list,
                    hero_state: HeroState::CollectMana(info),
                } => {
                    vec![(hero_list[0], info.action(board, hero_list[0], &mut self.solver_state))]
                }
                HeroGroupState {
                    hero_list,
                    hero_state: HeroState::Attacker(info),
                } => info.action(board, &hero_list, &mut self.solver_state),
                HeroGroupState {
                    hero_list,
                    hero_state: HeroState::Defender(info),
                } => {
                    vec![(hero_list[0], info.action(board, hero_list[0], &mut self.solver_state))]
                }
            };
            for (hero_id, action) in action_list.into_iter() {
                ret[hero_id] = action;
            }
        }

        // 相手に比べてマナがたくさんある || 十分マナが揃ったら攻撃態勢
        if !self.solver_state.strategy_changed && board.player.mana >= 200
            || (board.player.mana - board.opponent.mana >= 100)
        {
            self.solver_state.strategy_changed = true;
            self.hero_state.clear();
            self.hero_state.push(HeroGroupState {
                hero_list: vec![0, 1],
                hero_state: HeroState::Attacker(AttackerInfo::new()),
            });
            self.hero_state.push(HeroGroupState {
                hero_list: vec![2],
                hero_state: HeroState::Defender(DefenderInfo::new()),
            });
        }

        let elapsed = (Instant::now() - start).as_millis();
        eprintln!("elapsed: {}[ms]", elapsed);

        for (hero_id, hero) in board.player.hero_list.iter().enumerate() {
            self.solver_state.previous_position[hero_id] = hero.pos;
        }
        ret
    }
}

fn main() {
    input_old! {
        line_num: 2,
        base_x: i32,
        base_y: i32,
        heroes_per_player: usize,
    }

    // 色々面倒なので、player が left になるように盤面を点対称に移動させる
    // 出力する座標も、これを見て点対称に移動させる
    let point_symmetry_when_necessary = |p: IPoint| {
        let center = IPoint {
            x: MAX_X / 2,
            y: MAX_Y / 2,
        };
        let is_left = base_x < center.x;

        if is_left {
            p
        } else {
            p.point_symmetry(&center)
        }
    };

    let velocity_symmetry_when_necessary = |v: IPoint| {
        let center = IPoint {
            x: MAX_X / 2,
            y: MAX_Y / 2,
        };
        let is_left = base_x < center.x;
        if is_left {
            v
        } else {
            IPoint { x: -v.x, y: -v.y }
        }
    };

    let mut solver = Solver::new(
        &point_symmetry_when_necessary(IPoint { x: base_x, y: base_y }),
        heroes_per_player,
    );

    // game loop
    for turn in 1.. {
        let mut board = Board {
            player: Player::new(),
            opponent: Player::new(),
            monster_list: vec![],
            turn,
        };
        for i in 0..2 {
            input_old! {
                line_num: 1,
                health: i32,
                mana: i32,
            };
            if i == 0 {
                board.player.health = health;
                board.player.mana = mana;
                board.player.base = point_symmetry_when_necessary(IPoint { x: base_x, y: base_y });
            } else {
                board.opponent.health = health;
                board.opponent.mana = mana;
                board.opponent.base = point_symmetry_when_necessary(IPoint {
                    x: MAX_X - base_x,
                    y: MAX_Y - base_y,
                });
            }
        }

        input_old! {
            line_num: 1,
            entity_count: usize,
        }

        for _i in 0..entity_count {
            input_old! {
                line_num: 1,
                id: i32,
                entity_type: i32,
                x: i32,
                y: i32,
                shield_life: i32,
                is_controlled: i32,
                health: i32,
                vx: i32,
                vy: i32,
                near_base: i32,
                threat_for: i32,
            };
            if entity_type == 0 {
                let monster = Monster {
                    health,
                    v: velocity_symmetry_when_necessary(IPoint { x: vx, y: vy }),
                    threat_state: MonsterThreatState::to_threat_state(near_base, threat_for),
                    id,
                    pos: point_symmetry_when_necessary(IPoint { x, y }),
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.monster_list.push(monster);
            } else if entity_type == 1 {
                let hero = Hero {
                    id,
                    pos: point_symmetry_when_necessary(IPoint { x, y }),
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.player.hero_list.push(hero);
            } else if entity_type == 2 {
                let hero = Hero {
                    id,
                    pos: point_symmetry_when_necessary(IPoint { x, y }),
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.opponent.hero_list.push(hero);
            } else {
                panic!("unknown entity type");
            }
        }

        let action_list = solver.solve(&board);
        assert_eq!(action_list.len(), heroes_per_player);

        for action in action_list.into_iter() {
            match action {
                Action::Wait { message } => println!("WAIT {}", message),
                Action::Move { point, message } => {
                    let point = point_symmetry_when_necessary(point);
                    println!("MOVE {} {} {}", point.x, point.y, message);
                }
                Action::Wind { point, message } => {
                    let point = point_symmetry_when_necessary(point);
                    println!("SPELL WIND {} {} {}", point.x, point.y, message);
                }
                Action::Shield { entity_id, message } => println!("SPELL SHIELD {} {}", entity_id, message),
                Action::Control {
                    entity_id,
                    point,
                    message,
                } => {
                    let point = point_symmetry_when_necessary(point);
                    println!("SPELL CONTROL {} {} {} {}", entity_id, point.x, point.y, message);
                }
            }
        }
    }
}
