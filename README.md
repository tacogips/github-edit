# Github Edit MCP Server

A powerful Model Context Protocol (MCP) server that provides AI assistants with direct access to GitHub repositories, issues, pull requests, and project data. Built in Rust for performance and reliability.

## Features

### 🔧 **GitHub Repository Management**
- Create, edit, and manage GitHub issues with comprehensive metadata
- Create, edit, and manage pull requests with review workflows
- Advanced project management with GitHub Projects integration
- Repository administration (milestones, labels, basic configuration)
- Direct API-based operations for efficient GitHub resource management

### 📊 **Advanced Project Management**
- Update GitHub Projects fields (text, number, date, single/multi-select)
- Add issues and pull requests to projects with proper linking
- Manage project item metadata and status tracking
- Full GitHub Projects (beta) integration with field management
- Project node ID resolution for GraphQL operations

### 🎯 **Issue & Pull Request Operations**
- Create, edit, comment on, and manage issue lifecycle
- Create, edit, comment on, and manage pull request workflows
- Add/remove assignees, reviewers, labels, and milestones
- State management (open/closed) with proper transitions
- Comment editing and management with full versioning

### 🔧 **Repository Administration**
- Create and manage repository milestones with due dates
- Create, update, and delete repository labels with colors
- Repository-level configuration and metadata management
- Direct API access for administrative operations

### 🏗️ **Transport & Integration**
- STDIO transport for Claude Desktop integration
- HTTP/SSE transport for web-based access
- Model Context Protocol (MCP) 2024-11-05 compliance
- JSON-RPC 2.0 protocol with proper error handling

## Installation

### Prerequisites
- Rust 2024 edition
- GitHub Personal Access Token (for API access)
- Required token permissions: `repo`, `project`, `read:org`

### From Source
```bash
git clone https://github.com/tacogips/github-edit
cd github-edit
CARGO_TERM_QUIET=true cargo build --release
```

### Binaries
Two main binaries are built:
- `github-edit-mcp`: MCP server for AI assistant integration
- `github-edit-cli`: Command-line interface for direct usage

## Quick Start

### 1. Set up GitHub Token
```bash
export GITHUB_EDIT_GITHUB_TOKEN="ghp_your_token_here"
```

### 2. Run MCP Server
```bash
# STDIO mode (for Claude Desktop)
./target/release/github-edit-mcp stdio --github-token $GITHUB_EDIT_GITHUB_TOKEN

# HTTP mode (for web integrations) 
./target/release/github-edit-mcp http --address 0.0.0.0:8080 --github-token $GITHUB_EDIT_GITHUB_TOKEN

# Enable debug mode and sync operations
./target/release/github-edit-mcp stdio --debug --sync
```

### 3. Use CLI Tools
```bash
# Create a new issue
./target/release/github-edit-cli issue create -r https://github.com/owner/repo -t "Bug: App crashes" -b "Detailed description..."

# Add comment to issue
./target/release/github-edit-cli issue comment -r https://github.com/owner/repo -i 123 -b "I can confirm this bug"

# Create a new pull request
./target/release/github-edit-cli pull-request create -r https://github.com/owner/repo -t "Fix auth bug" --head feature-branch --base main

# Update project field
./target/release/github-edit-cli project update-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --field-type text --value "In Progress"
```

## Claude Desktop Integration

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "github-edit": {
      "command": "/path/to/github-edit-mcp",
      "args": ["stdio"],
      "env": {
        "GITHUB_EDIT_GITHUB_TOKEN": "ghp_your_token_here"
      }
    }
  }
}
```

## MCP Tools

### Project Management Tools

#### `update_project_item_field`
Update a project item field using string parameters. Supports text, number, date, single_select, and multi_select field types.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "project_item_id": "PVTI_lADOBw6lbs4AAVGQzgF6sCo",
  "project_field_id": "PVTF_lADOBw6lbs4AAVGQzgF6sCo",
  "field_type": "text",
  "value": "In Progress"
}
```

#### `get_project_node_id`
Get project node ID from project identifier.

```json
{
  "project_owner": "octocat",
  "project_number": 1,
  "project_type": "user"
}
```

