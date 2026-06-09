# MonCheri draft tree

Status: draft. Domain, API host, final graph model, and SurrealDB details are not set in stone.

moncheri
├─ purpose
│  ├─ private searchable records app
│  ├─ user enters records through frontend
│  ├─ Rust API stores and amends records in SurrealDB
│  ├─ user searches from frontend
│  ├─ Tantivy returns candidate record IDs only
│  └─ Rust API checks SurrealDB permissions before returning results
│
├─ security_first
│  ├─ from_day_1
│  ├─ frontend_is_display_only
│  ├─ Rust_API_is_security_boundary
│  ├─ SurrealDB_holds_permission_data
│  └─ search_never_bypasses_permissions
│
├─ users
│  ├─ adminUser
│  │  └─ role: Administrator
│  ├─ standardUser
│  │  └─ role: Contributor
│  └─ guestUser
│     └─ role: Reader
│
├─ roles
│  ├─ Administrator
│  ├─ Contributor
│  └─ Reader
│
├─ permissions
│  ├─ create_item
│  ├─ amend_item
│  ├─ move_item
│  ├─ search_item
│  ├─ view_item
│  ├─ archive_item
│  ├─ restore_item
│  └─ delete_archived_item
│
├─ light_audit
│  ├─ fields
│  │  ├─ utc_at
│  │  ├─ user_id
│  │  ├─ action
│  │  └─ record_id
│  └─ actions
│     ├─ create_item
│     ├─ amend_item
│     ├─ move_item
│     ├─ archive_item
│     ├─ restore_item
│     └─ delete_archived_item
│
├─ frontend
│  ├─ folder: app
│  ├─ technology: Leptos CSR
│  ├─ current_host: GitHub Pages
│  └─ current_url: https://ictdepends.github.io/moncheri/
│
├─ api
│  ├─ folder: api
│  ├─ technology: Rust
│  ├─ host: not_decided
│  └─ first_endpoint: GET /health
│
├─ core
│  ├─ folder: core
│  ├─ technology: Rust library crate
│  └─ graph_model_status
│     ├─ not_final
│     ├─ flat_item_shape_not_accepted_as_final
│     └─ SurrealDB_graph_design_to_discuss_next
│
├─ SurrealDB
│  ├─ role: source_of_truth
│  ├─ exact_instance: not_created_yet
│  └─ graph_questions
│     ├─ what_is_record
│     ├─ what_is_item
│     ├─ what_is_location
│     ├─ what_is_container
│     ├─ what_is_node
│     ├─ what_is_relation
│     └─ how_permissions_follow_graph_paths
│
├─ Tantivy
│  ├─ role: derived_search_index
│  ├─ source_of_truth: no
│  ├─ returns: candidate_record_ids_only
│  └─ rebuildable_from: SurrealDB
│
├─ CI_CD
│  ├─ current_provider: GitHub Actions
│  ├─ current_status: working
│  └─ current_deploy_target: GitHub Pages
│
├─ domain
│  ├─ status: not_decided
│  └─ current_free_url: https://ictdepends.github.io/moncheri/
│
└─ next_design_discussion
   └─ SurrealDB_graph_model
