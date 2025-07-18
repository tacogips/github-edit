fn main() {
    let octocrab = octocrab::Octocrab::builder().build().unwrap();
    let _pulls = octocrab.pulls("owner", "repo");
    let _issues = octocrab.issues("owner", "repo");
}
