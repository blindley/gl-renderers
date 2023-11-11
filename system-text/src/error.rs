#[derive(Debug, Clone)]
pub enum Error {
    Compile(CompileError),
    Link(LinkError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::Compile(e) => write!(f, "{}", e),
            Error::Link(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<CompileError> for Error {
    fn from(e: CompileError) -> Self {
        Error::Compile(e)
    }
}

impl From<LinkError> for Error {
    fn from(e: LinkError) -> Self {
        Error::Link(e)
    }
}

#[derive(Debug, Clone)]
pub struct CompileError {
    message: String,
}

impl CompileError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CompileError: {}", self.message)
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug, Clone)]
pub struct LinkError {
    message: String,
}

impl LinkError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LinkError: {}", self.message)
    }
}

impl std::error::Error for LinkError {}
