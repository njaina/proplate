use crate::ui;

#[derive(Debug)]
pub enum ProplateErrorKind {
    TemplateNotFound,
    InvalidTemplate,
    Fs,
    PromptUser,
}

#[derive(Debug)]
pub struct ProplateError {
    pub kind: ProplateErrorKind,
    pub reason: String,
}

pub type ProplateResult<T> = Result<T, ProplateError>;

impl ProplateError {
    pub fn new(kind: ProplateErrorKind, reason: &str) -> ProplateError {
        Self {
            kind,
            reason: reason.to_string(),
        }
    }

    pub fn invalid_template_conf(details: &str) -> ProplateError {
        Self::new(ProplateErrorKind::InvalidTemplate, details)
    }

    pub fn fs(details: &str) -> ProplateError {
        Self::new(ProplateErrorKind::Fs, details)
    }

    pub fn local_template_not_found(id: &str) -> ProplateError {
        Self::new(
            ProplateErrorKind::TemplateNotFound,
            &format!("Local template (id={}) is not found.", id),
        )
    }

    pub fn remote_template_not_found(id: &str) -> ProplateError {
        Self::new(
            ProplateErrorKind::TemplateNotFound,
            &format!("Remote template (id={}) is not found.", id),
        )
    }

    pub fn prompt(details: &str) -> ProplateError {
        Self::new(ProplateErrorKind::PromptUser, details)
    }
}

impl ui::AsError for ProplateError {
    fn print_err(&self) -> String {
        ui::error(&format!("{:?}", self))
    }
}
