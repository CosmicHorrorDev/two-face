use std::fs;

use similar::TextDiff;

#[test]
fn full_matches_prebuilt() {
    let full = two_face::acknowledgement::listing();
    let rendered = full.to_md();
    let prebuilt = fs::read_to_string("generated/acknowledgements_full.md").unwrap();

    let diff = TextDiff::from_lines(&rendered, &prebuilt);
    // The only extra lines should be from assets that don't require acknowledgements which still
    // gets included in the prebuilt listing and not the embedded
    insta::assert_snapshot!(diff.unified_diff(), @r#"
    @@ -5,6 +5,22 @@
     # Syntaxes
     
     <details>
    +<summary>syntaxes/01_Packages/LICENSE</summary>
    +
    +````text
    +If not otherwise specified (see below), files in this repository fall under the following license:
    +
    +    Permission to copy, use, modify, sell and distribute this
    +    software is granted. This software is provided "as is" without
    +    express or implied warranty, and with no claim as to its
    +    suitability for any purpose.
    +
    +An exception is made for files in readable text which contain their own license information, or files where an accompanying file exists (in the same directory) with a “-license” suffix added to the base-name name of the original file, and an extension of txt, html, or similar. For example “tidy” is accompanied by “tidy-license.txt”.
    +
    +````
    +</details>
    +
    +<details>
     <summary>syntaxes/01_Packages/Rust/LICENSE.txt</summary>
     
     ````text
    @@ -507,6 +523,58 @@
     </details>
     
     <details>
    +<summary>syntaxes/02_Extra/GLSL/LICENSE</summary>
    +
    +````text
    +This is free and unencumbered software released into the public domain.
    +
    +Anyone is free to copy, modify, publish, use, compile, sell, or
    +distribute this software, either in source code form or as a compiled
    +binary, for any purpose, commercial or non-commercial, and by any
    +means.
    +
    +In jurisdictions that recognize copyright laws, the author or authors
    +of this software dedicate any and all copyright interest in the
    +software to the public domain. We make this dedication for the benefit
    +of the public at large and to the detriment of our heirs and
    +successors. We intend this dedication to be an overt act of
    +relinquishment in perpetuity of all present and future rights to this
    +software under copyright law.
    +
    +THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    +EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    +MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    +IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
    +OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
    +ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    +OTHER DEALINGS IN THE SOFTWARE.
    +
    +For more information, please refer to <http://unlicense.org/>
    +
    +````
    +</details>
    +
    +<details>
    +<summary>syntaxes/02_Extra/GraphQL/LICENSE</summary>
    +
    +````text
    +        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    +                    Version 2, December 2004
    +
    + Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    +
    + Everyone is permitted to copy and distribute verbatim or modified
    + copies of this license document, and changing it is allowed as long
    + as the name is changed.
    +
    +            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    +   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    +
    +  0. You just DO WHAT THE FUCK YOU WANT TO.
    +````
    +</details>
    +
    +<details>
     <summary>syntaxes/02_Extra/Groff/LICENSE</summary>
     
     ````text
    @@ -1722,6 +1790,38 @@
     </details>
     
     <details>
    +<summary>syntaxes/02_Extra/PureScript/LICENSE</summary>
    +
    +````text
    +This is free and unencumbered software released into the public domain.
    +
    +Anyone is free to copy, modify, publish, use, compile, sell, or
    +distribute this software, either in source code form or as a compiled
    +binary, for any purpose, commercial or non-commercial, and by any
    +means.
    +
    +In jurisdictions that recognize copyright laws, the author or authors
    +of this software dedicate any and all copyright interest in the
    +software to the public domain. We make this dedication for the benefit
    +of the public at large and to the detriment of our heirs and
    +successors. We intend this dedication to be an overt act of
    +relinquishment in perpetuity of all present and future rights to this
    +software under copyright law.
    +
    +THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    +EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    +MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    +IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
    +OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
    +ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    +OTHER DEALINGS IN THE SOFTWARE.
    +
    +For more information, please refer to <http://unlicense.org>
    +
    +````
    +</details>
    +
    +<details>
     <summary>syntaxes/02_Extra/PureScript/NOTICE</summary>
     
     ````text
    "#);
}