#### `update_project_item_text_field`
Update a project item text field.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "project_item_id": "PVTI_lADOBw6lbs4AAVGQzgF6sCo",
  "project_field_id": "PVTF_lADOBw6lbs4AAVGQzgF6sCo",
  "text_value": "Ready for review"
}
```

#### `update_project_item_number_field`
Update a project item number field.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "project_item_id": "PVTI_lADOBw6lbs4AAVGQzgF6sCo",
  "project_field_id": "PVTF_lADOBw6lbs4AAVGQzgF6sCo",
  "number_value": 85
}
```

#### `update_project_item_date_field`
Update a project item date field.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "project_item_id": "PVTI_lADOBw6lbs4AAVGQzgF6sCo",
  "project_field_id": "PVTF_lADOBw6lbs4AAVGQzgF6sCo",
  "date_value": "2024-01-15T10:30:00Z"
}
```

#### `update_project_item_single_select_field`
Update a project item single select field.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "project_item_id": "PVTI_lADOBw6lbs4AAVGQzgF6sCo",
  "project_field_id": "PVTF_lADOBw6lbs4AAVGQzgF6sCo",
  "option_id": "f75ad846"
}
```

#### `add_issue_to_project`
Add an issue to a project.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "repository_owner": "octocat",
  "repository_name": "Hello-World",
  "issue_number": 123
}
```

#### `add_pull_request_to_project`
Add a pull request to a project.

```json
{
  "project_node_id": "PN_kwDOBw6lbs4AAVGQ",
  "repository_owner": "octocat",
  "repository_name": "Hello-World",
  "pull_request_number": 456
}
```

### Pull Request Management Tools

#### `create_pull_request`
Create a new pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "title": "Fix authentication bug",
  "head_branch": "feature-auth-fix",
  "base_branch": "main",
  "body": "This PR fixes the authentication bug by...",
  "draft": false
}
```

#### `add_comment_to_pull_request`
Add a comment to a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "body": "LGTM! Great work on this fix."
}
```

#### `edit_comment_on_pull_request`
Edit an existing pull request comment.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "comment_number": 456,
  "body": "Updated comment with clarification..."
}
```

#### `close_pull_request`
Close a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123
}
```

#### `edit_pull_request_title`
Edit the title of a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "title": "Updated: Fix authentication bug with OAuth flow"
}
```

#### `edit_pull_request_body`
Edit the body of a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "body": "Updated implementation with performance improvements..."
}
```

#### `add_assignees_to_pull_request`
Add assignees to a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "new_assignees": ["user1", "user2"]
}
```

#### `remove_assignees_from_pull_request`
Remove assignees from a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "assignees": ["user1", "user2"]
}
```

#### `add_requested_reviewers_to_pull_request`
Add requested reviewers to a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "new_reviewers": ["reviewer1", "reviewer2"]
}
```

#### `add_labels_to_pull_request`
Add labels to a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "labels": ["bug", "critical"]
}
```

#### `remove_labels_from_pull_request`
Remove labels from a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "labels": ["bug", "critical"]
}
```

#### `add_milestone_to_pull_request`
Add milestone to a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123,
  "milestone_number": 5
}
```

#### `remove_milestone_from_pull_request`
Remove milestone from a pull request.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "pr_number": 123
}
```

### Issue Management Tools

#### `create_issue`
Create a new issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "title": "Bug: Application crashes on startup",
  "body": "When I run the application with...",
  "assignees": ["user1", "user2"],
  "labels": ["bug", "critical"],
  "milestone_number": 1
}
```

#### `add_comment_to_issue`
Add a comment to an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "body": "I can confirm this bug on macOS 13.2"
}
```

#### `edit_comment_on_issue`
Edit an existing issue comment.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "comment_number": 456,
  "body": "Updated comment with clarification..."
}
```

#### `edit_issue_title`
Edit the title of an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "title": "Updated: Bug found in authentication module"
}
```

#### `edit_issue_body`
Edit the body of an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "body": "Updated description with reproduction steps..."
}
```

#### `update_issue_state`
Update the state of an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "state": "closed"
}
```

#### `add_assignees_to_issue`
Add assignees to an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "new_assignees": ["user1", "user2"]
}
```

#### `remove_assignees_from_issue`
Remove assignees from an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "assignees": ["user1", "user2"]
}
```

#### `add_labels_to_issue`
Add labels to an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "labels": ["bug", "critical"]
}
```

#### `add_milestone_to_issue`
Add milestone to an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "milestone_number": 1
}
```

#### `remove_labels_from_issue`
Remove labels from an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123,
  "labels": ["bug", "critical"]
}
```

#### `remove_milestone_from_issue`
Remove milestone from an issue.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "issue_number": 123
}
```

