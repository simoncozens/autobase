use skrifa::Tag;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutobaseError {
    #[error("No baseline tag found for script {script} and baseline tag {tag}")]
    BaselineTagNotFound { script: Tag, tag: Tag },
    #[error("Problem reading font: {0}")]
    FontRead(#[from] skrifa::raw::ReadError),
    #[error("BASE table script list not found")]
    BaseScriptListNotFound,
    #[error("BASE script record not found for script {script}")]
    BaseScriptNotFound { script: Tag },
    #[error("Error building binary font: {0}")]
    FontBuild(#[from] write_fonts::BuilderError),
}
