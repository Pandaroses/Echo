mod question;
mod template;

pub use template::QuestionTemplate;
pub use question::Question;

pub enum Difficulty {
    Easy,
    Medium,
    Hard
}