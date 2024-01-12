use cadency_core::CadencyError;
use rand::Rng;

pub(crate) trait RollDice {
    fn roll(&self) -> i64;
}

struct Dice {
    sides: i64,
}

impl Dice {
    fn new(sides: i64) -> Self {
        Self { sides }
    }
}

impl RollDice for Dice {
    fn roll(&self) -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides) as i64
    }
}

pub(crate) struct Throw {
    dices: Vec<Dice>,
    bonus: i64,
}

impl Throw {
    fn new(dices: Vec<Dice>, bonus: i64) -> Self {
        Self { dices, bonus }
    }

    pub(crate) fn validate(&self) -> Result<(), CadencyError> {
        for dice in &self.dices {
            if dice.sides <= 1 {
                return Err(CadencyError::Command {
                    message: "Amount of sides must be greater then `1`".to_string(),
                });
            } else if dice.sides.gt(&100) {
                return Err(CadencyError::Command {
                    message: "Amount of sides must be at most `100`".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl RollDice for Throw {
    fn roll(&self) -> i64 {
        self.dices.iter().map(|dice| dice.roll()).sum::<i64>() + self.bonus
    }
}

impl std::str::FromStr for Throw {
    type Err = CadencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const UNSUPPORTED_PATTERN_ERROR: &str =
            "Unsupported pattern. Only the following patterns are supported: e.g. `d6`, `2d6`, 2d6+1` or `2d6-1`";

        let has_multiple_dices = !s.starts_with('d')
            && s.contains('d')
            && s.chars()
                .next()
                .ok_or(CadencyError::Command {
                    message: UNSUPPORTED_PATTERN_ERROR.to_string(),
                })?
                .is_ascii_digit();

        let bonus_parser = |throw_str: &str| {
            let has_positive_bonus =
                throw_str.contains('+') && !throw_str.ends_with('+') && !throw_str.starts_with('+');
            let has_negative_bonus =
                throw_str.contains('-') && !throw_str.ends_with('-') && !throw_str.starts_with('-');
            let bonus_sign = if has_positive_bonus {
                '+'
            } else if has_negative_bonus {
                '-'
            } else if !has_positive_bonus && !has_negative_bonus {
                // No bonus - Finish early and return 0
                return Ok(0);
            } else {
                unreachable!("Bonus sign is either + or -")
            };

            throw_str.split(bonus_sign).collect::<Vec<&str>>()[1]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i64>()
                .map(|bonus| if has_negative_bonus { -bonus } else { bonus })
                .map_err(|_| CadencyError::Command {
                    message: UNSUPPORTED_PATTERN_ERROR.to_string(),
                })
        };

        let throw = if has_multiple_dices {
            let amount = s
                .chars()
                .take_while(|c| c.is_ascii_digit() && c != &'d')
                .collect::<String>()
                .parse::<i64>()
                .map_err(|_| CadencyError::Command {
                    message: UNSUPPORTED_PATTERN_ERROR.to_string(),
                })?;

            let dice_sides = s.split('d').collect::<Vec<&str>>()[1]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i64>()
                .map_err(|_| CadencyError::Command {
                    message: UNSUPPORTED_PATTERN_ERROR.to_string(),
                })?;
            let dices = (0..amount)
                .map(|_| Dice::new(dice_sides))
                .collect::<Vec<Dice>>();
            Throw::new(dices, bonus_parser(s)?)
        } else {
            let dice_sides = s
                .replace('d', "")
                .trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i64>()
                .map_err(|_| CadencyError::Command {
                    message: UNSUPPORTED_PATTERN_ERROR.to_string(),
                })?;

            Throw::new(vec![Dice::new(dice_sides)], bonus_parser(s)?)
        };
        Ok(throw)
    }
}

#[cfg(test)]
mod test_throw {
    use super::*;

    #[test]
    fn test_multiple_dices_with_bonus_single_digit() {
        let throw = "2d6+1".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 2);
        assert_eq!(throw.dices[0].sides, 6);
        assert_eq!(throw.dices[1].sides, 6);
        assert_eq!(throw.bonus, 1);
    }

    #[test]
    fn test_multiple_dices_with_negativ_bonus() {
        let throw = "2d6-1".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 2);
        assert_eq!(throw.dices[0].sides, 6);
        assert_eq!(throw.dices[1].sides, 6);
        assert_eq!(throw.bonus, -1);
    }

    #[test]
    fn test_multiple_dices_with_bonus_multiple_digits() {
        let throw = "2d100+10".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 2);
        assert_eq!(throw.dices[0].sides, 100);
        assert_eq!(throw.dices[1].sides, 100);
        assert_eq!(throw.bonus, 10);
    }

    #[test]
    fn test_single_dice_single_digit() {
        let throw = "d6".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 1);
        assert_eq!(throw.dices[0].sides, 6);
        assert_eq!(throw.bonus, 0);
    }

    #[test]
    fn test_single_dice_multiple_digits() {
        let throw = "d100".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 1);
        assert_eq!(throw.dices[0].sides, 100);
        assert_eq!(throw.bonus, 0);
    }

    #[test]
    fn test_multiple_dices_single_digit() {
        let throw = "2d6".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 2);
        assert_eq!(throw.dices[0].sides, 6);
        assert_eq!(throw.dices[1].sides, 6);
        assert_eq!(throw.bonus, 0);
    }

    #[test]
    fn test_multiple_dices_multiple_digits() {
        let throw = "2d100".parse::<Throw>().unwrap();
        assert_eq!(throw.dices.len(), 2);
        assert_eq!(throw.dices[0].sides, 100);
        assert_eq!(throw.dices[1].sides, 100);
        assert_eq!(throw.bonus, 0);
    }
}
