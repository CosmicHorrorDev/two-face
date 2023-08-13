use std::{fmt::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Holds the license type, text, and relative path for a syntax or theme definition
#[derive(Deserialize, Serialize)]
pub struct License {
    pub ty: LicenseType,
    pub text: String,
    pub rel_path: PathBuf,
}

impl License {
    /// Appends a section for this license to `md`
    pub fn write_md(&self, md: &mut String) {
        write!(
            md,
            "\
            ## {}\n\n\
            <details>\n\
            <summary>License text</summary>\n\
            {}\n\
            </details>\
            ",
            self.rel_path.display(),
            self.text
        )
        .expect("Infallible");

        // Make sure the last char is a newline to not mess up formatting later
        if !md.ends_with('\n') {
            md.push('\n')
        }

        // Add two more newlines to make it easy to distinguish where this text ends and the next
        // starts
        md.push_str("\n\n");
    }

    /// Whether or not this type of license requires acknowledgement
    pub fn needs_acknowledgement(&self) -> bool {
        self.ty.needs_acknowledgement()
    }
}

/// The full range of included licenses
///
/// Disclaimer: I am not a lawyer
#[derive(Deserialize, Serialize, Debug)]
#[non_exhaustive]
pub enum LicenseType {
    /// Sublime's custom license
    Sublime,
    /// [MIT License](https://choosealicense.com/licenses/mit/)
    Mit,
    /// [BSD 2-Clause License](https://choosealicense.com/licenses/bsd-2-clause/)
    Bsd2Clause,
    // TODO: what is the distinction here from the other BSD 2-Clause license?
    Bsd2ClauseFreeBsd,
    /// [The Unlicense](https://choosealicense.com/licenses/unlicense/)
    Unlicense,
    /// [BSD 3-Clause License](https://choosealicense.com/licenses/bsd-3-clause/)
    Bsd3Clause,
    /// [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
    Apache2,
    /// [Do What The F*ck You Want To Public License](https://choosealicense.com/licenses/wtfpl/)
    Wtfpl,
}

impl LicenseType {
    /// Whether or not the license requires acknowledgement
    pub fn needs_acknowledgement(&self) -> bool {
        match self {
            Self::Mit
            | Self::Bsd2Clause
            | Self::Bsd2ClauseFreeBsd
            | Self::Bsd3Clause
            | Self::Apache2 => true,
            Self::Sublime | Self::Unlicense | Self::Wtfpl => false,
        }
    }
}

/// Holds all the license information for embedded syntaxes and themes
#[derive(Deserialize, Serialize)]
pub struct Acknowledgements {
    pub(crate) for_syntaxes: Vec<License>,
    pub(crate) for_themes: Vec<License>,
}

const BAT_ACK: &str = "\
Most of the code for generating both theme and syntax dumps along with the
curation of said themes and syntaxes is taken from the
[`bat` project](https://github.com/sharkdp/bat).
";

impl Acknowledgements {
    /// Display the license information as Markdown
    ///
    /// The output is roughly as follows
    ///
    /// ```md
    /// Most of the code for generating both theme and syntax dumps along with the
    /// curation of said themes and syntaxes is taken from the
    /// [`bat` project](https://github.com/sharkdp/bat).
    ///
    /// # Syntaxes
    ///
    /// ## syntaxes/01_Packages/Rust/LICENSE.txt
    ///
    /// <details>
    /// <summary>License text</summary>
    /// ...Elided license text...
    /// </details>
    ///
    /// # Themes
    ///
    /// ## themes/1337-Scheme/LICENSE
    ///
    /// <details>
    /// <summary>License text</summary>
    /// ...Elided license text...
    /// </details>
    /// ```
    pub fn to_md(&self) -> String {
        let mut md = String::from(BAT_ACK);

        if !self.for_syntaxes.is_empty() {
            md.write_str("\n# Syntaxes\n\n").expect("Infallible");
            for license in &self.for_syntaxes {
                license.write_md(&mut md);
            }
        }

        if !self.for_themes.is_empty() {
            md.write_str("# Themes\n\n").expect("Infallible");
            for license in &self.for_themes {
                license.write_md(&mut md);
            }
        }

        md
    }
}
