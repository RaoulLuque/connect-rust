/// Mirrors encoded gamestates the standard connect four 6x7 grid at the middle (fourth) column
/// using bitwise operations
pub fn mirror_gamestate(gamestate_to_be_mirrored: u128) -> u128 {
    // Move first colum to seventh
    (3541991048129292582915 & gamestate_to_be_mirrored) * 4096 +
    // Move second colum to sixth
    (14167964192517170331660 & gamestate_to_be_mirrored) * 256 +
    // Move third colum to fifth
    (56671856770068681326640 & gamestate_to_be_mirrored) * 16 +

    // Copy fourth column
    (226687427080274725306560 & gamestate_to_be_mirrored) +

    // Move fifth colum to third
    (906749708321098901226240 & gamestate_to_be_mirrored) / 16 +
    // Move sixth colum to second
    (3626998833284395604904960 & gamestate_to_be_mirrored) / 256 +
    // Move seventh colum to first
    (14507995333137582419619840 & gamestate_to_be_mirrored) / 4096
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
