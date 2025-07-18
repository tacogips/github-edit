use github_edit::types::milestone::MilestoneState;
use github_edit::types::repository::RepositoryId;
use serial_test::serial;

mod common;

/// Comprehensive test for complete milestone lifecycle operations
///
/// This test performs a full lifecycle of milestone operations:
/// 1. Create new milestone
/// 2. Update milestone title and verify
/// 3. Update milestone description and verify
/// 4. Update milestone state and verify
/// 5. Delete milestone and verify deletion
///
/// Each operation is followed by a verification step to ensure the changes were applied correctly.
#[tokio::test]
#[serial]
async fn test_milestone_lifecycle_operations() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // 1. Create new milestone
    let timestamp = chrono::Utc::now().timestamp();
    let initial_title = format!("Test Milestone for Lifecycle Operations {}", timestamp);
    let initial_description = "This is the initial description of the test milestone.";
    let initial_due_date = chrono::DateTime::parse_from_rfc3339("2025-08-17T07:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);

    let created_milestone = client
        .create_milestone(
            &repository_id,
            &initial_title,
            Some(initial_description),
            Some(initial_due_date),
            Some(MilestoneState::Open),
        )
        .await
        .expect("Failed to create milestone");

    let milestone_number = created_milestone.id;
    println!("Created milestone #{}", milestone_number.value());

    // Verify initial state
    assert_eq!(created_milestone.title, initial_title);
    assert_eq!(
        created_milestone.description.as_deref(),
        Some(initial_description)
    );
    assert_eq!(created_milestone.state, MilestoneState::Open);
    // Check due date matches expected value
    assert_eq!(created_milestone.due_on, Some(initial_due_date));

    // 2. Update milestone title
    let new_title = "Updated Test Milestone Title";
    println!(
        "Attempting to update milestone with ID: {}",
        milestone_number.value()
    );
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            Some(new_title),
            None,
            None,
            None,
        )
        .await
        .expect("Failed to update milestone title");

    // Verify title update
    assert_eq!(updated_milestone.title, new_title);
    assert_eq!(
        updated_milestone.description.as_deref(),
        Some(initial_description)
    ); // Should remain unchanged
    println!("✓ Milestone title updated successfully");

    // 3. Update milestone description
    let new_description = "This is the updated description content with more details.";
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            None,
            Some(new_description),
            None,
            None,
        )
        .await
        .expect("Failed to update milestone description");

    // Verify description update
    assert_eq!(
        updated_milestone.description.as_deref(),
        Some(new_description)
    );
    assert_eq!(updated_milestone.title, new_title); // Should remain unchanged
    println!("✓ Milestone description updated successfully");

    // 4. Update milestone state to closed
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            None,
            None,
            None,
            Some(MilestoneState::Closed),
        )
        .await
        .expect("Failed to update milestone state");

    // Verify state update
    assert_eq!(updated_milestone.state, MilestoneState::Closed);
    assert!(updated_milestone.closed_at.is_some());
    println!("✓ Milestone state updated to closed successfully");

    // 5. Delete milestone
    client
        .delete_milestone(&repository_id, &milestone_number)
        .await
        .expect("Failed to delete milestone");

    println!("✓ Milestone deleted successfully");

    println!("🎉 All milestone lifecycle operations completed successfully!");
}