### Repository Management Tools

#### `create_milestone`
Create a new milestone in a repository.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "title": "v1.0.0",
  "description": "Initial release",
  "due_on": "2024-01-15T10:30:00Z",
  "state": "open"
}
```

#### `create_label`
Create a new label in a repository.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "name": "bug",
  "color": "ff0000",
  "description": "Something isn't working"
}
```

#### `update_label`
Update an existing label in a repository.

```json
{
  "repository_url": "https://github.com/owner/repo",
  "old_name": "bug",
  "new_name": "critical-bug",
  "color": "ff0000",
  "description": "Critical issue requiring immediate attention"
}
```

#### Output Format Options
- **`light`** (default): Minimal information including title, status, truncated body, and key metadata
- **`rich`**: Comprehensive details including full body, comments, labels, assignees, dates, and all metadata

## CLI Commands

The GitHub Edit CLI provides comprehensive GitHub resource management capabilities focused on editing and updating operations.

### Issue Management
```bash
# Get issue details
github-edit-cli issue get https://github.com/owner/repo/issues/123

# Create a new issue
github-edit-cli issue create -r https://github.com/owner/repo -t "Bug: App crashes" -b "Detailed description..."

# Add comment to issue
github-edit-cli issue comment -r https://github.com/owner/repo -i 123 -b "I can confirm this bug"

# Edit issue title
github-edit-cli issue edit-title -r https://github.com/owner/repo -i 123 -t "Updated title"

# Edit issue body
github-edit-cli issue edit-body -r https://github.com/owner/repo -i 123 -b "Updated description"

# Update issue state
github-edit-cli issue update-state -r https://github.com/owner/repo -i 123 -s closed

# Edit issue comment
github-edit-cli issue edit-comment -r https://github.com/owner/repo -i 123 -c 456 -b "Updated comment"

# Delete issue comment
github-edit-cli issue delete-comment -r https://github.com/owner/repo -i 123 -c 456

# Add/remove assignees
github-edit-cli issue add-assignees -r https://github.com/owner/repo -i 123 -a user1,user2
github-edit-cli issue remove-assignees -r https://github.com/owner/repo -i 123 -a user1,user2

# Remove labels
github-edit-cli issue remove-labels -r https://github.com/owner/repo -i 123 -l bug,enhancement

# Delete issue
github-edit-cli issue delete -r https://github.com/owner/repo -i 123

# Set/remove milestone
github-edit-cli issue set-milestone -r https://github.com/owner/repo -i 123 -m 1
github-edit-cli issue remove-milestone -r https://github.com/owner/repo -i 123
```

### Pull Request Management
```bash
# Note: pull-request get command is currently disabled

# Create a new pull request
github-edit-cli pull-request create -r https://github.com/owner/repo -t "Fix auth bug" --head feature-branch --base main

# Add comment to pull request
github-edit-cli pull-request comment -r https://github.com/owner/repo -p 123 -b "LGTM!"

# Close pull request
github-edit-cli pull-request close -r https://github.com/owner/repo -p 123

# Edit pull request title/body
github-edit-cli pull-request edit-title -r https://github.com/owner/repo -p 123 -t "Updated title"
github-edit-cli pull-request edit-body -r https://github.com/owner/repo -p 123 -b "Updated description"

# Edit/delete pull request comments
github-edit-cli pull-request edit-comment -r https://github.com/owner/repo -p 123 -c 456 -b "Updated comment"
github-edit-cli pull-request delete-comment -r https://github.com/owner/repo -p 123 -c 456

# Manage assignees and reviewers
github-edit-cli pull-request add-assignees -r https://github.com/owner/repo -p 123 -a user1,user2
github-edit-cli pull-request remove-assignees -r https://github.com/owner/repo -p 123 -a user1,user2
github-edit-cli pull-request add-reviewers -r https://github.com/owner/repo -p 123 -u reviewer1,reviewer2

# Manage labels
github-edit-cli pull-request add-labels -r https://github.com/owner/repo -p 123 -l bug,critical
github-edit-cli pull-request remove-labels -r https://github.com/owner/repo -p 123 -l bug,critical

# Manage milestones
github-edit-cli pull-request add-milestone -r https://github.com/owner/repo -p 123 -m 5
github-edit-cli pull-request remove-milestone -r https://github.com/owner/repo -p 123
```

