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

    pub fn needs_acknowledgement(&self) -> bool {
        self.ty.needs_acknowledgement()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LicenseType {
    Sublime,
    Mit,
    Bsd2Clause,
    Bsd2ClauseFreeBsd,
    Unlicense,
    Bsd3Clause,
    Apache2,
    Wtfpl,
}

impl LicenseType {
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
