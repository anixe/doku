use proc_macro2::TokenStream;

pub enum Error {
    Darling(darling::Error),
    Syn(syn::Error),
}

impl Error {
    pub fn compile(self) -> TokenStream {
        match self {
            Error::Darling(err) => err.write_errors(),
            Error::Syn(err) => err.into_compile_error(),
        }
    }
}

impl From<darling::Error> for Error {
    fn from(err: darling::Error) -> Self {
        Self::Darling(err)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Self::Syn(err)
    }
}
