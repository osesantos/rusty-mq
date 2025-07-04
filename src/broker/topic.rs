// Description: This module provides functionality to match a topic against a pattern.
// Supports wildcard matching for MQTT-like topics.
pub fn match_topic(pattern: &str, topic: &str) -> bool {
    let pat_parts: Vec<&str> = pattern.split('.').collect();
    let topic_parts: Vec<&str> = topic.split('.').collect();

    for (i, pat) in pat_parts.iter().enumerate() {
        match *pat {
            ">" => return true,
            "*" => continue,
            _ => {
                if i >= topic_parts.len() || pat != &topic_parts[i] {
                    return false;
                }
            }
        }
    }

    pat_parts.len() == topic_parts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(match_topic("user.created", "user.created"));
        assert!(!match_topic("user.created", "user.updated"));
    }

    #[test]
    fn test_wildcard_match() {
        assert!(match_topic("user.*", "user.created"));
        assert!(match_topic("user.*", "user.updated"));
        assert!(!match_topic("user.*", "admin.created"));
    }

    #[test]
    fn test_multi_level_wildcard() {
        assert!(match_topic("user.>", "user.created"));
        assert!(match_topic("user.>", "user.updated.profile"));
        assert!(match_topic(">", "anything.really.anything"));
        assert!(!match_topic("user.>", "admin.created"));
    }

    #[test]
    fn test_mixed_patterns() {
        assert!(match_topic("user.*.profile", "user.created.profile"));
        assert!(match_topic("user.>.profile", "user.created.profile"));
        assert!(!match_topic("user.*.profile", "user.created"));
        assert!(!match_topic("user.>.profile", "admin.created.profile"));
    }

    #[test]
    fn test_mismatched_length() {
        assert!(match_topic("user.>", "user.created.profile"));
        assert!(!match_topic("user.created", "user.created.profile"));
        assert!(!match_topic("user.*", "user.created.profile"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(match_topic(">", "any.topic"));
        assert!(match_topic("user.>", "user.anything"));
        assert!(match_topic("user.>", "user"));
        assert!(match_topic("*", "*"));
        assert!(!match_topic("user.*", "user"));
        assert!(!match_topic("*", "user.created"));
    }
}
