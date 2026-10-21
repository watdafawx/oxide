/// How many swings of `damage` break a block with `hp`? swings_to_break(10, 3) -> 4 (10, 7, 4, 1, gone)
pub fn swings_to_break(hp: i32, damage: i32) -> u32 {
    let mut hp = hp;
    let mut swings = 0;
    while hp > 0 {
        hp -= damage;
        swings += 1;
    }
    swings
}

/// How many items does `fuel` smelt, `per_smelt` fuel each? smelts_before_empty(100, 24) -> 4
pub fn smelts_before_empty(fuel: u32, per_smelt: u32) -> u32 {
    let mut fuel = fuel;
    let mut smelts = 0;
    while fuel >= per_smelt {
        fuel -= per_smelt;
        smelts += 1;
    }
    smelts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swings_until_the_block_breaks() {
        assert_eq!(swings_to_break(10, 3), 4);
        assert_eq!(swings_to_break(9, 3), 3);
        assert_eq!(swings_to_break(0, 3), 0, "already broken, no swings");
    }

    #[test]
    fn smelts_until_the_fuel_runs_out() {
        assert_eq!(smelts_before_empty(100, 24), 4);
        assert_eq!(smelts_before_empty(23, 24), 0);
        assert_eq!(smelts_before_empty(48, 24), 2);
    }
}
