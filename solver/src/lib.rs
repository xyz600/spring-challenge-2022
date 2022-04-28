use std::{collections::HashSet, time::Instant};

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

#[derive(Debug)]
struct Hero {
    id: i32,
    pos: Point,
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
    Wait {
        message: String,
    },
    Move {
        point: Point,
        message: String,
    },
    Wind {
        point: Point,
        message: String,
    },
    Shield {
        entity_id: i32,
        message: String,
    },
    Control {
        entity_id: i32,
        point: Point,
        message: String,
    },
}

#[derive(PartialEq, Copy, Clone)]
struct CollectManaInfo {
    home: Point,
}

impl CollectManaInfo {
    fn calculate_home_to_collect_mana(base_pos: &Point, hero_id: usize) -> Point {
        if hero_id == 0 {
            Point {
                x: MAX_X * 6 / 10,
                y: MAX_Y * 6 / 10,
            }
        } else if hero_id == 1 {
            Point {
                x: MAX_X * 4 / 10,
                y: MAX_Y * 4 / 10,
            }
        } else {
            let rad = std::f64::consts::PI / 4.0;
            let radius = DETECT_BASE_RADIUS as f64;
            let dx = (rad.cos() * radius) as i32;
            let dy = (rad.sin() * radius) as i32;
            *base_pos + Point { x: dx, y: dy }
        }
    }

