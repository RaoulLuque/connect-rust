use std::vec::Vec;

#[derive(Clone, Debug)]
struct Entry {
    gamestate: u128,
    value: i8,
}

pub struct TranspositionTable {
    entries: Vec<Entry>,
    number_of_true_entries: u32,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            entries: vec![
                Entry {
                    gamestate: 0,
                    value: 0
                };
                // 8388593 entries use about 48 mb of memory
                8388593
            ],
            number_of_true_entries: 0,
        }
    }

    pub fn index(&self, gamestate: u128) -> usize {
        return (gamestate.min(mirror_gamestate(gamestate)) % self.entries.len() as u128) as usize;
    }

    pub fn put(&mut self, gamestate: u128, value: i8) {
        // Use the minimum of the mirrored and normal gamestate as key for both
        let key_gamestate = gamestate.min(mirror_gamestate(gamestate));

        let index = self.index(key_gamestate);
        if let Some(entry) = self.entries.get(index) {
            if entry.gamestate == 0 {
                self.number_of_true_entries += 1;
                println!(
                    "The number of true entries is: {}",
                    self.number_of_true_entries
                );
            } else if entry.gamestate != gamestate && entry.gamestate != mirror_gamestate(gamestate)
            {
                println!("An entry in the transposition table just got overwritten");
            }
        }

        self.entries.insert(
            index,
            Entry {
                gamestate: key_gamestate,
                value,
            },
        )
    }

    pub fn get(&self, gamestate: u128) -> Option<i8> {
        let key_gamestate = gamestate.min(mirror_gamestate(gamestate));
        let index = self.index(key_gamestate);

        if let Some(entry) = self.entries.get(index) {
            if entry.gamestate == key_gamestate {
                println!("The transposition table is used for: {:?}", entry);
                return Some(entry.value);
            }
        }

        None
    }
}

/// Mirrors encoded gamestates the standard connect four 6x7 grid at the middle (fourth) column
/// using bitwise operations
pub fn mirror_gamestate(gamestate: u128) -> u128 {
    // Move first colum to seventh
    (3541991048129292582915 & gamestate) * 4096 +
    // Move second colum to sixth
    (14167964192517170331660 & gamestate) * 256 +
    // Move third colum to fifth
    (56671856770068681326640 & gamestate) * 16 +

    // Copy fourth column
    (226687427080274725306560 & gamestate) +

    // Move fifth colum to third
    (906749708321098901226240 & gamestate) / 16 +
    // Move sixth colum to second
    (3626998833284395604904960 & gamestate) / 256 +
    // Move seventh colum to first
    (14507995333137582419619840 & gamestate) / 4096
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: u128 = 2;

    #[test]
    fn mirror_gamestate_given_gamestate_return_correct_mirror() {
        assert_eq!(mirror_gamestate(1), 4096);
        assert_eq!(mirror_gamestate(65536), 16777216);
        assert_eq!(mirror_gamestate(524288), 8388608);
        assert_eq!(mirror_gamestate(1048576), 1048576);
        assert_eq!(mirror_gamestate(1642496), 26214401);
        assert_eq!(mirror_gamestate(27917287425), 704374640640);
        assert_eq!(
            mirror_gamestate(18894078743396915085312),
            302236066660044465242112
        );
        assert_eq!(mirror_gamestate(274877906944), 1073741824);
    }
}
