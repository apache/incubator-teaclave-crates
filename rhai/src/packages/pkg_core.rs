#[cfg(feature = "no_std")]
use std::prelude::v1::*;

use super::*;
use crate::def_package;
use crate::module::ModuleFlags;

def_package! {
    /// Core package containing basic facilities.
    ///
    /// # Contents
    ///
    /// * [`LanguageCorePackage`][super::LanguageCorePackage]
    /// * [`ArithmeticPackage`][super::ArithmeticPackage]
    /// * [`BasicStringPackage`][super::BasicStringPackage]
    /// * [`BasicIteratorPackage`][super::BasicIteratorPackage]
    /// * [`BasicFnPackage`][super::BasicFnPackage]
    /// * [`DebuggingPackage`][super::DebuggingPackage]
    pub CorePackage(lib) :
            LanguageCorePackage,
            ArithmeticPackage,
            BasicStringPackage,
            BasicIteratorPackage,
            BasicFnPackage,
            #[cfg(feature = "debugging")] DebuggingPackage
        {
        lib.flags |= ModuleFlags::STANDARD_LIB;
    }
}