    fn new(base_pos: &Point, hero_id: usize) -> CollectManaInfo {
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
                } else if hero.shield_life == 0 && solver.can_spell(board, true) && solver.is_opponent_speller {
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

            if hero.shield_life == 0 && solver.can_spell(board, false) && solver.is_opponent_speller {
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

    fn enumerate_multiple_hit(&self, board: &Board, hero_id: usize) -> Vec<(usize, Point)> {
        let mut ret = vec![];

        let enable = |p: &Point| board.player.base.distance(p) > DETECT_BASE_RADIUS + 3000;

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

    fn shortest_move(&self, m: &Monster, pos: &Point) -> (i32, Point) {
        // FIXME: 共通化
        (1, m.next_pos())
    }

    /// 攻撃したい target は含んだ状態で、可能な限り hit が多い位置を探索する
    fn enumerate_multiple_hit_with_target(
        &self,
        board: &Board,
        hero_id: usize,
        target: &Monster,
    ) -> Vec<(usize, Point)> {
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

#[derive(PartialEq, Copy, Clone)]
struct AttackerInfo {
    home: Point,
    idle_counter: usize,
    home_shifted: bool,
}

impl AttackerInfo {
    fn new() -> AttackerInfo {
        AttackerInfo {
            home: Point {
                x: MAX_X - 2000,
                y: MAX_Y - 2000,
            },
            idle_counter: 0,
            home_shifted: false,
        }
    }

    fn action(&mut self, board: &Board, hero_id: usize, solver: &mut SolverState) -> Action {
        let hero = &board.player.hero_list[hero_id];

        // home にしばらく居座っていたら、手前で防御されている可能性が高いので、home を少し上にずらす
        if !self.home_shifted && self.home == hero.pos {
            self.idle_counter += 1;

            if self.idle_counter == 10 {
                self.home_shifted = true;
                self.home.x -= 1000;
                self.home.y -= 1000;
            }
        } else {
            self.idle_counter = 0;
        }

        if hero.shield_life == 0 && solver.can_spell(board, false) && solver.is_opponent_speller {
            solver.spell_count += 1;
            Action::Shield {
                entity_id: hero.id,
                message: format!("[at]shield self!"),
            }
        } else if self.home.distance(&hero.pos) > 4000 {
            Action::Move {
                point: self.home,
                message: format!("[at]home 1"),
            }
        } else if board
            .opponent
            .hero_list
            .iter()
            .filter(|op_h| {
                op_h.pos.distance(&board.opponent.base) < DETECT_BASE_RADIUS + 1000
                    && op_h.shield_life == 0
                    && board
                        .monster_list
                        .iter()
                        .map(|m| m.pos.distance(&board.opponent.base))
                        .min()
                        .unwrap_or(std::i32::MAX)
                        < op_h.pos.distance(&board.opponent.base) // 敵 hero より内側に monster が居座っている
                    && board
                        .monster_list
                        .iter()
                        .filter(|m| m.threat_state.threat_opponent() && m.health >= 6)
                        .count()
                        > 0
            })
            .count()
            > 0
        {
            // 相手陣地に十分な monster がいるので、妨害が最善
            // 相手陣地付近にいる 敵 hero を遠ざけ続ける
            let op_hero = board
                .opponent
                .hero_list
                .iter()
                .filter(|op_h| op_h.shield_life == 0)
                .min_by_key(|op_h| op_h.pos.distance(&board.opponent.base))
                .unwrap();
            if op_hero.pos.distance(&hero.pos) <= CONTROL_RADIUS && solver.can_spell(board, false) {
                solver.spell_count += 1;
                Action::Control {
                    entity_id: op_hero.id,
                    point: op_hero.pos * 2 - board.opponent.base,
                    message: format!("[at]op control"),
                }
            } else {
                Action::Move {
                    point: op_hero.pos,
                    message: format!("[at]op_h shortest"),
                }
            }
        } else if let Some(monster) = board
            .monster_list
            .iter()
            .filter(|m| {
                m.shield_life == 0
                    && m.threat_state.threat_opponent()
                    && m.pos.distance(&board.opponent.base) < DETECT_BASE_RADIUS
            })
            .min_by_key(|m| hero.pos.distance(&m.pos))
        {
            // 相手ゴール前ががら空き
            if board
                .opponent
                .hero_list
                .iter()
                .filter(|h| h.pos.distance(&board.opponent.base) < DETECT_BASE_RADIUS)
                .count()
                == 0
            {
                if hero.pos.distance(&monster.pos) <= WIND_RADIUS && solver.can_spell(board, false) {
                    // wind の方が到達速度が速そう
                    solver.spell_count += 1;
                    Action::Wind {
                        point: board.opponent.base,
                        message: format!("[at]shield"),
                    }
                } else {
                    let (_, point) = self.shortest_move(&monster, &hero.pos);
                    Action::Move {
                        point,
                        message: format!("[at]shortest"),
                    }
                }
            } else {
                // 守備がいるなら、shield を張りたい
                if hero.pos.distance(&monster.pos) <= SHIELD_RADIUS && solver.can_spell(board, false) {
                    // そうでなければ shield で包む
                    solver.spell_count += 1;
                    Action::Shield {
                        entity_id: monster.id,
                        message: format!("[at]shield"),
                    }
                } else {
                    let (_, point) = self.shortest_move(&monster, &hero.pos);
                    Action::Move {
                        point,
                        message: format!("[at]shortest"),
                    }
                }
            }
        } else {
            // 敵が見つけられなかったら、go home
            Action::Move {
                point: self.home,
                message: format!("[at]home 2"),
            }
        }
    }

    fn shortest_move(&self, m: &Monster, pos: &Point) -> (i32, Point) {
        // FIXME: 共通化
        (1, m.next_pos())
    }
}

#[derive(PartialEq, Clone)]
struct MidFielderInfo {
    home: Point,
    assisted: HashSet<i32>,
}

impl MidFielderInfo {
    fn new() -> MidFielderInfo {
        MidFielderInfo {
            home: Point {
                y: MAX_Y - 5500,
                x: MAX_X - 5500,
            },
            assisted: HashSet::new(),
        }
    }

    fn action(&mut self, board: &Board, hero_id: usize, solver: &mut SolverState) -> Action {
        let hero = &board.player.hero_list[hero_id];

        if hero.shield_life == 0 && solver.can_spell(board, false) && solver.is_opponent_speller {
            // 敵が邪魔をしてくるやつで、シールドが切れたら張り直す
            solver.spell_count += 1;
            Action::Shield {
                entity_id: hero.id,
                message: format!("[as]shield self!"),
            }
        } else if self.home.distance(&hero.pos) > 4000 {
            // 前線に十分近くなければ、前線へ移動を優先
            Action::Move {
                point: self.home,
                message: format!("[as]home 1"),
            }
        } else if let Some(target) = board
            .monster_list
            .iter()
            .filter(|m| {
                DETECT_BASE_RADIUS <= m.pos.distance(&board.opponent.base)
                    && hero.pos.distance(&m.pos) <= HERO_RECOGNIZABLE_RADIUS
                    && !m.threat_state.threat_opponent()
                    && m.shield_life == 0
                    && board
                        .opponent
                        .hero_list
                        .iter()
                        .filter(|op_h| op_h.pos.distance(&m.pos) <= ATTACK_HIT_RADIUS)
                        .count()
                        == 0
                    && m.health >= 10
            })
            .min_by_key(|m| hero.pos.distance(&m.pos))
        {
            // CONTROL 中に wind されると、方向を変えないので control が無駄になってしまう

            if hero.pos.distance(&target.pos) < CONTROL_RADIUS && solver.can_spell(board, false) {
                solver.spell_count += 1;

                let edge_list = [
                    Point {
                        x: MAX_X + MAX_MONSTER_VELOCITY - DETECT_BASE_RADIUS,
                        y: MAX_Y - MAX_MONSTER_VELOCITY,
                    },
                    Point {
                        x: MAX_X - MAX_MONSTER_VELOCITY,
                        y: MAX_Y + MAX_MONSTER_VELOCITY - DETECT_BASE_RADIUS,
                    },
                ];

                let goal = edge_list.iter().min_by_key(|p| p.distance(&target.pos)).unwrap();
                // control で送る
                solver.midfielder_countrol_count += 1;
                Action::Control {
                    entity_id: target.id,
                    point: *goal,
                    message: format!("[as]control"),
                }
            } else {
                // 近い monster に近づく
                let (turn, point) = self.shortest_move(&target, &hero.pos);
                Action::Move {
                    point,
                    message: format!("[as]shortest"),
                }
            }
        } else {
            Action::Move {
                point: self.home,
                message: format!("[as]home 2"),
            }
        }
    }

    fn shortest_move(&self, m: &Monster, pos: &Point) -> (i32, Point) {
        (1, m.next_pos())
    }
}

#[derive(PartialEq, Copy, Clone)]
struct DefenderInfo {
    home: Point,
}

impl DefenderInfo {
    fn new() -> DefenderInfo {
        DefenderInfo {
            home: Point { y: 3000, x: 3000 },
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
            } else if hero.shield_life == 0 && solver.can_spell(board, true) && solver.is_opponent_speller {
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

    fn shortest_move(&self, m: &Monster, pos: &Point) -> (i32, Point) {
        // FIXME: 共通化
        (1, m.next_pos())
    }

    /// 攻撃したい target は含んだ状態で、可能な限り hit が多い位置を探索する
    fn enumerate_multiple_hit_with_target(
        &self,
        board: &Board,
        hero_id: usize,
        target: &Monster,
    ) -> Vec<(usize, Point)> {
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

#[derive(PartialEq, Clone)]
enum HeroState {
    CollectMana(CollectManaInfo),
    Attacker(AttackerInfo),
    MidFielder(MidFielderInfo),
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
    is_opponent_speller: bool,
    spell_count: i32,
    // mid fielder が何回 control したか
    // これを見て attacker が妨害工作をするタイミングを決める
    midfielder_countrol_count: i32,

    strategy_changed: bool,

    // 相手の戦略概要の推測
    opponent_strategy: OpponentStrategyType,

    prev_hero_pos: Vec<Point>,
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
pub struct Solver {
    hero_state: Vec<HeroState>,
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
    fn hero_size(&self) -> usize {
        self.hero_state.len()
    }

    pub fn new(base_pos: &Point, hero_size: usize) -> Solver {
        Solver {
            hero_state: (0..hero_size)
                .map(|hero_id| HeroState::CollectMana(CollectManaInfo::new(base_pos, hero_id)))
                .collect::<Vec<_>>(),
            solver_state: SolverState {
                is_opponent_speller: false,
                spell_count: 0,
                midfielder_countrol_count: 0,
                strategy_changed: false,
                opponent_strategy: OpponentStrategyType::NotEstimated,
                prev_hero_pos: vec![Point { x: 0, y: 0 }; 3],
            },
        }
    }

    pub fn solve(&mut self, board: &Board) -> Vec<Action> {
        let start = Instant::now();

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

        if board.turn == 1 {
            for hero_id in 0..3 {
                self.solver_state.prev_hero_pos[hero_id] = board.player.hero_list[hero_id].pos;
            }
        }

        for (hero_id, hero) in board.player.hero_list.iter().enumerate() {
            if hero.is_controlled {
                self.solver_state.is_opponent_speller = true;
            }
            // WIND を使われた
            if self.solver_state.prev_hero_pos[hero_id].distance(&hero.pos) > 1200 {
                self.solver_state.is_opponent_speller = true;
            }
        }

        let ret = (0..self.hero_size())
            .map(|hero_id| -> Action {
                match &mut self.hero_state[hero_id] {
                    HeroState::CollectMana(info) => info.action(board, hero_id, &mut self.solver_state),
                    HeroState::Attacker(info) => info.action(board, hero_id, &mut self.solver_state),
                    HeroState::MidFielder(info) => info.action(board, hero_id, &mut self.solver_state),
                    HeroState::Defender(info) => info.action(board, hero_id, &mut self.solver_state),
                }
            })
            .collect::<Vec<_>>();

        // 相手に比べてマナがたくさんある || 十分マナが揃ったら攻撃態勢
        if !self.solver_state.strategy_changed && board.player.mana >= 200
            || (board.player.mana - board.opponent.mana >= 100)
        {
            self.solver_state.strategy_changed = true;
            self.hero_state[0] = HeroState::Attacker(AttackerInfo::new());
            self.hero_state[1] = HeroState::MidFielder(MidFielderInfo::new());
            self.hero_state[2] = HeroState::Defender(DefenderInfo::new());
        }

        let elapsed = (Instant::now() - start).as_millis();
        eprintln!("elapsed: {}[ms]", elapsed);

        for hero_id in 0..3 {
            self.solver_state.prev_hero_pos[hero_id] = board.player.hero_list[hero_id].pos;
        }

        ret
    }
}
