use ammonia::clean;
use html_escape::encode_text;
use regex::Regex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

/// Input sanitization service for preventing XSS attacks
#[allow(dead_code)]
pub struct InputSanitizerService {
    html_cleaner: ammonia::Builder<'static>,
}

impl Default for InputSanitizerService {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl InputSanitizerService {
    pub fn new() -> Self {
        // Configure HTML sanitizer with safe tags and attributes
        let mut html_cleaner = ammonia::Builder::new();
        
        // Allow only safe HTML tags for content
        let allowed_tags: HashSet<&str> = [
            "p", "br", "strong", "em", "u", "h1", "h2", "h3", "h4", "h5", "h6",
            "ul", "ol", "li", "blockquote", "code", "pre", "a", "img"
        ].iter().cloned().collect();
        
        let allowed_attributes: HashSet<&str> = [
            "href", "src", "alt", "title", "class"
        ].iter().cloned().collect();
        
        html_cleaner
            .tags(allowed_tags)
            .generic_attributes(allowed_attributes)
            .link_rel(Some("noopener noreferrer"))
            .url_schemes(maplit::hashset!["http", "https", "mailto"]);
        
        Self { html_cleaner }
    }
    
    /// Sanitize HTML content for safe display
    pub fn sanitize_html(&self, input: &str) -> String {
        clean(input)
    }
    
    /// Sanitize text content by escaping HTML entities
    pub fn sanitize_text(&self, input: &str) -> String {
        encode_text(input).to_string()
    }
    
    /// Sanitize user input for database storage (prevents most injection attacks)
    pub fn sanitize_user_input(&self, input: &str) -> String {
        // Remove null bytes and control characters
        let cleaned = self.remove_control_characters(input);
        
        // Limit length to prevent DoS
        let truncated = if cleaned.len() > 10000 {
            format!("{}...", &cleaned[..10000])
        } else {
            cleaned
        };
        
        // Escape HTML entities to prevent XSS
        self.sanitize_text(&truncated)
    }
    
    /// Sanitize content for rich text editors (allows safe HTML)
    pub fn sanitize_rich_content(&self, input: &str) -> String {
        self.html_cleaner.clean(input).to_string()
    }
    
    /// Remove potentially dangerous control characters
    fn remove_control_characters(&self, input: &str) -> String {
        static CONTROL_CHAR_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap()
        });
        
        CONTROL_CHAR_REGEX.replace_all(input, "").to_string()
    }
    
    /// Validate and sanitize URL input
    pub fn sanitize_url(&self, input: &str) -> Result<String, String> {
        // Basic URL validation
        if input.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        
        // Check for dangerous protocols
        let dangerous_protocols = ["javascript:", "data:", "vbscript:", "file:"];
        let lower_input = input.to_lowercase();
        
        for protocol in &dangerous_protocols {
            if lower_input.starts_with(protocol) {
                return Err("Dangerous URL protocol detected".to_string());
            }
        }
        
        // Allow only HTTP(S) and mailto
        if !lower_input.starts_with("http://") 
            && !lower_input.starts_with("https://") 
            && !lower_input.starts_with("mailto:") 
            && !lower_input.starts_with("/") {
            return Err("Only HTTP(S), mailto, and relative URLs are allowed".to_string());
        }
        
        Ok(input.to_string())
    }
    
    /// Sanitize SQL-like input (basic protection, use parameterized queries!)
    pub fn sanitize_sql_input(&self, input: &str) -> String {
        static SQL_INJECTION_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter|exec|execute|declare|cast|char|nchar|varchar|nvarchar|syscolumns|sysobjects|information_schema)").unwrap()
        });
        
        // Replace potential SQL keywords with harmless text
        SQL_INJECTION_REGEX.replace_all(input, "[FILTERED]").to_string()
    }
    
    /// Validate email format (basic validation)
    pub fn validate_email(&self, email: &str) -> Result<String, String> {
        static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        });
        
        if EMAIL_REGEX.is_match(email) {
            Ok(email.to_lowercase())
        } else {
            Err("Invalid email format".to_string())
        }
    }
    
    /// Sanitize username (alphanumeric + limited special chars)
    pub fn sanitize_username(&self, username: &str) -> Result<String, String> {
        static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^[a-zA-Z0-9_.-]{3,50}$").unwrap()
        });
        
        if USERNAME_REGEX.is_match(username) {
            Ok(username.to_string())
        } else {
            Err("Username must be 3-50 characters and contain only letters, numbers, dots, hyphens, and underscores".to_string())
        }
    }
}

/// Helper functions for common sanitization needs
#[allow(dead_code)]
pub fn escape_html(text: &str) -> String {
    encode_text(text).to_string()
}

#[allow(dead_code)]
pub fn sanitize_search_query(query: &str) -> String {
    // Remove special regex characters that could be used for injection
    static SEARCH_SANITIZE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[.*+?^${}()|[\]\\]").unwrap()
    });
    
    SEARCH_SANITIZE_REGEX.replace_all(query, "\\$0").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_sanitization() {
        let sanitizer = InputSanitizerService::new();
        let malicious_html = r#"<script>alert('xss')</script><p>Safe content</p>"#;
        let result = sanitizer.sanitize_html(malicious_html);
        
        assert!(!result.contains("<script>"));
        assert!(result.contains("Safe content"));
    }

    #[test]
    fn test_text_escaping() {
        let sanitizer = InputSanitizerService::new();
        let text_with_html = "<div>Test & 'quotes'</div>";
        let result = sanitizer.sanitize_text(text_with_html);
        
        assert!(result.contains("&lt;div&gt;"));
        assert!(result.contains("&amp;"));
    }

    #[test]
    fn test_url_validation() {
        let sanitizer = InputSanitizerService::new();
        
        assert!(sanitizer.sanitize_url("https://example.com").is_ok());
        assert!(sanitizer.sanitize_url("javascript:alert(1)").is_err());
        assert!(sanitizer.sanitize_url("data:text/html,<script>").is_err());
    }

    #[test]
    fn test_email_validation() {
        let sanitizer = InputSanitizerService::new();
        
        assert!(sanitizer.validate_email("test@example.com").is_ok());
        assert!(sanitizer.validate_email("invalid-email").is_err());
        assert!(sanitizer.validate_email("test@").is_err());
    }
}