use std::iter;

use super::day::*;

pub struct Instance;

struct Weapon {
    cost: u16,
    damage: u8,
}

struct Armour {
    cost: u16,
    armour: u8,
}

#[derive(PartialEq, Eq)]
struct Ring {
    cost: u16,
    damage: u8,
    armour: u8,
}

struct Shop {
    weapons: Vec<Weapon>,
    armours: Vec<Armour>,
    rings: Vec<Ring>,
}

impl Shop {
    fn new() -> Self {
        let weapons = vec![
            Weapon { cost: 8, damage: 4 },
            Weapon {
                cost: 10,
                damage: 5,
            },
            Weapon {
                cost: 25,
                damage: 6,
            },
            Weapon {
                cost: 40,
                damage: 7,
            },
            Weapon {
                cost: 74,
                damage: 8,
            },
        ];
        let armours = vec![
            Armour {
                cost: 13,
                armour: 1,
            },
            Armour {
                cost: 31,
                armour: 2,
            },
            Armour {
                cost: 53,
                armour: 3,
            },
            Armour {
                cost: 75,
                armour: 4,
            },
            Armour {
                cost: 102,
                armour: 5,
            },
        ];
        let rings = vec![
            Ring {
                cost: 25,
                damage: 1,
                armour: 0,
            },
            Ring {
                cost: 50,
                damage: 2,
                armour: 0,
            },
            Ring {
                cost: 100,
                damage: 3,
                armour: 0,
            },
            Ring {
                cost: 20,
                damage: 0,
                armour: 1,
            },
            Ring {
                cost: 40,
                damage: 0,
                armour: 2,
            },
            Ring {
                cost: 80,
                damage: 0,
                armour: 3,
            },
        ];
        Shop {
            weapons,
            armours,
            rings,
        }
    }
}

struct Player<'a> {
    health: u8,
    weapon: &'a Weapon,
    armour: Option<&'a Armour>,
    ring1: Option<&'a Ring>,
    ring2: Option<&'a Ring>,
}

impl<'a> Player<'a> {
    fn new(
        weapon: &'a Weapon,
        armour: Option<&'a Armour>,
        ring1: Option<&'a Ring>,
        ring2: Option<&'a Ring>,
    ) -> Self {
        Player {
            health: 100,
            weapon,
            armour,
            ring1,
            ring2,
        }
    }

    fn attack(&self, boss: &mut Boss) {
        let damage = if self.damage() <= boss.armour {
            1
        } else {
            self.damage() - boss.armour
        };

        if boss.health < damage {
            boss.health = 0;
        } else {
            boss.health -= damage;
        }
    }

    fn defend(&mut self, boss: &Boss) {
        let damage = if boss.damage <= self.armour() {
            1
        } else {
            boss.damage - self.armour()
        };

        if self.health < damage {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    fn armour(&self) -> u8 {
        self.armour.map_or(0, |a| a.armour)
            + self.ring1.map_or(0, |a| a.armour)
            + self.ring2.map_or(0, |a| a.armour)
    }

    fn damage(&self) -> u8 {
        self.weapon.damage + self.ring1.map_or(0, |a| a.damage) + self.ring2.map_or(0, |a| a.damage)
    }

    fn bling(self) -> u16 {
        self.weapon.cost
            + self.armour.map_or(0, |a| a.cost)
            + self.ring1.map_or(0, |a| a.cost)
            + self.ring2.map_or(0, |a| a.cost)
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Clone)]
struct Boss {
    health: u8,
    damage: u8,
    armour: u8,
}

impl Boss {
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
        let armour = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let boss_stats = Boss {
            health,
            damage,
            armour,
        };
        let shop = Shop::new();

        let mut cost_min = u16::MAX;
        let mut cost_max = 0;

        let maybe_armour = shop.armours.iter().map(Some).chain(iter::once(None));
        let maybe_ring = shop.rings.iter().map(Some).chain(iter::once(None));

        for weapon in shop.weapons.iter() {
            for armour in maybe_armour.clone() {
                for ring1 in maybe_ring.clone() {
                    for ring2 in maybe_ring.clone() {
                        if ring1.zip(ring2).map_or(false, |(a, b)| a == b) {
                            break;
                        }

                        let mut player = Player::new(weapon, armour, ring1, ring2);
                        let mut boss = boss_stats.clone();

                        while boss.is_alive() && player.is_alive() {
                            player.attack(&mut boss);
                            if boss.is_alive() {
                                player.defend(&boss);
                            }
                        }

                        if player.is_alive() {
                            cost_min = cost_min.min(player.bling());
                        } else {
                            cost_max = cost_max.max(player.bling());
                        }
                    }
                }
            }
        }

        Ok(DayResult {
            part1: cost_min.to_string(),
            part2: Some(cost_max.to_string()),
        })
    }
}
