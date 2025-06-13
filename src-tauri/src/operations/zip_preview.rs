use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::operations::get_archive_content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeNode {
    pub key: String,
    pub name: String,
    pub path: String,
    pub node_type: String, // "file" or "directory"
    pub children: Option<Vec<FileTreeNode>>,
    pub expanded: bool,
    pub level: u32,
}

// Build file tree structure from flat file paths
pub fn build_file_tree(paths: Vec<String>) -> Vec<FileTreeNode> {
    let mut dir_map = HashMap::new();
    let mut all_nodes = HashMap::new();

    // Build directory map for quick lookup
    for path in &paths {
        let parts: Vec<&str> = path.split('\\').collect();
        let mut current_path = String::new();
        for part in parts.iter().take(parts.len().saturating_sub(1)) {
            if part.is_empty() {
                continue;
            }
            current_path = if current_path.is_empty() {
                part.to_string()
            } else {
                format!("{}\\{}", current_path, part)
            };
            dir_map.insert(current_path.clone(), true);
        }
    }

    // Create all nodes first
    for path in paths {
        if path.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = path.split('\\').filter(|p| !p.is_empty()).collect();
        if parts.is_empty() {
            continue;
        }

        let mut current_path = String::new();

        for (index, part) in parts.iter().enumerate() {
            current_path = if current_path.is_empty() {
                part.to_string()
            } else {
                format!("{}\\{}", current_path, part)
            };

            let is_last = index == parts.len() - 1;
            let is_file = is_last && !dir_map.contains_key(&current_path);

            // Create node if it doesn't exist
            if !all_nodes.contains_key(&current_path) {
                let node = FileTreeNode {
                    key: current_path.clone(),
                    name: part.to_string(),
                    path: current_path.clone(),
                    node_type: if is_file {
                        "file".to_string()
                    } else {
                        "directory".to_string()
                    },
                    level: index as u32,
                    children: if is_file { None } else { Some(Vec::new()) },
                    expanded: false,
                };
                all_nodes.insert(current_path.clone(), node);
            }
        }
    }

    // Build hierarchy
    let mut root_nodes = Vec::new();
    let mut sorted_paths: Vec<String> = all_nodes.keys().cloned().collect();
    sorted_paths.sort();

    for path in sorted_paths {
        if let Some(node) = all_nodes.remove(&path) {
            // Find parent path
            let parts: Vec<&str> = path.split('\\').collect();

            if parts.len() == 1 {
                // Root level node
                root_nodes.push(node);
            } else {
                // Find parent
                let parent_path = parts[..parts.len() - 1].join("\\");

                // Add to parent's children (we'll handle this after collecting all nodes)
                // For now, just add to root if parent not found
                if all_nodes.contains_key(&parent_path) {
                    // Parent will be processed later, skip for now
                    all_nodes.insert(path, node);
                } else {
                    // Check if parent is already in root_nodes
                    let mut found_parent = false;
                    for root_node in &mut root_nodes {
                        if add_child_to_node(root_node, &parent_path, node.clone()) {
                            found_parent = true;
                            break;
                        }
                    }
                    if !found_parent {
                        root_nodes.push(node);
                    }
                }
            }
        }
    }

    // Handle remaining nodes
    while !all_nodes.is_empty() {
        let mut processed_any = false;
        let remaining_paths: Vec<String> = all_nodes.keys().cloned().collect();

        for path in remaining_paths {
            if let Some(node) = all_nodes.get(&path) {
                let parts: Vec<&str> = path.split('\\').collect();
                let parent_path = parts[..parts.len() - 1].join("\\");

                let mut found_parent = false;
                for root_node in &mut root_nodes {
                    if add_child_to_node(root_node, &parent_path, node.clone()) {
                        all_nodes.remove(&path);
                        found_parent = true;
                        processed_any = true;
                        break;
                    }
                }

                if !found_parent && parts.len() == 1 {
                    // This should be a root node
                    if let Some(node) = all_nodes.remove(&path) {
                        root_nodes.push(node);
                        processed_any = true;
                    }
                }
            }
        }

        // Prevent infinite loop
        if !processed_any {
            break;
        }
    }

    // Sort nodes
    sort_nodes(&mut root_nodes);

    // Auto-expand if single top folder
    if root_nodes.len() == 1 && root_nodes[0].children.is_some() {
        root_nodes[0].expanded = true;
    }

    root_nodes
}

// Helper function to add child to appropriate parent
fn add_child_to_node(node: &mut FileTreeNode, parent_path: &str, child: FileTreeNode) -> bool {
    if node.path == parent_path {
        if let Some(ref mut children) = node.children {
            children.push(child);
            return true;
        }
    }

    if let Some(ref mut children) = node.children {
        for child_node in children {
            if add_child_to_node(child_node, parent_path, child.clone()) {
                return true;
            }
        }
    }

    false
}

fn sort_nodes(nodes: &mut Vec<FileTreeNode>) {
    nodes.sort_by(|a, b| {
        // Directories first
        if a.node_type != b.node_type {
            return if a.node_type == "directory" {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            };
        }
        // Alphabetical by name
        a.name.cmp(&b.name)
    });

    // Sort children recursively
    for node in nodes {
        if let Some(ref mut children) = node.children {
            if !children.is_empty() {
                sort_nodes(children);
            }
        }
    }
}

// Get archive content with built file tree
pub async fn get_archive_tree(
    path: String,
    password: Option<String>,
) -> Result<Vec<FileTreeNode>, Box<dyn Error + Send + Sync>> {
    let file_paths = get_archive_content(path, password).await?;
    Ok(build_file_tree(file_paths))
}
