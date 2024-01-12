use super::*;

/// Returns whether the board is full and the game is over given an encoded gamestate
pub fn is_full(gamestate: u128) -> bool {
    gamestate.count_ones() == 42
}

/// Returns true if someone has won or the board is full otherwise false given an encoded gamestate
pub fn is_over(gamestate: u128) -> bool {
    if is_full(gamestate) {
        true
    } else {
        match is_won(gamestate) {
            Some(_) => true,
            None => false,
        }
    }
}

/// Returns which player color has won the game if so given an encoded gamestate
pub fn is_won(gamestate: u128) -> Option<PlayerColor> {
    // Can't win before 7th turn
    if gamestate.count_ones() < 6 {
        return None;
    };

    // Vector of possible winning combinations for blue
    let blue_winning_gamestates: Vec<u128> = vec![
        85,
        1392640,
        22817013760,
        373833953443840,
        6124895493223874560,
        100350287760979960791040,
        340,
        5570560,
        91268055040,
        1495335813775360,
        24499581972895498240,
        401401151043919843164160,
        1360,
        22282240,
        365072220160,
        5981343255101440,
        97998327891581992960,
        1605604604175679372656640,
        5440,
        89128960,
        1460288880640,
        23925373020405760,
        391993311566327971840,
        6422418416702717490626560,
        4398314962945,
        17593259851780,
        70373039407120,
        281492157628480,
        1125968630513920,
        4503874522055680,
        18015498088222720,
        72061992352890880,
        288247969411563520,
        1152991877646254080,
        4611967510585016320,
        18447870042340065280,
        73791480169360261120,
        295165920677441044480,
        1180663682709764177920,
        4722654730839056711680,
        18890618923356226846720,
        75562475693424907386880,
        302249902773699629547520,
        1208999611094798518190080,
        4835998444379194072760320,
        281479271743489,
        1125917086973956,
        4503668347895824,
        18014673391583296,
        4611756388245323776,
        18447025552981295104,
        73788102211925180416,
        295152408847700721664,
        75559016665011384745984,
        302236066660045538983936,
        1208944266640182155935744,
        4835777066560728623742976,
        4399120515136,
        17596482060544,
        70385928242176,
        281543712968704,
        72075190519988224,
        288300762079952896,
        1153203048319811584,
        4612812193279246336,
        1180879921479487062016,
        4723519685917948248064,
        18894078743671792992256,
        75576314974687171969024,
    ];

    //Vector of possible winning combinations for red
    let red_winning_gamestates: Vec<u128> = vec![
        170,
        2785280,
        45634027520,
        747667906887680,
        12249790986447749120,
        200700575521959921582080,
        680,
        11141120,
        182536110080,
        2990671627550720,
        48999163945790996480,
        802802302087839686328320,
        2720,
        44564480,
        730144440320,
        11962686510202880,
        195996655783163985920,
        3211209208351358745313280,
        10880,
        178257920,
        2920577761280,
        47850746040811520,
        783986623132655943680,
        12844836833405434981253120,
        8796629925890,
        35186519703560,
        140746078814240,
        562984315256960,
        2251937261027840,
        9007749044111360,
        36030996176445440,
        144123984705781760,
        576495938823127040,
        2305983755292508160,
        9223935021170032640,
        36895740084680130560,
        147582960338720522240,
        590331841354882088960,
        2361327365419528355840,
        9445309461678113423360,
        37781237846712453693440,
        151124951386849814773760,
        604499805547399259095040,
        2417999222189597036380160,
        9671996888758388145520640,
        562958543486978,
        2251834173947912,
        9007336695791648,
        36029346783166592,
        9223512776490647552,
        36894051105962590208,
        147576204423850360832,
        590304817695401443328,
        151118033330022769491968,
        604472133320091077967872,
        2417888533280364311871488,
        9671554133121457247485952,
        8798241030272,
        35192964121088,
        140771856484352,
        563087425937408,
        144150381039976448,
        576601524159905792,
        2306406096639623168,
        9225624386558492672,
        2361759842958974124032,
        9447039371835896496128,
        37788157487343585984512,
        151152629949374343938048,
    ];

    if red_winning_gamestates
        .into_iter()
        .filter(|x| *x & gamestate == *x)
        .collect::<Vec<u128>>()
        .len()
        > 0
    {
        Some(PlayerColor::Red)
    } else if blue_winning_gamestates
        .into_iter()
        .filter(|x| *x & gamestate == *x)
        .collect::<Vec<u128>>()
        .len()
        > 0
    {
        Some(PlayerColor::Blue)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_won_given_empty_gamestate_return_none() {
        assert_eq!(is_won(0), None);
    }

    #[test]
    fn is_won_given_winning_verticals_blue_return_blue() {
        assert_eq!(is_won(39584834666497), Some(PlayerColor::Blue));
        assert_eq!(
            is_won(39584834666497 * 4 * 4 * BASE.pow(28)),
            Some(PlayerColor::Blue)
        );
    }

    #[test]
    fn is_won_given_winning_verticals_red_return_red() {
        assert_eq!(is_won(35191083368472), Some(PlayerColor::Red));
        assert_eq!(
            is_won(35191083368472 * 4 * BASE.pow(14)),
            Some(PlayerColor::Red)
        );
        assert_eq!(is_won(8028692909229660883124224), Some(PlayerColor::Red));
    }

    #[test]
    fn is_won_given_winning_diagonals_blue_return_blue() {
        assert_eq!(is_won(457403279671297), Some(PlayerColor::Blue));
        assert_eq!(
            is_won(457403279671297 * 4 * 4 * BASE.pow(14)),
            Some(PlayerColor::Blue)
        );
    }

    #[test]
    fn is_won_given_winning_diagonals_red_return_red() {
        assert_eq!(is_won(8798509547648), Some(PlayerColor::Red));
        assert_eq!(
            is_won(8798509547648 * 4 * 4 * BASE.pow(28)),
            Some(PlayerColor::Red)
        );
    }

    #[test]
    fn is_over_given_someone_has_won_return_true() {
        assert_eq!(is_over(688213), true);
    }

    #[test]
    fn is_over_given_full_board_return_true() {
        assert_eq!(is_over(6447604371278022265099605), true);
    }

    #[test]
    fn is_full_given_full_board_return_true() {
        assert_eq!(is_full(6447604371278022265099605), true);
    }

    #[test]
    fn is_full_given_not_full_board_return_false() {
        assert_eq!(is_full(24934), false);
        assert_eq!(is_full(2405), false);
    }
}
