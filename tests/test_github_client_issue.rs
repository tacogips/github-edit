use github_edit::types::issue::{IssueNumber, IssueState};
use github_edit::types::repository::RepositoryId;
use serial_test::serial;

mod common;

/// Comprehensive test for complete issue lifecycle operations
///
/// This test performs a full lifecycle of issue operations:
/// 1. Create new issue
/// 2. Edit issue title and verify
/// 3. Edit issue body and verify
/// 4. Add assignee and verify
/// 5. Change assignee (remove) and verify
/// 6. Add comment and verify
/// 7. Edit comment and verify
/// 8. Delete comment and verify
/// 9. Delete issue and verify deletion
///
/// Each operation is followed by a get_issue call to verify the changes.
#[tokio::test]
#[serial]
async fn test_issue_lifecycle_operations() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // 1. Create new issue
    let initial_title = "Test Issue for Lifecycle Operations";
    let initial_body = "This is the initial body of the test issue.";
    let created_issue = client
        .create_issue(
            &repository_id,
            initial_title,
            Some(initial_body),
            None, // no assignees initially
            None, // no labels
            None, // no milestone
        )
        .await
        .expect("Failed to create issue");

    let issue_number = IssueNumber::new(created_issue.issue_id.number);
    println!("Created issue #{}", issue_number);

    // Verify initial state
    let retrieved_issue = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get created issue");

    assert_eq!(retrieved_issue.title, initial_title);
    assert_eq!(retrieved_issue.body.as_deref(), Some(initial_body));
    assert_eq!(retrieved_issue.state, IssueState::Open);
    assert!(retrieved_issue.assignees.is_empty());

    // 2. Edit issue title
    let new_title = "Updated Test Issue Title";
    client
        .edit_issue_title(&repository_id, issue_number, new_title)
        .await
        .expect("Failed to edit issue title");

    // Verify title edit
    let issue_after_title_edit = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after title edit");

    assert_eq!(issue_after_title_edit.title, new_title);
    println!("✓ Issue title edited successfully");

    // 3. Edit issue body
    let new_body = "This is the updated body content with more details.";
    client
        .edit_issue_body(&repository_id, issue_number, new_body)
        .await
        .expect("Failed to edit issue body");

    // Verify body edit
    let issue_after_body_edit = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after body edit");

    assert_eq!(issue_after_body_edit.body.as_deref(), Some(new_body));
    println!("✓ Issue body edited successfully");

    // 4. Add assignee
    let assignees = vec!["tacogips".to_string()];
    client
        .edit_issue_assignees(&repository_id, issue_number, &assignees)
        .await
        .expect("Failed to add assignee");

    // Verify assignee addition
    let issue_after_assignee_add = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after assignee add");

    assert_eq!(issue_after_assignee_add.assignees, assignees);
    println!("✓ Assignee added successfully");

    // 5. Change assignee (remove assignee)
    let empty_assignees: Vec<String> = vec![];
    client
        .edit_issue_assignees(&repository_id, issue_number, &empty_assignees)
        .await
        .expect("Failed to remove assignee");

    // Verify assignee removal
    let issue_after_assignee_remove = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after assignee remove");

    assert!(issue_after_assignee_remove.assignees.is_empty());
    println!("✓ Assignee removed successfully");

    // 6. Add comment
    let comment_body = "This is a test comment for the issue lifecycle test.";
    let comment_number = client
        .add_issue_comment(&repository_id, issue_number, comment_body)
        .await
        .expect("Failed to add comment");

    // Verify comment addition
    let issue_after_comment_add = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after comment add");

    assert_eq!(issue_after_comment_add.comments.len(), 1);
    assert_eq!(issue_after_comment_add.comments[0].body, comment_body);
    println!("✓ Comment added successfully");

    // 7. Edit comment
    let updated_comment_body = "This is the updated comment text with modifications.";
    client
        .edit_issue_comment(
            &repository_id,
            issue_number,
            comment_number,
            updated_comment_body,
        )
        .await
        .expect("Failed to edit comment");

    // Verify comment edit
    let issue_after_comment_edit = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after comment edit");

    assert_eq!(issue_after_comment_edit.comments.len(), 1);
    assert_eq!(
        issue_after_comment_edit.comments[0].body,
        updated_comment_body
    );
    println!("✓ Comment edited successfully");

    // 8. Delete comment
    client
        .delete_issue_comment(&repository_id, issue_number, comment_number)
        .await
        .expect("Failed to delete comment");

    // Verify comment deletion
    let issue_after_comment_delete = client
        .get_issue(&repository_id, issue_number)
        .await
        .expect("Failed to get issue after comment delete");

    assert!(issue_after_comment_delete.comments.is_empty());
    println!("✓ Comment deleted successfully");

    // 9. Delete issue
    client
        .delete_issue(&repository_id, issue_number)
        .await
        .expect("Failed to delete issue");

    // Verify issue deletion - this should fail since the issue no longer exists
    let delete_verification_result = client.get_issue(&repository_id, issue_number).await;

    assert!(
        delete_verification_result.is_err(),
        "Issue should not exist after deletion"
    );
    println!("✓ Issue deleted successfully");

    println!("🎉 All issue lifecycle operations completed successfully!");
}

/// Test that creating an issue in a non-existent repository fails immediately
///
/// This test verifies that the fix for the retry bug is working correctly.
/// When attempting to create an issue in a repository that doesn't exist,
/// the operation should fail immediately with a NonRetryable error (404)
/// instead of retrying 15 times with exponential backoff.
#[tokio::test]
#[serial]
async fn test_create_issue_nonexistent_repository_fails_immediately() {
    let client = common::create_test_github_client();

    // Use a repository that definitely doesn't exist
    let nonexistent_repo = RepositoryId::new("nonexistent-user-12345", "nonexistent-repo-67890");

    let start_time = std::time::Instant::now();

    // Attempt to create an issue in the non-existent repository
    let result = client
        .create_issue(
            &nonexistent_repo,
            "Test Issue",
            Some("This should fail immediately"),
            None,
            None,
            None,
        )
        .await;

    let elapsed = start_time.elapsed();

    // The operation should fail
    assert!(
        result.is_err(),
        "Creating issue in non-existent repository should fail"
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
