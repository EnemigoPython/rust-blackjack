mod deck;
mod player;
mod io;

#[cfg(test)]
mod tests {
    use super::deck::tests as deck_tests;
    use super::player::tests as player_tests;

    #[test]
    fn create_card() {
        deck_tests::create_card();
    }

    #[test]
    fn create_deck() {
        deck_tests::create_deck();
    }

    #[test]
    fn shuffle_deck() {
        deck_tests::shuffle_deck();
    }

    #[test]
    fn deal_from_deck() {
        deck_tests::deal_from_deck();
    }

    #[test]
    fn create_player() {
        player_tests::create_player();
    }

    #[test]
    fn deal_player_cards() {
        player_tests::deal_player_cards();
    }

    #[test]
    fn check_valid_moves() {
        player_tests::check_valid_moves();
    }

    #[test]
    fn make_bet() {
        player_tests::make_bet();
    }

    #[test]
    fn create_player_list() {
        player_tests::create_player_list();
    }
}
