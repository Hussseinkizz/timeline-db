struct Node {
    id: String,
    name: String,
    author: String,
    version: u64,
    author_id: String, // id of user who created this node
    created_at: String,
    updated_at: String,
    active: bool,
    parent_id: String, // id of parent node
    branch_id: String, // id of branch this node belongs to
    data: String,      // JSON string for the node's data
    schema_id: String,
    latest_commit_id: String, // id of latest commit
}

struct Schema {
    id: String,
    name: String,
    version: u64,
    description: String,
    created_at: String,
    updated_at: String,
    active: bool,
    properties: Vec<PropertySchema>,
    depends_on_schemas: Vec<String>, // Tracks dependencies on other schemas
}

struct PropertyReference {
    property_id: String, // id of property this property references
    property_name: String,
    schema_id: String,
    required: bool,
    node_schema_id: String,
    node_id: String,
}

struct PropertySchema {
    id: String, // id of the property
    name: String,
    description: String,
    property_type: String,  // string, number, boolean, object, array, etc.
    node_schema_id: String, // id of schema
    required: bool,
    default_value: String,
    references: Vec<PropertyReference>,
}

struct Commit {
    id: String,
    node_id: String,                    // Links commit to node
    author_id: String,                  // User who created this commit
    previous_commit_id: Option<String>, // Tracks commit history (linked list)
    author: String,
    created_at: String,
    changes: Vec<Change>,
}

struct Change {
    id: String,
    operation: String,      // create or update or delete
    property: String,       // property name
    property_id: String,    // id of property
    new_value: String,      // value of property
    previous_value: String, // previous value of property
}

struct Branch {
    id: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
    parent_id: Option<String>, // id of the parent branch
    active: bool,
    user_ids: Vec<String>,            // Users in this branch
    node_ids: Vec<String>,            // Nodes in this branch
    latest_commit_id: Option<String>, // Tracks latest commit in this branch
}

struct VersionChange {
    id: String,
    operation: String,      // merge, commit, reverse, rebase, cherry-pick, etc.
    source_node_id: String, // Source node (if applicable)
    target_node_id: String, // Target node (if applicable)
    source_branch_id: Option<String>, // The branch where the change originates
    target_branch_id: Option<String>, // The branch where the change is applied
    created_at: String,     // Timestamp of the operation
    author_id: String,      // User who initiated the change
    commit_id: Option<String>, // Resulting commit (if applicable)
    previous_version_change_id: Option<String>, // Tracks reversals
    conflicts: Vec<Change>, // Tracks merge conflicts
    resolved_by: Option<String>, // ID of the user who resolved conflicts (if any)
    resolved_at: Option<String>, // Timestamp when conflicts were resolved
}

struct User {
    id: String,
    name: String,
    email: String,
    password: String, // hashed
    role: String,
    role_group: String,
    created_at: String,
    updated_at: String,
    active: bool,
}

struct Permission {
    id: String,
    name: String,
    operation: String, // read or write or delete or update or all
    description: String,
    created_at: String,
    updated_at: String,
    active: bool,
    resource_id: String, // node id
    resource_name: String,
    user_ids: Vec<String>,       // users who are granted this permission
    role_ids: Vec<String>,       // roles permitted to this permission
    role_group_ids: Vec<String>, // role groups permitted to this permission
}
