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

struct Board {
    player: Player,
    opponent: Player,
    monster_list: Vec<Monster>,
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

#[derive(Clone, Copy)]
enum Action {
    Wait,
    Move { point: Point },
}

impl Action {
    fn print(&self) {
        match *self {
            Action::Wait => println!("WAIT"),
            Action::Move { point } => println!("MOVE {} {}", point.x, point.y),
        }
    }
}

struct Solver {
    // このターンで向かうべき位置があれば覚えておく
    target: Vec<Option<Point>>,
}

impl Solver {
    fn new(hero_size: usize) -> Solver {
        Solver {
            target: vec![None; hero_size],
        }
    }

    fn hero_size(&self) -> usize {
        self.target.len()
    }

    fn solve(&mut self, board: &Board) -> Vec<Action> {
        let mut ret = vec![Action::Wait; self.hero_size()];

        let mut is_already_target = vec![false; board.monster_list.len()];

        // 既にtarget があるなら、見つけて登録
        for hero_id in 0..self.hero_size() {
            if let Some(target) = self.target[hero_id] {
                let mut find = false;
                for (monster_index, monster) in board.monster_list.iter().enumerate() {
                    if monster.threat_state.threat_player() && monster.pos == target {
                        is_already_target[monster_index] = true;
                        self.target[hero_id] = Some(monster.next_pos());
                        ret[hero_id] = Action::Move {
                            point: monster.next_pos(),
                        };
                        find = true;
                        break;
                    }
                }
                if !find {
                    // 消滅しているので、解除
                    self.target[hero_id] = None;
                }
            }
        }

        // 先頭から順に、近くの危険生物を見つけて、排除
        for hero_id in 0..self.hero_size() {
            if let None = self.target[hero_id] {
                let hero = &board.player.hero_list[hero_id];

                let mut nearest_monster = std::usize::MAX;
                let mut nearest_distance = std::i32::MAX;

                for (monster_index, monster) in board.monster_list.iter().enumerate() {
                    let dist = monster.pos.distance2(&hero.pos);
                    if !is_already_target[monster_index]
                        && monster.threat_state.threat_player()
                        && (dist < nearest_distance)
                    {
                        nearest_monster = monster_index;
                        nearest_distance = dist;
                    }
                }
                if nearest_monster < board.monster_list.len() {
                    let monster = &board.monster_list[nearest_monster as usize];
                    self.target[hero_id] = Some(monster.next_pos());
                    ret[hero_id] = Action::Move {
                        point: monster.next_pos(),
                    };
                    is_already_target[nearest_monster] = true;
                } else {
                    ret[hero_id] = Action::Move {
                        point: board.player.base,
                    };
                }
            }
        }

        ret
    }
}

const MAX_X: i32 = 17630;
const MAX_Y: i32 = 9000;

fn main() {
    input_old! {
        line_num: 2,
        base_x: i32,
        base_y: i32,
        heroes_per_player: usize,
    }

    let mut solver = Solver::new(heroes_per_player);

    // game loop
    loop {
        let mut board = Board {
            player: Player::new(),
            opponent: Player::new(),
            monster_list: vec![],
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
                board.player.base = Point { x: base_x, y: base_y };
            } else {
                board.opponent.health = health;
                board.opponent.mana = mana;
                board.opponent.base = Point {
                    x: MAX_X - base_x,
                    y: MAX_Y - base_y,
                };
            }
        }

        input_old! {
            line_num: 1,
            entity_count: usize,
        }

        for i in 0..entity_count {
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
                    v: Point { x: vx, y: vy },
                    threat_state: MonsterThreatState::to_threat_state(near_base, threat_for),
                    id,
                    pos: Point { x, y },
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.monster_list.push(monster);
            } else if entity_type == 1 {
                let hero = Hero {
                    id,
                    pos: Point { x, y },
                    shield_life,
                    is_controlled: is_controlled == 1,
                };
                board.player.hero_list.push(hero);
            } else if entity_type == 2 {
                let hero = Hero {
                    id,
                    pos: Point { x, y },
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

        for action in action_list.iter() {
            action.print();
        }
    }
}
