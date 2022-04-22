// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

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

#[derive(Copy, Clone, PartialEq)]
struct Point {
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

struct Board {
    player: Player,
    opponent: Player,
    monster_list: Vec<Monster>,
    spawn_list: Vec<Point>,
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

    fn print(&self) {
        for m in self.monster_list.iter() {
            eprintln!("monster id: {}, pos: ({} {})", m.id, m.pos.x, m.pos.y);
        }
    }
}

struct Player {
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

struct Hero {
    id: i32,
    pos: Point,
    shield_life: i32,    // not use
    is_controlled: bool, // not use
}

#[derive(Clone, Copy, PartialEq)]
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

struct Monster {
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
enum Action {
    Wait { message: String },
    Move { point: Point, message: String },
    Wind { point: Point, message: String },
}

#[derive(PartialEq, Copy, Clone)]
enum HeroState {
    CollectMana {
        target_monster: Option<i32>,
    },
    Attacker {
        internal_state: AttackerState,
        target_monster: Option<i32>,
    },
    Defender {
        target_monster: Option<i32>,
    },
}

impl HeroState {
    fn target_monster(&self) -> Option<i32> {
        match *self {
            HeroState::CollectMana { target_monster } => target_monster,
            HeroState::Attacker {
                internal_state,
                target_monster,
            } => target_monster,
            HeroState::Defender { target_monster } => target_monster,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum AttackerState {
    InitialMove(Point),
    Target,
    Wind,
}

struct Solver {
    hero_state: Vec<HeroState>,
}

const MAX_X: i32 = 17630;
const MAX_Y: i32 = 9000;
const DETECT_BASE_RADIUS: i32 = 5000;
const WIND_RADIUS: i32 = 1280;
const MAX_PLAYER_VELOCITY: i32 = 800;
const MAX_MONSTER_VELOCITY: i32 = 400;
const THREASHOLD_BASE_DAMAGE_RADIUS: i32 = 300;
const PLAYER_DAMAGE: i32 = 2;
const VELOCITY_DIFF: i32 = MAX_PLAYER_VELOCITY - MAX_MONSTER_VELOCITY;

impl Solver {
    fn new(hero_size: usize) -> Solver {
        Solver {
            hero_state: vec![HeroState::CollectMana { target_monster: None }; hero_size],
        }
    }

    fn hero_size(&self) -> usize {
        self.hero_state.len()
    }

    fn target_undecided(&self, board: &Board, hero_id: usize) -> bool {
        // target が存在しない or target がいるけど既に board にはいない
        if let Some(monster_id) = self.hero_state[hero_id].target_monster() {
            if let Some(_monster) = board.monster(monster_id) {
                false
            } else {
                true
            }
        } else {
            true
        }
    }

    fn calculate_home(base_pos: &Point, hero_id: usize) -> Point {
        let rad = std::f64::consts::PI / 8.0 * ((hero_id + 1) as f64);
        let radius = (DETECT_BASE_RADIUS + 2000) as f64;
        let dx = (rad.cos() * radius) as i32;
        let dy = (rad.sin() * radius) as i32;
        eprintln!("home: {} {}", dx, dy);
        *base_pos + Point { x: dx, y: dy }
    }

    fn solve(&mut self, board: &Board) -> Vec<Action> {
        board.print();

        let mut ret = vec![
            Action::Wait {
                message: "initial wait".to_string()
            };
            self.hero_size()
        ];

        for hero_id in 0..self.hero_size() {
            let target_undecided = self.target_undecided(board, hero_id);
            let hero = &board.player.hero_list[hero_id];

            ret[hero_id] = match &mut self.hero_state[hero_id] {
                HeroState::CollectMana { target_monster } => {
                    // undecided になってるので、一旦消す
                    *target_monster = None;

                    // 脅威になるものを優先に、近くの敵を優先してマナを貯める
                    // FIXME: できるだけたくさんの敵を殴れるポイントに向かう
                    let mut candidates = board.monster_list.iter().collect::<Vec<_>>();
                    if !candidates.is_empty() {
                        candidates.sort_by(|m1, m2| {
                            if m1.threat_state.threat_level() != m2.threat_state.threat_level() {
                                m2.threat_state.threat_level().cmp(&m1.threat_state.threat_level())
                            } else {
                                let d1 = m1.pos.distance2(&board.player.base);
                                let d2 = m2.pos.distance2(&board.player.base);
                                d1.cmp(&d2)
                            }
                        });
                        *target_monster = Some(candidates[0].id);
                    }

                    if let Some(monster_id) = *target_monster {
                        // 次の state に移る
                        const COLLECT_MANA_FINISH_TURN: usize = 80;
                        if board.turn >= COLLECT_MANA_FINISH_TURN {
                            self.hero_state[hero_id] = if hero_id == 0 {
                                HeroState::Defender { target_monster: None }
                            } else if hero_id == 1 {
                                let point = board.opponent.base + Point { x: -6000, y: -1000 };
                                HeroState::Attacker {
                                    internal_state: AttackerState::InitialMove(point),
                                    target_monster: None,
                                }
                            } else {
                                let point = board.opponent.base + Point { x: -1000, y: -6000 };
                                HeroState::Attacker {
                                    internal_state: AttackerState::InitialMove(point),
                                    target_monster: None,
                                }
                            };
                        }
                        Action::Move {
                            point: board.monster(monster_id).unwrap().next_pos(),
                            message: format!("collect mana: target {}", monster_id),
                        }
                    } else {
                        // 敵がいなかったらhome を目指す
                        Action::Move {
                            point: Self::calculate_home(&board.player.base, hero_id),
                            message: format!("collect mana: enemy is none"),
                        }
                    }
                }
                HeroState::Attacker {
                    internal_state,
                    target_monster,
                } => {
                    let mut select_target = || {
                        let mut candidates = board
                            .monster_list
                            .iter()
                            .filter(|m| {
                                !m.threat_state.threat_opponent() && board.opponent.base.distance(&m.pos) <= 8000
                            })
                            .collect::<Vec<_>>();

                        *target_monster = if candidates.is_empty() {
                            None
                        } else {
                            candidates.sort_by_key(|m| m.next_pos().distance2(&hero.pos));
                            Some(candidates[0].id)
                        }
                    };

                    // target が決まってなかったら、選定
                    if target_undecided {
                        select_target();
                    }

                    match internal_state {
                        AttackerState::InitialMove(point) => {
                            let point = *point;
                            // 到着したら、次のターン用に target 指定
                            if hero.pos.distance(&point) < MAX_PLAYER_VELOCITY {
                                *internal_state = AttackerState::Target;
                                select_target();
                            }
                            // 初期配置に移動
                            Action::Move {
                                point,
                                message: format!("attack: init go to ({}, {})", point.x, point.y),
                            }
                        }
                        AttackerState::Target => {
                            if let Some(monster_id) = target_monster {
                                let monster = board.monster(target_monster.unwrap()).unwrap();
                                // wind 圏内なら Wind に遷移
                                if hero.pos.distance(&monster.next_pos()) < WIND_RADIUS - VELOCITY_DIFF {
                                    *internal_state = AttackerState::Wind;
                                }
                                let point = monster.next_pos();
                                Action::Move {
                                    point,
                                    message: format!("attack: target to go ({}, {})", point.x, point.y),
                                }
                            } else {
                                let point = if hero_id == 1 {
                                    board.opponent.base + Point { x: -6000, y: -1000 }
                                } else {
                                    board.opponent.base + Point { x: -1000, y: -6000 }
                                };
                                Action::Move {
                                    point,
                                    message: format!("attack: cannot find enemy: go home ({}, {})", point.x, point.y),
                                }
                            }
                        }
                        AttackerState::Wind => {
                            *internal_state = AttackerState::Target;
                            select_target();

                            // マナが残っていて、Wind 圏内に入ったら Wind !
                            Action::Wind {
                                point: board.opponent.base,
                                message: format!("wind!"),
                            }
                        }
                    }
                }
                HeroState::Defender { target_monster } => {
                    // 脅威になるものを優先に、近くの敵を優先してマナを貯める
                    // FIXME: できるだけたくさんの敵を殴れるポイントに向かう
                    let mut candidates = board.monster_list.iter().collect::<Vec<_>>();
                    assert!(!candidates.is_empty());
                    candidates.sort_by(|m1, m2| {
                        if m1.threat_state.threat_level() != m2.threat_state.threat_level() {
                            m2.threat_state.threat_level().cmp(&m1.threat_state.threat_level())
                        } else {
                            let d1 = m1.pos.distance2(&board.player.base);
                            let d2 = m2.pos.distance2(&board.player.base);
                            d1.cmp(&d2)
                        }
                    });
                    *target_monster = Some(candidates[0].id);

                    let monster = board.monster(target_monster.unwrap()).unwrap();
                    // あと1手で自陣がダメージを喰らうなら、wind
                    if monster.health > PLAYER_DAMAGE
                        && board.player.base.distance(&monster.next_pos()) < THREASHOLD_BASE_DAMAGE_RADIUS
                    {
                        let point = hero.pos * 2 - board.player.base;
                        Action::Wind {
                            point,
                            message: format!("defender: immediate avoidance ({} {})", point.x, point.y),
                        }
                    } else {
                        let point = monster.next_pos();
                        Action::Move {
                            point,
                            message: format!("defender: track target ({} {})", point.x, point.y),
                        }
                    }
                }
            }
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
    let point_symmetry_when_necessary = |p: Point| {
        let center = Point {
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

    let velocity_symmetry_when_necessary = |v: Point| {
        let center = Point {
            x: MAX_X / 2,
            y: MAX_Y / 2,
        };
        let is_left = base_x < center.x;
        if is_left {
            v
        } else {
            Point { x: -v.x, y: -v.y }
        }
    };

    let mut solver = Solver::new(heroes_per_player);

    // game loop
    for turn in 1.. {
        let mut board = Board {
            player: Player::new(),
            opponent: Player::new(),
            monster_list: vec![],
            spawn_list: vec![
                Point {
                    x: MAX_X / 2,
                    y: WIND_RADIUS,
                },
                Point {
                    x: MAX_X / 2 + 4000,
                    y: WIND_RADIUS,
                },
                Point {
                    x: MAX_X / 2,
                    y: MAX_Y - WIND_RADIUS,
                },
                Point {
                    x: MAX_X / 2 - 4000,
                    y: MAX_Y - WIND_RADIUS,
                },
            ],
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
                board.player.base = point_symmetry_when_necessary(Point { x: base_x, y: base_y });
            } else {
                board.opponent.health = health;
                board.opponent.mana = mana;
                board.opponent.base = point_symmetry_when_necessary(Point {
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
                    v: velocity_symmetry_when_necessary(Point { x: vx, y: vy }),
                    threat_state: MonsterThreatState::to_threat_state(near_base, threat_for),
                    id,
                    pos: point_symmetry_when_necessary(Point { x, y }),
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.monster_list.push(monster);
            } else if entity_type == 1 {
                let hero = Hero {
                    id,
                    pos: point_symmetry_when_necessary(Point { x, y }),
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.player.hero_list.push(hero);
            } else if entity_type == 2 {
                let hero = Hero {
                    id,
                    pos: point_symmetry_when_necessary(Point { x, y }),
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
            }
        }
    }
}
