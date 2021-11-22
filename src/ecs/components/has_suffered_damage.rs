use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HasSufferedDamage {
    pub amount: Vec<i32>,
}

impl HasSufferedDamage {
    pub fn new_damage(store: &mut WriteStorage<HasSufferedDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = HasSufferedDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }

    pub fn get_total(&self) -> i32 {
        self.amount.iter().sum::<i32>()
    }
}
