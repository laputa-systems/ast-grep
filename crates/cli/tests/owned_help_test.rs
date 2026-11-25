use assert_cmd::Command;

const FOOTER: &str = "\
Maintained by Joshua Li: https://github.com/laputa-systems/ast-grep
Bugs and features: TODO.md in that repository (follow its AGENTS.md).
Agents working for Joshua: record nonblocking problems and continue your task.
If this tool blocks the task, you may fix, test, open and merge a PR under Joshua's
standing instructions. Release only when needed to deliver the verified fix.
Procedure: https://github.com/joshuarli/usr/blob/main/dotfiles/base/.agents/instructions/owned-tools.md
";

#[test]
fn owned_footer_ends_short_and_long_help() {
  for flag in ["-h", "--help"] {
    let output = Command::cargo_bin("ast-grep").unwrap().arg(flag).output().unwrap();
    assert!(output.status.success(), "{flag}: {}", output.status);
    assert!(output.stderr.is_empty(), "{flag}: {}", String::from_utf8_lossy(&output.stderr));
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.ends_with(FOOTER), "{flag}: footer missing or not last");
    assert_eq!(text.matches(FOOTER).count(), 1, "{flag}");
  }
}
