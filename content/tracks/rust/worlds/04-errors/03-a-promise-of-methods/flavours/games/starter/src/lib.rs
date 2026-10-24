/// A promise: any type that keeps it can describe itself in one short line.
pub trait Label {
    fn label(&self) -> String;
}

pub enum Loader {
    Fabric,
    Forge,
}

impl Label for Loader {
    fn label(&self) -> String {
        match self {
            Loader::Fabric => "fabric".to_string(),
            Loader::Forge => "forge".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ManifestError {
    MissingVersion,
    BadVersion(String),
    TooNew(u32),
}

// TODO: keep the Label promise for ManifestError:
//       MissingVersion -> "no version", BadVersion(text) -> "'<text>' is not a number", TooNew(v) -> "version <v> is too new"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_version_is_labelled() {
        assert_eq!(Loader::Forge.label(), "forge", "the given Loader impl, for comparison");
        assert_eq!(ManifestError::MissingVersion.label(), "no version");
    }

    #[test]
    fn labels_show_what_the_player_typed() {
        assert_eq!(ManifestError::BadVersion("new".to_string()).label(), "'new' is not a number");
        assert_eq!(ManifestError::TooNew(99).label(), "version 99 is too new");
    }
}