### Project Management
```bash
# Update project field (generic)
github-edit-cli project update-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --field-type text --value "In Progress"

# Update specific field types
github-edit-cli project update-text-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --text-value "Ready for review"
github-edit-cli project update-number-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --number-value 85
github-edit-cli project update-date-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --date-value "2024-12-31T23:59:59Z"
github-edit-cli project update-single-select-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --option-id "f75ad846"

# Add items to project
github-edit-cli project add-issue --project-node-id "PN_xxx" --owner "octocat" --repo "Hello-World" --issue-number 123
github-edit-cli project add-pull-request --project-node-id "PN_xxx" --owner "octocat" --repo "Hello-World" --pull-request-number 456
```

### Repository Management
```bash
# Create milestone
github-edit-cli repository create-milestone -r https://github.com/owner/repo -t "v1.0.0" -d "Initial release"

# Update milestone
github-edit-cli repository update-milestone -r https://github.com/owner/repo -m 1 -t "v1.0.1"

# Delete milestone
github-edit-cli repository delete-milestone -r https://github.com/owner/repo -m 1

# Create label
github-edit-cli repository create-label -r https://github.com/owner/repo -n "bug" -c "ff0000" -d "Something isn't working"

# Update label
github-edit-cli repository update-label -r https://github.com/owner/repo -o "bug" -n "critical-bug"

# Delete label
github-edit-cli repository delete-label -r https://github.com/owner/repo -n "bug"
```

## Configuration

### Environment Variables
- `GITHUB_EDIT_GITHUB_TOKEN`: GitHub Personal Access Token
- `GITHUB_EDIT_PROFILE`: Default profile name
- `GITHUB_EDIT_CONFIG_DIR`: Custom configuration directory

### GitHub Token Permissions
Your GitHub token needs the following permissions:
- `repo`: Access to repository data and issues
- `project`: Access to GitHub Projects (beta)
- `read:org`: Access to organization projects
- `read:user`: Access to user profile information

### Profile Configuration
Profiles are stored in `~/.config/github-edit/profiles/` (or system equivalent).

## Development

### Building
```bash
CARGO_TERM_QUIET=true cargo build
```

### Testing
```bash
CARGO_TERM_QUIET=true cargo test
```

### Linting
```bash
CARGO_TERM_QUIET=true cargo clippy
```

### Documentation
```bash
CARGO_TERM_QUIET=true cargo doc --no-deps
```

### Test Resources
- Test Repository: https://github.com/tacogips/gitcodes-mcp-test-1
- Test Project: https://github.com/users/tacogips/projects/1

## Architecture

### Core Modules
- **`github`**: GitHub API client using octocrab with retry logic and rate limiting
- **`tools`**: MCP tool implementations for issues, PRs, projects, and repositories
- **`transport`**: MCP server transport layers (stdio, HTTP/SSE)
- **`types`**: Core data structures and domain models for GitHub entities
- **`bin`**: CLI and MCP server binaries
- **`services`**: Business logic layer (available but unused by current implementation)

### Key Technologies
- **Rust 2024**: Modern, safe systems programming
- **Tokio**: Async runtime for high-performance networking
- **Octocrab**: GitHub API client with REST and GraphQL support
- **rmcp**: Model Context Protocol implementation
- **Serde**: Serialization framework
- **Clap**: Command-line argument parsing

## Performance Features

- **Async/await**: Non-blocking I/O for high concurrency
- **Connection pooling**: Efficient GitHub API usage with timeout handling
- **Rate limiting**: Respects GitHub API limits with exponential backoff retry logic
- **Error handling**: Comprehensive error propagation with anyhow
- **Authentication**: GitHub Personal Access Token with secure handling
- **Protocol compliance**: MCP 2024-11-05 with JSON-RPC 2.0

## Security

- **Token security**: Tokens are never logged or exposed
- **TLS**: All connections use rustls for security
- **Input validation**: All inputs are validated and sanitized
- **Error handling**: Comprehensive error handling with anyhow

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the coding guidelines
4. Add tests for new functionality
5. Run the full test suite
6. Submit a pull request

### Coding Guidelines
- Follow Rust 2024 edition conventions
- Use `rustfmt` for formatting
- Run `clippy` for linting
- Write comprehensive tests
- Document public APIs

## License

MIT License - see LICENSE file for details.

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/tacogips/github-edit/issues
- Repository: https://github.com/tacogips/github-edit
