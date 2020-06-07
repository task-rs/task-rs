use super::Error;

#[derive(Debug)]
pub enum Reason {
    Empty,
    UndefinedTag(String),
    GitPushPullMissingArgument,
    GitPushPullRedundantArgument(Vec<String>),
}

impl Reason {
    pub(crate) fn err<Value>(self, text: &str) -> Result<Value, Error> {
        Err(Error {
            text: text.to_owned(),
            reason: self,
        })
    }
}

impl ToString for Reason {
    fn to_string(&self) -> String {
        use Reason::*;
        match self {
            Empty => "empty".to_owned(),
            UndefinedTag(tag) => format!("tag {:?} is not defined", tag),
            GitPushPullMissingArgument => "git-push-pull: missing argument".to_owned(),
            GitPushPullRedundantArgument(arguments) => {
                format!("git-push-pull: redundant arguments: {:?}", arguments)
            }
        }
    }
}