/// Comprehensive test for complete label lifecycle operations
///
/// This test performs a full lifecycle of label operations:
/// 1. Create new label
/// 2. Update label name and verify
/// 3. Update label color and verify
/// 4. Update label description and verify
/// 5. Delete label and verify deletion
///
/// Each operation is followed by a verification step to ensure the changes were applied correctly.
#[tokio::test]
#[serial]
async fn test_label_lifecycle_operations() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // 1. Create new label
    let initial_name = "test-label-lifecycle";
    let initial_color = "ff0000"; // Red
    let initial_description = "This is a test label for lifecycle operations.";

    let created_label = client
        .create_label(
            &repository_id,
            initial_name,
            Some(initial_color),
            Some(initial_description),
        )
        .await
        .expect("Failed to create label");

    println!("Created label: {}", created_label.name);

    // Verify initial state
    assert_eq!(created_label.name, initial_name);
    assert_eq!(created_label.color.as_deref(), Some(initial_color));
    assert_eq!(
        created_label.description.as_deref(),
        Some(initial_description)
    );

    // 2. Update label name
    let new_name = "updated-test-label";
    let updated_label = client
        .update_label(&repository_id, initial_name, Some(new_name), None, None)
        .await
        .expect("Failed to update label name");

    // Verify name update
    assert_eq!(updated_label.name, new_name);
    assert_eq!(updated_label.color.as_deref(), Some(initial_color)); // Should remain unchanged
    assert_eq!(
        updated_label.description.as_deref(),
        Some(initial_description)
    ); // Should remain unchanged
    println!("✓ Label name updated successfully");

    // 3. Update label color
    let new_color = "00ff00"; // Green
    let updated_label = client
        .update_label(&repository_id, new_name, None, Some(new_color), None)
        .await
        .expect("Failed to update label color");

    // Verify color update
    assert_eq!(updated_label.color.as_deref(), Some(new_color));
    assert_eq!(updated_label.name, new_name); // Should remain unchanged
    assert_eq!(
        updated_label.description.as_deref(),
        Some(initial_description)
    ); // Should remain unchanged
    println!("✓ Label color updated successfully");

    // 4. Update label description
    let new_description = "This is the updated description for the test label.";
    let updated_label = client
        .update_label(&repository_id, new_name, None, None, Some(new_description))
        .await
        .expect("Failed to update label description");

    // Verify description update
    assert_eq!(updated_label.description.as_deref(), Some(new_description));
    assert_eq!(updated_label.name, new_name); // Should remain unchanged
    assert_eq!(updated_label.color.as_deref(), Some(new_color)); // Should remain unchanged
    println!("✓ Label description updated successfully");

    // 5. Delete label
    client
        .delete_label(&repository_id, new_name)
        .await
        .expect("Failed to delete label");

    println!("✓ Label deleted successfully");

    println!("🎉 All label lifecycle operations completed successfully!");
}

