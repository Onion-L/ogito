use ogito::git::Git;

#[test]
fn test_git_args() {
    let mut git = Git::new();
    git.args(vec!["https://github.com/owner/repo.git", "/tmp/repo"]);
    assert_eq!(git.cmd, "git");
    assert_eq!(
        git.args,
        vec!["https://github.com/owner/repo.git", "/tmp/repo"]
    );
}
