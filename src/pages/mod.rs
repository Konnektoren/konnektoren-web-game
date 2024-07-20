pub mod about;
pub mod challenge;
pub mod home;
pub mod map;
pub mod profile;

#[cfg(feature = "yew-preview")]
pub mod preview;
mod results;

pub use about::AboutPage;
pub use challenge::ChallengePage;
pub use home::HomePage;
pub use map::MapPage;
pub use profile::ProfilePage;
pub use results::ResultsPage;
