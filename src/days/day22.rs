use std::collections::VecDeque;

use super::day::*;

pub struct Instance;

#[derive(Clone, Debug)]
struct Boss {
    health: u8,
    damage: u8,
    effect_poison: u8,
}

impl Boss {
    fn new(health: u8, damage: u8) -> Self {
        Boss {
            health,
            damage,
            effect_poison: 0,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana_cost(&self) -> u16 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Clone, Debug)]
struct Player {
    mana: u16,
    health: u8,
    mana_used: u16,
    effect_shield: u8,
    effect_recharge: u8,
    hard_mode: bool,
}

impl Player {
    fn new(hard_mode: bool) -> Self {
        Player {
            mana: 500,
            health: 50,
            mana_used: 0,
            effect_shield: 0,
            effect_recharge: 0,
            hard_mode,
        }
    }

    fn can_cast_next(&self, spell: &Spell, boss: &Boss) -> bool {
        let mana = if self.effect_recharge > 0 {
            self.mana + 101
        } else {
            self.mana
        };

        if mana < spell.mana_cost() {
            return false;
        }

        match spell {
            Spell::MagicMissile => true,
            Spell::Drain => true,
            Spell::Shield => self.effect_shield <= 1,
            Spell::Poison => boss.effect_poison <= 1,
            Spell::Recharge => self.effect_recharge <= 1,
        }
    }

    fn cast(&mut self, spell: &Spell, boss: &mut Boss) {
        assert!(self.can_cast_next(spell, boss));
        assert!(self.is_alive());
        assert!(boss.is_alive());

        self.get_armour_and_run_effects(boss);

        if self.hard_mode {
            self.health -= 1;
            if !self.is_alive() {
                return;
            }
        }

        let mana_cost = spell.mana_cost();
        self.mana -= mana_cost;
        self.mana_used += mana_cost;
        match spell {
            Spell::MagicMissile => boss.health -= boss.health.min(4),
            Spell::Drain => {
                boss.health -= boss.health.min(2);
                self.health += 2;
            }
            Spell::Shield => {
                assert!(self.effect_shield == 0);
                self.effect_shield = 6;
            }
            Spell::Poison => {
                assert!(boss.effect_poison == 0);
                boss.effect_poison = 6;
            }
            Spell::Recharge => {
                assert!(self.effect_recharge == 0);
                self.effect_recharge = 5;
            }
        }

        if boss.is_alive() {
            let armour = self.get_armour_and_run_effects(boss);
            let damage = boss.damage - armour;
            self.health -= self.health.min(damage);
        }
    }

    fn get_armour_and_run_effects(&mut self, boss: &mut Boss) -> u8 {
        if self.effect_recharge > 0 {
            self.effect_recharge -= 1;
            self.mana += 101;
        }
        if boss.effect_poison > 0 {
            boss.effect_poison -= 1;
            boss.health -= boss.health.min(3);
        }
        if self.effect_shield > 0 {
            self.effect_shield -= 1;
            7
        } else {
            0
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut lines = lines.iter();
        let health = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let damage = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let easy_player = Player::new(false);
        let hard_player = Player::new(true);

        let all_spells = vec![
            Spell::MagicMissile,
            Spell::Recharge,
            Spell::Poison,
            Spell::Shield,
            Spell::Drain,
        ];

        let mut best_win_easy = u16::MAX;
        let mut best_win_hard = u16::MAX;

        let mut open_states = VecDeque::new();
        open_states.push_back((easy_player, Boss::new(health, damage)));
        open_states.push_back((hard_player, Boss::new(health, damage)));

        while let Some((p, b)) = open_states.pop_front() {
            for spell in &all_spells {
                if p.can_cast_next(spell, &b) {
                    let mut np = p.clone();
                    let mut nb = b.clone();

                    np.cast(spell, &mut nb);

                    let best_win = if np.hard_mode {
                        &mut best_win_hard
                    } else {
                        &mut best_win_easy
                    };

                    if np.is_alive() && np.mana_used < *best_win {
                        if nb.is_alive() {
                            open_states.push_back((np, nb));
                        } else {
                            *best_win = np.mana_used;
                        }
                    }
                }
            }
        }

        Ok(DayResult {
            part1: best_win_easy.to_string(),
            part2: Some(best_win_hard.to_string()),
        })
    }
}
