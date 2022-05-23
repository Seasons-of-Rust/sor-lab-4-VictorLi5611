use std::{fmt};

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {

        let mut this_wins = 0;
        let mut other_wins = 0;

        for this_card in self.cards.iter(){
            for other_card in other.cards.iter(){
                let result = this_card.fight(other_card);
                match result{
                    FightResult::Win => this_wins += 1,
                    FightResult::Loss => other_wins += 1,
                    _ =>{}
                }
            }
        }

        match (this_wins, other_wins){
            (this_wins, other_wins) if this_wins < other_wins => FightResult::Loss,
            (this_wins, other_wins) if this_wins > other_wins => FightResult::Win,
            (this_wins, other_wins) if this_wins == other_wins => FightResult::Tie,
            (_,_) => unreachable!(),
        }


    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}
