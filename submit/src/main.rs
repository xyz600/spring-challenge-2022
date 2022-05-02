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
    fn in_board(&self, p: &IPoint) -> bool {
        0 <= p.x && p.x <= MAX_X && 0 <= p.y && p.y <= MAX_Y
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

    fn will_dead(&self, board: &Board) -> bool {
        let damage = board
            .player
            .hero_list
            .iter()
            .chain(board.opponent.hero_list.iter())
            .filter(|h| h.pos.in_range(&self.pos, ATTACK_HIT_RADIUS))
            .count() as i32
            * PLAYER_DAMAGE;

        damage >= self.health
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
                x: MAX_X / 2,
                y: MAX_Y - 1000,
            }
        } else if hero_id == 1 {
            IPoint {
                x: MAX_X / 2 - 4000,
                y: MAX_Y - 1000,
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

        let point = self.home;
        let candidate = self.enumerate_multiple_hit(board, hero_id);

        if candidate.is_empty() {
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
struct AttackerAttractMonsterInfo {
    center: IPoint,
    counter: i32,
    go_home: bool,
}

impl AttackerAttractMonsterInfo {
    fn new() -> AttackerAttractMonsterInfo {
        eprintln!("create gohome info");
        AttackerAttractMonsterInfo {
            // 勧誘する場所
            center: IPoint {
                x: MAX_X - 6500,
                y: MAX_Y - 1000,
            },
            counter: 0,
            go_home: false,
        }
    }
}

#[derive(Clone)]
struct AttackerInfo {
    home: [IPoint; 2],
    go_home: bool,
    counter: i32,
    controlled: HashSet<i32>,
}

impl AttackerInfo {
    fn new() -> AttackerInfo {
        AttackerInfo {
            home: [
                IPoint {
                    x: MAX_X - 6600,
                    y: MAX_Y - 1000,
                },
                IPoint {
                    x: MAX_X - 6000,
                    y: MAX_Y - 1000,
                },
            ],
            go_home: false,
            counter: 0,
            controlled: HashSet::new(),
        }
    }

    fn action(&mut self, board: &Board, hero_list: &Vec<usize>, solver: &mut SolverState) -> Vec<(usize, Action)> {
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

        // FIXME: 実装
        // - 速攻が成立するかを判定したい
        //   - 3人の hero の所在がすべて見えている
        //   - ソロのドリブルを相手が止める手段があるか否かを判定

        // FIXME: マナが切れたら、流石にマナを一定程度貯める動作をしないとね
        // 40 + 20 * (残りhp - 1) 位は貯めたいかな？

        let mut ret = vec![
            Action::Move {
                point: self.home[0],
                message: format!("[at5]go home"),
            },
            Action::Move {
                point: self.home[1],
                message: format!("[at5]go home"),
            },
        ];

        let mut decided = false;

        let hero0 = &board.player.hero_list[0];
        let hero1 = &board.player.hero_list[1];
        eprintln!("{:?} {:?}", hero0, hero1);

        // home に着いたら 勧誘開始
        if hero0.pos.distance2(&self.home[0]) == 0 {
            self.go_home = true;
        }

        // 1. double wind attack の2段目をできるなら採用
        // base からの距離が 4700 未満で、両方の hero が打てるなら、double wind
        let wind_target = board
            .monster_list
            .iter()
            .filter(|m| {
                m.pos.in_range(&board.opponent.base, SECOND_WIND_ATTACK_THREASHOLD)
                    && m.pos.in_range(&hero0.pos, WIND_RADIUS)
                    && m.pos.in_range(&hero1.pos, WIND_RADIUS)
                    && !m.will_dead(board)
                    && board // Wind attack が敵に妨害される可能性があるなら、20マナがもったいないので緊急中止
                        .opponent
                        .hero_list
                        .iter()
                        .all(|op_h| !op_h.pos.in_range(&m.pos, WIND_ATTACK_MARGIN))
            })
            .collect::<Vec<_>>();

        if board.player.mana >= 30 && !wind_target.is_empty() {
            decided = true;
            eprintln!("[at1] p = {:?}", wind_target[0].pos);

            // hero0 も hero1 も WIND できるなら Double Wind !!
            let dir = board.opponent.base - wind_target[0].pos;
            ret[0] = Action::Wind {
                point: hero0.pos + dir,
                message: format!("[at1] Double Wind!!"),
            };
            ret[1] = Action::Wind {
                point: hero1.pos + dir,
                message: format!("[at1] Double Wind!!"),
            };
        }

        // 2. double wind attack の1段目をできるなら採用
        // base からの距離が 6900 未満で、次に base からの距離が 4700 未満かつ両方の hero が打てる位置に置ける場合、single wind
        let target = board
            .monster_list
            .iter()
            .filter(|m| {
                m.health > 6
                    && m.pos.in_range(&hero0.pos, WIND_RADIUS)
                    && m.pos.in_range(&board.opponent.base, FIRST_WIND_ATTACK_THREASHOLD)
            })
            .collect::<Vec<_>>();
        if !decided && !target.is_empty() && board.player.mana >= 30 {
            if let Some((point, h1_point)) = self.decide_wind_target(&board, &target, hero0, hero1) {
                decided = true;
                eprintln!("[at2] p = {:?}", target[0]);

                // hero 0 は WIND する target を決定
                ret[0] = Action::Wind {
                    point,
                    message: format!("[at2] wind for dw"),
                };
                // hero 1 は 次の WIND に備えて位置取り
                ret[1] = Action::Move {
                    point: h1_point,
                    message: format!("[at2] move for dw"),
                }
            }
        }

        // 3. 1手以内で double wind attack の1段目まで持っていける場合は、そのように移動
        // monster を一通り見て、1手先の場所で double wind できそうならそこに移動して強制終了する
        if !decided {
            for turn in 1..6 {
                if decided {
                    continue;
                }
                for m in board.monster_list.iter() {
                    // 近くに敵 hero がいる場合は対象外
                    if m.health <= 6 || decided {
                        continue;
                    }

                    let np = m.pos + m.v * turn;
                    let expected_h0 = np + IPoint { x: 1100, y: 0 };
                    let expected_h1 = expected_h0 + IPoint { x: 600, y: 0 };

                    if np.in_range(&board.opponent.base, FIRST_WIND_ATTACK_THREASHOLD)
                        && !np.in_range(&board.opponent.base, DETECT_BASE_RADIUS)
                        && board.in_board(&np)
                        && expected_h0.in_range(&hero0.pos, MAX_PLAYER_VELOCITY * turn)
                        && expected_h1.in_range(&hero1.pos, MAX_PLAYER_VELOCITY * (turn + 1))
                    {
                        eprintln!("[at3] np = {:?}, turn = {}", np, turn);
                        decided = true;
                        self.counter = 0;
                        self.go_home = false;
                        ret[0] = Action::Move {
                            point: expected_h0,
                            message: format!("[at3] move"),
                        };
                        ret[1] = Action::Move {
                            point: expected_h1,
                            message: format!("[at3] move"),
                        };
                    }
                }

                if turn >= 2 {
                    let turn = turn - 1;
                    // 4. 2手で double wind attack の1段目まで持っていける(うち初手は control)場合は、control
                    // 2手先を読んで、double wind できそうなら control する
                    for m in board.monster_list.iter() {
                        // 近くに敵 hero がいる場合は対象外
                        if decided {
                            break;
                        }
                        // 2手目は control で rad でどこに飛ばせばいいかは全探索
                        for rad in 0..360 {
                            if m.health <= 6 || decided {
                                continue;
                            }
                            let rad = std::f64::consts::PI * 2.0 * (rad as f64) / 360.0;
                            let dir = (FPoint {
                                x: rad.cos(),
                                y: rad.sin(),
                            } * (MAX_MONSTER_VELOCITY as f64))
                                .to::<i32>();
                            let nnp = m.next_pos() + dir * turn;
                            let expected_hero0_pos = nnp + IPoint { x: 1100, y: 0 };
                            let expected_hero1_pos = expected_hero0_pos + IPoint { x: 600, y: 0 };

                            if board.player.mana >= 50
                                && !nnp.in_range(&board.opponent.base, DETECT_BASE_RADIUS)
                                && board.in_board(&nnp)
                                && hero0.pos.in_range(&m.pos, CONTROL_RADIUS)
                                && nnp.in_range(&board.opponent.base, FIRST_WIND_ATTACK_THREASHOLD)
                                && expected_hero0_pos.in_range(&hero0.pos, MAX_PLAYER_VELOCITY * turn)
                                && expected_hero1_pos.in_range(&hero1.pos, MAX_PLAYER_VELOCITY * (turn + 2))
                                && m.health > 10
                                && !self.controlled.contains(&m.id)
                            {
                                self.controlled.insert(m.id);
                                decided = true;
                                eprintln!("[at4] target: {:?}, turn = {}", nnp, turn);
                                ret[0] = Action::Control {
                                    entity_id: m.id,
                                    point: nnp,
                                    message: format!("[at4] control monster"),
                                };
                                ret[1] = Action::Move {
                                    point: expected_hero1_pos,
                                    message: format!("[at4] move"),
                                };
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !decided {
            if !self.go_home {
                // 5. 家に帰っていない場合はそれを優先
                eprintln!("[at5] go home");
            } else {
                // 6. 難しそうならランダムっぽく動いて待つ
                eprintln!("[at6] random walk");
                self.counter += 1;
                let rad = (self.counter * 240) as f64 / 360.0 * std::f64::consts::PI * 2.0;
                let dx = (MAX_PLAYER_VELOCITY as f64 * rad.cos()) as i32;
                let dy = (MAX_PLAYER_VELOCITY as f64 * rad.sin()) as i32;
                let np = hero0.pos + IPoint { x: dx, y: dy };

                ret[0] = Action::Move {
                    point: np,
                    message: format!("[at6] random move"),
                };

                let hero1_pos = np + IPoint { x: 600, y: 0 };
                ret[1] = Action::Move {
                    point: hero1_pos,
                    message: format!("[at6] random move"),
                };
            }
        }

        ret.into_iter().enumerate().collect::<Vec<_>>()
    }

    fn decide_wind_target(
        &self,
        board: &Board,
        monster: &Vec<&Monster>,
        hero0: &Hero,
        hero1: &Hero,
    ) -> Option<(IPoint, IPoint)> {
        let mut best_pos = IPoint::new();
        let mut best_h1_pos = IPoint::new();
        let mut best_eval = 0;
        // 1°刻みで実際に飛ばして、よさそうな場所を採用
        for rad in 0..360 {
            let rad = std::f64::consts::PI * 2.0 * (rad as f64) / 360.0;
            let x = (rad.cos() * WIND_DISTANCE as f64).round() as i32;
            let y = (rad.sin() * WIND_DISTANCE as f64).round() as i32;
            let diff = IPoint { x, y };
            let eval = monster
                .iter()
                .filter(|m| {
                    let np = m.pos + diff;
                    np.in_range(&hero0.pos, WIND_RADIUS)
                        && np.in_range(&hero1.pos, WIND_RADIUS)
                        && np.in_range(&board.opponent.base, SECOND_WIND_ATTACK_THREASHOLD)
                })
                .count();
            if eval > best_eval {
                best_eval = eval;
                best_pos = hero0.pos + diff;
                best_h1_pos = monster[0].pos + diff;
            }
        }
        if best_eval > 0 {
            Some((best_pos, best_h1_pos))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FPoint, Hero, IPoint, Monster, CONTROL_RADIUS, FIRST_WIND_ATTACK_THREASHOLD, MAX_MONSTER_VELOCITY,
        MAX_PLAYER_VELOCITY,
    };

    #[test]
    fn test_rad() {
        let m = Monster {
            id: 0,
            pos: IPoint { x: 10855, y: 6921 },
            shield_life: 0,
            is_controlled: false,
            health: 19,
            v: IPoint { x: 386, y: 102 },
            threat_state: crate::MonsterThreatState::NotThreat,
        };
        let hero0 = Hero {
            id: 0,
            pos: IPoint { x: 11245, y: 8000 },
            shield_life: 0,
            is_controlled: false,
        };
        let hero1 = Hero {
            id: 0,
            pos: IPoint { x: 11845, y: 8000 },
            shield_life: 0,
            is_controlled: false,
        };

        for turn in 1..4 {
            let mut best_eval = 100000000;
            let mut best_rad = 0;
            let mut best_h0 = 0;
            let mut best_h1 = 0;
            for rad in 0..360 {
                let frad = std::f64::consts::PI * 2.0 * (rad as f64) / 360.0;
                let dir = (FPoint {
                    x: frad.cos(),
                    y: frad.sin(),
                } * (MAX_MONSTER_VELOCITY as f64))
                    .to::<i32>();
                let nnp = m.next_pos() + dir * turn;

                let expected_hero0_pos = nnp + IPoint { x: 1100, y: 0 };
                let expected_hero1_pos = expected_hero0_pos + IPoint { x: 600, y: 0 };

                if !hero0.pos.in_range(&m.pos, CONTROL_RADIUS)
                    || !expected_hero0_pos.in_range(&hero0.pos, MAX_PLAYER_VELOCITY * turn)
                    || !expected_hero1_pos.in_range(&hero1.pos, MAX_PLAYER_VELOCITY * (turn + 2))
                    || m.health <= 10
                {
                    continue;
                }

                let eval = expected_hero0_pos.distance(&hero0.pos) + expected_hero1_pos.distance(&hero1.pos);
                if eval < best_eval {
                    best_eval = eval;
                    best_rad = rad;
                    best_h0 = expected_hero0_pos.distance(&hero0.pos);
                    best_h1 = expected_hero1_pos.distance(&hero1.pos);
                }
            }
            eprintln!("turn = {}", turn);
            eprintln!("eval = {}, rad = {}, ", best_eval, best_rad);
            eprintln!("h0 = {}, h1 = {}, ", best_h0, best_h1);
            // && nnp.in_range(&board.player.base, FIRST_WIND_ATTACK_THREASHOLD)
            // && expected_hero0_pos.in_range(&hero0.pos, MAX_PLAYER_VELOCITY * turn)
            // && expected_hero1_pos.in_range(&hero1.pos, MAX_PLAYER_VELOCITY * (turn + 2))
            // && m.health > 10
            eprintln!("----");
        }
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

    // monster の近くに敵 hero がいて、WIND が刺さるならそれを阻止
    fn opponent_wind_enable<'a, 'b>(&'a self, hero_id: usize, board: &'b Board) -> Option<(&'b Monster, &'b Hero)> {
        for m in board
            .monster_list
            .iter()
            .filter(|m| board.player.hero_list[hero_id].pos.in_range(&m.pos, WIND_RADIUS))
        {
            for op_hero in board.opponent.hero_list.iter() {
                if op_hero.pos.in_range(&m.pos, WIND_RADIUS) {
                    return Some((m, op_hero));
                }
            }
        }
        None
    }

    fn action(&mut self, board: &Board, hero_id: usize, solver: &mut SolverState) -> Action {
        let hero = &board.player.hero_list[hero_id];

        // base に一番近いやつを殴り続ける
        let candidate = board
            .monster_list
            .iter()
            .filter(|m| m.pos.distance(&board.player.base) <= DETECT_BASE_RADIUS)
            .min_by_key(|m| m.pos.distance(&board.player.base));

        if let Some((monster, op_hero)) = self.opponent_wind_enable(hero_id, board) {
            let point = hero.pos * 2 - board.player.base;
            solver.spell_count += 1;
            Action::Wind {
                point,
                message: format!("[def]1 wind"),
            }
        } else if let Some(monster) = candidate {
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
                    message: format!("[def]2 wind"),
                }
            // } else if hero.shield_life == 0 && solver.can_spell(board, true) && solver.is_opponent_speller[hero_id] {
            //     solver.spell_count += 1;
            //     Action::Shield {
            //         entity_id: hero.id,
            //         message: format!("shield self!"),
            //     }
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
            // 敵の hero が陣地内にいるなら、そこに近づく
            if let Some(op_h) = board
                .opponent
                .hero_list
                .iter()
                .filter(|op_h| op_h.pos.in_range(&board.player.base, DETECT_BASE_RADIUS))
                .min_by_key(|op_h| op_h.pos.distance(&board.player.base))
            {
                Action::Move {
                    point: op_h.pos,
                    message: format!("[def]go opponent"),
                }
            } else {
                Action::Move {
                    point: self.home,
                    message: format!("[def]go home"),
                }
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
struct SolverState {
    // 相手が自分の hero に対して一度でも妨害呪文をかけてきたか
    is_opponent_speller: [bool; 3],
    spell_count: i32,

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
const WIND_DISTANCE: i32 = 2200;
const WIND_ATTACK_MARGIN: i32 = 0;
const FIRST_WIND_ATTACK_THREASHOLD: i32 = THREASHOLD_BASE_DAMAGE_RADIUS + 3 * WIND_DISTANCE + WIND_ATTACK_MARGIN;
const SECOND_WIND_ATTACK_THREASHOLD: i32 = THREASHOLD_BASE_DAMAGE_RADIUS + 2 * WIND_DISTANCE + WIND_ATTACK_MARGIN;

impl Solver {
    fn new(base_pos: &IPoint, hero_size: usize) -> Solver {
        Solver {
            hero_state: (0..hero_size)
                .map(|hero_id| HeroGroupState {
                    hero_list: vec![hero_id],
                    hero_state: if hero_id == 2 {
                        HeroState::Defender(DefenderInfo::new())
                    } else {
                        HeroState::CollectMana(CollectManaInfo::new(base_pos, hero_id))
                    },
                })
                .collect::<Vec<_>>(),
            solver_state: SolverState {
                is_opponent_speller: [false; 3],
                spell_count: 0,
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

        eprintln!("monster: ");
        for m in board.monster_list.iter() {
            eprintln!(
                "id: {}, pos: {:?}, v: {:?} dist base: {}, dh0: {}, dh1: {}",
                m.id,
                m.pos,
                m.v,
                board.opponent.base.distance(&m.pos),
                board.player.hero_list[0].pos.distance(&m.pos),
                board.player.hero_list[1].pos.distance(&m.pos)
            );
        }
        eprintln!("hero: ");
        for h in board.player.hero_list.iter() {
            eprintln!("id: {}, pos: {:?}", h.id, h.pos,);
        }
        self.solver_state.spell_count = 0;

        for (hero_id, hero) in board.player.hero_list.iter().enumerate() {
            // WIND を使われているかは、直前の場所との距離で判断
            if hero.is_controlled
                || !self.solver_state.previous_position[hero_id].in_range(&hero.pos, WIND_RADIUS - 100)
            {
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
        if !self.solver_state.strategy_changed && board.player.mana >= 150 {
            self.solver_state.strategy_changed = true;
            // 防御だけ残しておく
            self.hero_state.retain(|g| g.hero_list[0] == 2);
            self.hero_state.push(HeroGroupState {
                hero_list: vec![0, 1],
                hero_state: HeroState::Attacker(AttackerInfo::new()),
            });
        } else if self.solver_state.strategy_changed && board.player.mana < 30 {
            // 防御だけ残しておく
            self.hero_state.retain(|g| g.hero_list[0] == 2);
            for hero_id in 0..2 {
                self.hero_state.push(HeroGroupState {
                    hero_list: vec![hero_id],
                    hero_state: HeroState::CollectMana(CollectManaInfo::new(&board.player.base, hero_id)),
                });
            }
        } else if self.solver_state.strategy_changed && board.player.mana >= 40 {
            // 防御だけ残しておく
            self.hero_state.retain(|g| g.hero_list[0] == 2);
            self.hero_state.push(HeroGroupState {
                hero_list: vec![0, 1],
                hero_state: HeroState::Attacker(AttackerInfo::new()),
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