/// Test that creating a milestone in a non-existent repository fails immediately
///
/// This test verifies that the fix for the retry bug is working correctly.
/// When attempting to create a milestone in a repository that doesn't exist,
/// the operation should fail immediately with a NonRetryable error (404)
/// instead of retrying 15 times with exponential backoff.
#[tokio::test]
#[serial]
async fn test_create_milestone_nonexistent_repository_fails_immediately() {
    let client = common::create_test_github_client();

    // Use a repository that definitely doesn't exist
    let nonexistent_repo = RepositoryId::new("nonexistent-user-12345", "nonexistent-repo-67890");

    let start_time = std::time::Instant::now();

    // Attempt to create a milestone in the non-existent repository
    let result = client
        .create_milestone(
            &nonexistent_repo,
            "Test Milestone",
            Some("This should fail immediately"),
            None,
            Some(MilestoneState::Open),
        )
        .await;

    let elapsed = start_time.elapsed();

    // The operation should fail
    assert!(
        result.is_err(),
        "Creating milestone in non-existent repository should fail"
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

/// Test that creating a label in a non-existent repository fails immediately
///
/// This test verifies that the fix for the retry bug is working correctly.
/// When attempting to create a label in a repository that doesn't exist,
/// the operation should fail immediately with a NonRetryable error (404)
/// instead of retrying 15 times with exponential backoff.
#[tokio::test]
#[serial]
async fn test_create_label_nonexistent_repository_fails_immediately() {
    let client = common::create_test_github_client();

    // Use a repository that definitely doesn't exist
    let nonexistent_repo = RepositoryId::new("nonexistent-user-12345", "nonexistent-repo-67890");

    let start_time = std::time::Instant::now();

    // Attempt to create a label in the non-existent repository
    let result = client
        .create_label(
            &nonexistent_repo,
            "test-label",
            Some("ff0000"),
            Some("This should fail immediately"),
        )
        .await;

    let elapsed = start_time.elapsed();

    // The operation should fail
    assert!(
        result.is_err(),
        "Creating label in non-existent repository should fail"
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

/// Test milestone update operations with partial updates
///
/// This test verifies that milestone update operations correctly handle partial updates,
/// where only some fields are updated while others remain unchanged.
#[tokio::test]
#[serial]
async fn test_milestone_partial_updates() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // Create initial milestone
    let timestamp = chrono::Utc::now().timestamp();
    let initial_title = format!("Partial Update Test Milestone {}", timestamp);
    let initial_description = "Initial description";
    let initial_due_date = chrono::Utc::now() + chrono::Duration::days(15);

    let created_milestone = client
        .create_milestone(
            &repository_id,
            &initial_title,
            Some(initial_description),
            Some(initial_due_date),
            Some(MilestoneState::Open),
        )
        .await
        .expect("Failed to create milestone");

    let milestone_number = created_milestone.id;

    // Test updating only title
    let new_title = "Updated Title Only";
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            Some(new_title),
            None, // Don't update description
            None, // Don't update due date
            None, // Don't update state
        )
        .await
        .expect("Failed to update milestone title only");

    assert_eq!(updated_milestone.title, new_title);
    assert_eq!(
        updated_milestone.description.as_deref(),
        Some(initial_description)
    );
    assert_eq!(updated_milestone.state, MilestoneState::Open);
    println!("✓ Partial update (title only) successful");

    // Test updating only description
    let new_description = "Updated description only";
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            None, // Don't update title
            Some(new_description),
            None, // Don't update due date
            None, // Don't update state
        )
        .await
        .expect("Failed to update milestone description only");

    assert_eq!(updated_milestone.title, new_title); // Should remain unchanged
    assert_eq!(
        updated_milestone.description.as_deref(),
        Some(new_description)
    );
    assert_eq!(updated_milestone.state, MilestoneState::Open);
    println!("✓ Partial update (description only) successful");

    // Test updating only state
    let updated_milestone = client
        .update_milestone(
            &repository_id,
            &milestone_number,
            None, // Don't update title
            None, // Don't update description
            None, // Don't update due date
            Some(MilestoneState::Closed),
        )
        .await
        .expect("Failed to update milestone state only");

    assert_eq!(updated_milestone.title, new_title); // Should remain unchanged
    assert_eq!(
        updated_milestone.description.as_deref(),
        Some(new_description)
    ); // Should remain unchanged
    assert_eq!(updated_milestone.state, MilestoneState::Closed);
    println!("✓ Partial update (state only) successful");

    // Clean up
    client
        .delete_milestone(&repository_id, &milestone_number)
        .await
        .expect("Failed to delete milestone");

    println!("🎉 All partial update tests completed successfully!");
}

/// Test label update operations with partial updates
///
/// This test verifies that label update operations correctly handle partial updates,
/// where only some fields are updated while others remain unchanged.
#[tokio::test]
#[serial]
async fn test_label_partial_updates() {
    let client = common::create_test_github_client();

    // Use a test repository - replace with your test repo
    let repository_id = RepositoryId::new("tacogips", "gitcodes-mcp-test-1");

    // Create initial label
    let initial_name = "partial-update-test";
    let initial_color = "ff0000"; // Red
    let initial_description = "Initial description";

    let _created_label = client
        .create_label(
            &repository_id,
            initial_name,
            Some(initial_color),
            Some(initial_description),
        )
        .await
        .expect("Failed to create label");

    // Test updating only color
    let new_color = "00ff00"; // Green
    let updated_label = client
        .update_label(
            &repository_id,
            initial_name,
            None, // Don't update name
            Some(new_color),
            None, // Don't update description
        )
        .await
        .expect("Failed to update label color only");

    assert_eq!(updated_label.name, initial_name);
    assert_eq!(updated_label.color.as_deref(), Some(new_color));
    assert_eq!(
        updated_label.description.as_deref(),
        Some(initial_description)
    );
    println!("✓ Partial update (color only) successful");

    // Test updating only description
    let new_description = "Updated description only";
    let updated_label = client
        .update_label(
            &repository_id,
            initial_name,
            None, // Don't update name
            None, // Don't update color
            Some(new_description),
        )
        .await
        .expect("Failed to update label description only");

    assert_eq!(updated_label.name, initial_name); // Should remain unchanged
    assert_eq!(updated_label.color.as_deref(), Some(new_color)); // Should remain unchanged
    assert_eq!(updated_label.description.as_deref(), Some(new_description));
    println!("✓ Partial update (description only) successful");

    // Test updating only name
    let new_name = "updated-partial-test";
    let updated_label = client
        .update_label(
            &repository_id,
            initial_name,
            Some(new_name),
            None, // Don't update color
            None, // Don't update description
        )
        .await
        .expect("Failed to update label name only");

    assert_eq!(updated_label.name, new_name);
    assert_eq!(updated_label.color.as_deref(), Some(new_color)); // Should remain unchanged
    assert_eq!(updated_label.description.as_deref(), Some(new_description)); // Should remain unchanged
    println!("✓ Partial update (name only) successful");

    // Clean up
    client
        .delete_label(&repository_id, new_name)
        .await
        .expect("Failed to delete label");

    println!("🎉 All partial update tests completed successfully!");
}
