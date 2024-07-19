use crate::model::WebSession;
use konnektoren_core::game::{Game, GamePath, GameState};
use konnektoren_core::prelude::{Challenge, ChallengeFactory, ChallengeType, Session};

pub trait ChallengeLoader<T> {
    fn level_a1() -> T;
    fn default_connectors() -> T;
}

impl ChallengeLoader<ChallengeFactory> for ChallengeFactory {
    fn level_a1() -> ChallengeFactory {
        let data = include_str!("../assets/challenges/articles.yml");
        let articles: ChallengeType = serde_yaml::from_str(data).unwrap();

        let articles_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_info.yml")).unwrap();

        let data = include_str!("../assets/challenges/reflexivpronouns.yml");
        let reflexivpronouns: ChallengeType = serde_yaml::from_str(data).unwrap();

        let reflexivpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/reflexivpronouns_info.yml"
        ))
        .unwrap();

        let data = include_str!("../assets/challenges/personal_pronouns.yml");
        let personalpronouns: ChallengeType = serde_yaml::from_str(data).unwrap();

        let personalpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/personal_pronouns_info.yml"
        ))
        .unwrap();

        let data = include_str!("../assets/challenges/konnektoren.yml");
        let connectives: ChallengeType = serde_yaml::from_str(data).unwrap();

        let connectives_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/connectives_info.yml"))
                .unwrap();

        ChallengeFactory {
            challenge_types: vec![
                articles,
                articles_info,
                reflexivpronouns,
                reflexivpronouns_info,
                personalpronouns,
                personalpronouns_info,
                connectives,
                connectives_info,
            ],
        }
    }

    fn default_connectors() -> ChallengeFactory {
        Self::default()
    }
}

impl ChallengeLoader<GamePath> for GamePath {
    fn level_a1() -> GamePath {
        let articles: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_a1.yml")).unwrap();
        articles
    }

    fn default_connectors() -> GamePath {
        Self::default()
    }
}

impl ChallengeLoader<GameState> for GameState {
    fn level_a1() -> GameState {
        GameState {
            current_challenge_index: 0,
            game: Game::level_a1(),
            challenge: Challenge::default(),
            current_task_index: 0,
        }
    }

    fn default_connectors() -> GameState {
        Self::default()
    }
}

impl ChallengeLoader<Game> for Game {
    fn level_a1() -> Game {
        let game_path = GamePath::level_a1();
        let challenge_factory = ChallengeFactory::level_a1();
        Game {
            game_path,
            challenge_factory,
            challenge_history: Default::default(),
        }
    }

    fn default_connectors() -> Game {
        Self::default()
    }
}

impl ChallengeLoader<Session> for Session {
    fn level_a1() -> Session {
        Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_a1(),
        }
    }

    fn default_connectors() -> Session {
        Self::default()
    }
}

impl ChallengeLoader<WebSession> for WebSession {
    fn level_a1() -> WebSession {
        WebSession {
            id: "websession".into(),
            session: Session::level_a1(),
        }
    }

    fn default_connectors() -> WebSession {
        Self::default()
    }
}