/// The purpose of this module is to share contents which are important for the games coordination (player handling, game phase enforcement) and the play field storing the state of the game and abstractions around it.

pub mod game {

    use std::{fmt::Display, ops::Not};

    use self::state::representation::types::FieldState;

    pub mod painting {
        use once_cell::sync::Lazy;
        use yansi::Style;

        const EMP_COLOR: (u8, u8, u8) = (193, 49, 0);
        pub static EMP: Lazy<Style> =
            Lazy::new(|| Style::new(yansi::Color::RGB(EMP_COLOR.0, EMP_COLOR.1, EMP_COLOR.2)));

        pub static HIGHLIGHT: Lazy<Style> = Lazy::new(|| Style::new(yansi::Color::Blue));
        pub static ERROR: Lazy<Style> = Lazy::new(|| Style::new(yansi::Color::Red).bold());
    }

    pub mod efficient_state;
    pub mod state;

    pub type Field = (char, u8);

    #[derive(Debug, Clone, Copy)]
    pub enum PlayerColor {
        White,
        Black,
    }

    impl Display for PlayerColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PlayerColor::White => f.write_str("●"),
                PlayerColor::Black => f.write_str("○"),
            }
        }
    }

    impl Into<FieldState> for PlayerColor {
        fn into(self) -> FieldState {
            match self {
                PlayerColor::White => FieldState::White,
                PlayerColor::Black => FieldState::Black,
            }
        }
    }

    impl Into<u16> for PlayerColor {
        /// Needed for the [EfficientPlayField] representation of the enum
        fn into(self) -> u16 {
            match self {
                PlayerColor::White => 1u16,
                PlayerColor::Black => 2u16,
            }
        }
    }

    impl Not for PlayerColor {
        type Output = PlayerColor;

        fn not(self) -> Self::Output {
            if let PlayerColor::White = self {
                PlayerColor::Black
            } else {
                PlayerColor::White
            }
        }
    }
}
