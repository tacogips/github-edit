use github_edit::types::pull_request::{Branch, PullRequestNumber, PullRequestState};
use github_edit::types::repository::RepositoryId;
use serial_test::serial;

mod common;

/// Comprehensive test for complete pull request lifecycle operations
///
/// This test performs a full lifecycle of pull request operations:
/// 1. Create new pull request from github-edit-test branch to main
/// 2. Edit pull request body and verify
/// 3. Add assignee and verify
/// 4. Change assignee (remove) and verify
/// 5. Add comment and verify
/// 6. Edit comment and verify
/// 7. Delete comment and verify
/// 8. Close pull request and verify closure
///
/// Each operation is followed by a get_pull_request call to verify the changes.
#[tokio::test]
#[serial]
async fn test_pull_request_lifecycle_operations() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // 1. Create new pull request
    let initial_title = "Test PR for Lifecycle Operations";
    let initial_body = "This is the initial body of the test pull request.";
    let head_branch = Branch::new("bugfix/api-client");
    let base_branch = Branch::new("main");

    let created_pr = client
        .create_pull_request(
            &repository_id,
            initial_title,
            &head_branch,
            &base_branch,
            Some(initial_body),
            Some(false), // not a draft
        )
        .await
        .expect("Failed to create pull request");

    let pr_number = PullRequestNumber::new(created_pr.pull_request_id.number);
    println!("Created pull request #{}", pr_number.value());

    // Verify initial state
    let retrieved_pr = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get created pull request");

    assert_eq!(retrieved_pr.title, initial_title);
    assert_eq!(retrieved_pr.body.as_deref(), Some(initial_body));
    assert_eq!(retrieved_pr.head_branch, head_branch.0);
    assert_eq!(retrieved_pr.base_branch, base_branch.0);
    assert!(retrieved_pr.assignees.is_empty());

    // 2. Edit pull request body
    let new_body = "This is the updated body content with more details about the PR changes.";
    client
        .edit_pull_request_body(&repository_id, pr_number, new_body)
        .await
        .expect("Failed to edit pull request body");

    // Verify body edit
    let pr_after_body_edit = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after body edit");

    assert_eq!(pr_after_body_edit.body.as_deref(), Some(new_body));
    println!("✓ Pull request body edited successfully");

    // 3. Add assignee
    let assignees = vec!["tacogips".to_string()];
    client
        .add_pull_request_assignees(&repository_id, pr_number, &assignees)
        .await
        .expect("Failed to add assignee to pull request");

    // Verify assignee addition
    let pr_after_assignee_add = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after assignee add");

    assert_eq!(pr_after_assignee_add.assignees.len(), 1);
    assert_eq!(pr_after_assignee_add.assignees[0].username(), "tacogips");
    println!("✓ Assignee added successfully");

    // 4. Change assignee (remove assignee)
    let empty_assignees: Vec<String> = vec![];
    client
        .edit_pull_request_assignees(&repository_id, pr_number, &empty_assignees)
        .await
        .expect("Failed to remove assignee from pull request");

    // Verify assignee removal
    let pr_after_assignee_remove = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after assignee remove");

    assert!(pr_after_assignee_remove.assignees.is_empty());
    println!("✓ Assignee removed successfully");

    // 5. Add comment
    let comment_body = "This is a test comment for the pull request lifecycle test.";
    let comment_number = client
        .add_pull_request_comment(&repository_id, pr_number, comment_body)
        .await
        .expect("Failed to add comment to pull request");

    // Verify comment addition
    let pr_after_comment_add = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after comment add");

    assert_eq!(pr_after_comment_add.comments.len(), 1);
    assert_eq!(pr_after_comment_add.comments[0].body, comment_body);
    println!("✓ Comment added successfully");

    // 6. Edit comment
    let updated_comment_body = "This is the updated comment text with modifications for the PR.";
    client
        .edit_pull_request_comment(
            &repository_id,
            pr_number,
            comment_number,
            updated_comment_body,
        )
        .await
        .expect("Failed to edit pull request comment");

    // Verify comment edit
    let pr_after_comment_edit = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after comment edit");

    assert_eq!(pr_after_comment_edit.comments.len(), 1);
    assert_eq!(pr_after_comment_edit.comments[0].body, updated_comment_body);
    println!("✓ Comment edited successfully");

    // 7. Delete comment
    client
        .delete_pull_request_comment(&repository_id, pr_number, comment_number)
        .await
        .expect("Failed to delete pull request comment");

    // Verify comment deletion
    let pr_after_comment_delete = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after comment delete");

    assert!(pr_after_comment_delete.comments.is_empty());
    println!("✓ Comment deleted successfully");

    // 8. Close pull request
    client
        .close_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to close pull request");

    // Verify pull request is closed
    let closed_pr = client
        .get_pull_request(&repository_id, pr_number)
        .await
        .expect("Failed to get pull request after closing");

    assert_eq!(closed_pr.title, initial_title);
    assert_eq!(closed_pr.body.as_deref(), Some(new_body));
    assert!(closed_pr.comments.is_empty());
    assert!(closed_pr.assignees.is_empty());
    assert_eq!(closed_pr.state, PullRequestState::Closed);
    println!("✓ Pull request closed successfully");

    println!("🎉 All pull request lifecycle operations completed successfully!");
}

/// Test that creating a pull request in a non-existent repository fails immediately
///
/// This test verifies that the fix for the retry bug is working correctly.
/// When attempting to create a pull request in a repository that doesn't exist,
/// the operation should fail immediately with a NonRetryable error (404)
/// instead of retrying 15 times with exponential backoff.
#[tokio::test]
#[serial]
async fn test_create_pull_request_nonexistent_repository_fails_immediately() {
    let client = common::create_test_github_client();

    // Use a repository that definitely doesn't exist
    let nonexistent_repo = RepositoryId::new("nonexistent-user-12345", "nonexistent-repo-67890");

    let start_time = std::time::Instant::now();

    // Attempt to create a pull request in the non-existent repository
    let head_branch = Branch::new("feature-branch");
    let base_branch = Branch::new("main");

    let result = client
        .create_pull_request(
            &nonexistent_repo,
            "Test PR",
            &head_branch,
            &base_branch,
            Some("This should fail immediately"),
            Some(false), // not a draft
        )
        .await;

    let elapsed = start_time.elapsed();

    // The operation should fail
    assert!(
        result.is_err(),
        "Creating pull request in non-existent repository should fail"
    );

    // The operation should fail quickly (within 5 seconds)
    // Before the fix, this would take ~30 seconds due to 15 retry attempts
    assert!(
        elapsed.as_secs() < 5,
        "Operation should fail quickly (took {} seconds), not retry multiple times",
        elapsed.as_secs()
    );

    // Verify the error message indicates it's a client error (4xx)
    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("404") || error_message.contains("Not Found"),
        "Error should indicate repository not found: {}",
        error_message
    );

    println!(
        "✓ Non-existent repository correctly failed in {} seconds",
        elapsed.as_secs_f64()
    );
}
