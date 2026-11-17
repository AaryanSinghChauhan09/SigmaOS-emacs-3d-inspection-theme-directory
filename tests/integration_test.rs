// Integration test for SigmaOS-emacs-3d-inspection-theme-directory
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_emacs_3d_inspection_theme_directory as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
