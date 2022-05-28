mod deck;

#[cfg(test)]
mod tests {
    use super::deck::tests as deck_tests;

    #[test]
    fn create_card() {
        deck_tests::create_card();
    }
}
