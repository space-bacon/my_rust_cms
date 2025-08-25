use std::collections::HashMap;

/// File type validation service for secure uploads
pub struct FileSecurityService {
    allowed_types: HashMap<String, Vec<u8>>,
    max_file_size: usize,
}

impl FileSecurityService {
    pub fn new(max_file_size: usize) -> Self {
        let mut allowed_types = HashMap::new();
        
        // Image formats (safe)
        allowed_types.insert("image/jpeg".to_string(), vec![0xFF, 0xD8, 0xFF]);
        allowed_types.insert("image/png".to_string(), vec![0x89, 0x50, 0x4E, 0x47]);
        allowed_types.insert("image/gif".to_string(), vec![0x47, 0x49, 0x46, 0x38]);
        allowed_types.insert("image/webp".to_string(), vec![0x52, 0x49, 0x46, 0x46]);
        allowed_types.insert("image/svg+xml".to_string(), vec![]); // SVG files are XML-based
        
        // Document formats (limited)
        allowed_types.insert("text/plain".to_string(), vec![]); // Text files don't have magic bytes
        allowed_types.insert("application/pdf".to_string(), vec![0x25, 0x50, 0x44, 0x46]);
        
        // Archive formats (with caution) 
        allowed_types.insert("application/zip".to_string(), vec![0x50, 0x4B]);
        
        Self {
            allowed_types,
            max_file_size,
        }
    }
    
    /// Validate file by checking magic bytes and size
    pub fn validate_file(&self, content_type: &str, data: &[u8]) -> Result<(), FileSecurityError> {
        // Check file size
        if data.len() > self.max_file_size {
            return Err(FileSecurityError::FileTooLarge(data.len(), self.max_file_size));
        }
        
        // Check if content type is allowed
        let magic_bytes = self.allowed_types.get(content_type)
            .ok_or_else(|| FileSecurityError::UnsupportedFileType(content_type.to_string()))?;
        
        // For text files, perform additional validation
        if content_type == "text/plain" {
            return self.validate_text_file(data);
        }
        
        // For SVG files, perform special validation
        if content_type == "image/svg+xml" {
            return self.validate_svg_file(data);
        }
        
        // Check magic bytes if they exist for this file type
        if !magic_bytes.is_empty() && data.len() >= magic_bytes.len() {
            if &data[..magic_bytes.len()] != magic_bytes.as_slice() {
                return Err(FileSecurityError::InvalidMagicBytes(content_type.to_string()));
            }
        }
        
        // Additional security checks
        self.scan_for_malicious_content(data)?;
        
        Ok(())
    }
    
    /// Validate text files for malicious content
    fn validate_text_file(&self, data: &[u8]) -> Result<(), FileSecurityError> {
        // Check if it's valid UTF-8
        let text = std::str::from_utf8(data)
            .map_err(|_| FileSecurityError::InvalidTextEncoding)?;
        
        // Check for suspicious patterns that might indicate malicious content
        let suspicious_patterns = [
            "<script",
            "javascript:",
            "vbscript:",
            "onload=",
            "onerror=",
            "onclick=",
            "eval(",
            "exec(",
        ];
        
        let lower_text = text.to_lowercase();
        for pattern in &suspicious_patterns {
            if lower_text.contains(pattern) {
                return Err(FileSecurityError::SuspiciousContent(pattern.to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Validate SVG files for malicious content
    fn validate_svg_file(&self, data: &[u8]) -> Result<(), FileSecurityError> {
        // Check if it's valid UTF-8
        let svg_content = std::str::from_utf8(data)
            .map_err(|_| FileSecurityError::InvalidTextEncoding)?;
        
        // Check if it starts with XML declaration or SVG tag
        let trimmed = svg_content.trim();
        if !trimmed.starts_with("<?xml") && !trimmed.starts_with("<svg") {
            return Err(FileSecurityError::InvalidSvgFormat);
        }
        
        // Check for dangerous SVG elements and attributes
        let dangerous_patterns = [
            "<script",           // JavaScript execution
            "javascript:",       // JavaScript URLs
            "data:text/html",    // HTML data URLs
            "data:image/svg+xml", // Nested SVG
            "<foreignObject",    // Can embed HTML
            "<iframe",           // Can embed external content
            "<object",           // Can embed external content
            "<embed",            // Can embed external content
            "<link",             // Can load external resources
            "<meta",             // Can contain redirects
            "onload=",           // Event handlers
            "onerror=",
            "onclick=",
            "onmouseover=",
            "onfocus=",
            "onblur=",
            "onchange=",
            "onsubmit=",
            "xlink:href=\"javascript:", // XLink JavaScript
            "href=\"javascript:",       // Direct JavaScript
            "vbscript:",         // VBScript
            "expression(",       // CSS expressions
            "eval(",             // JavaScript eval
            "setTimeout(",       // JavaScript timers
            "setInterval(",
            "document.",         // DOM access
            "window.",           // Window object access
            "location.",         // Location object access
        ];
        
        let lower_content = svg_content.to_lowercase();
        for pattern in &dangerous_patterns {
            if lower_content.contains(pattern) {
                return Err(FileSecurityError::DangerousSvgContent(pattern.to_string()));
            }
        }
        
        // Check for external references that could be used for data exfiltration
        let external_patterns = [
            "http://",
            "https://",
            "ftp://",
            "file://",
        ];
        
        for pattern in &external_patterns {
            if lower_content.contains(pattern) {
                return Err(FileSecurityError::ExternalSvgReference(pattern.to_string()));
            }
        }
        
        // Ensure the SVG is well-formed by checking for basic structure
        if !lower_content.contains("<svg") || !lower_content.contains("</svg>") {
            return Err(FileSecurityError::InvalidSvgStructure);
        }
        
        Ok(())
    }
    
    /// Scan for potentially malicious content in binary files
    fn scan_for_malicious_content(&self, data: &[u8]) -> Result<(), FileSecurityError> {
        // Check for embedded executables or scripts
        let malicious_signatures: &[&[u8]] = &[
            b"MZ",              // Windows PE executable  
            &[0x7f, 0x45, 0x4c, 0x46], // Linux ELF executable
            &[0xca, 0xfe, 0xba, 0xbe], // macOS Mach-O
            b"#!/bin/sh",       // Shell script
            b"#!/bin/bash",     // Bash script
            b"<?php",           // PHP code
            b"<%",              // ASP/JSP code
        ];
        
        for signature in malicious_signatures {
            if data.windows(signature.len()).any(|window| window == *signature) {
                return Err(FileSecurityError::MaliciousContent);
            }
        }
        
        Ok(())
    }
    
    /// Get safe filename by sanitizing user input
    pub fn sanitize_filename(&self, filename: &str) -> String {
        // Remove directory traversal attempts
        let filename = filename.replace("..", "").replace("/", "").replace("\\", "");
        
        // Remove or replace unsafe characters
        let safe_chars: String = filename
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
            .collect();
        
        // Ensure filename isn't empty and doesn't start with a dot
        if safe_chars.is_empty() || safe_chars.starts_with('.') {
            format!("file_{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
        } else {
            safe_chars
        }
    }
}

#[derive(Debug)]
pub enum FileSecurityError {
    FileTooLarge(usize, usize),
    UnsupportedFileType(String),
    InvalidMagicBytes(String),
    InvalidTextEncoding,
    SuspiciousContent(String),
    MaliciousContent,
    InvalidSvgFormat,
    DangerousSvgContent(String),
    ExternalSvgReference(String),
    InvalidSvgStructure,
}

impl std::fmt::Display for FileSecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSecurityError::FileTooLarge(actual, max) => {
                write!(f, "File too large: {} bytes (max: {} bytes)", actual, max)
            }
            FileSecurityError::UnsupportedFileType(mime) => {
                write!(f, "Unsupported file type: {}", mime)
            }
            FileSecurityError::InvalidMagicBytes(mime) => {
                write!(f, "File content doesn't match declared type: {}", mime)
            }
            FileSecurityError::InvalidTextEncoding => {
                write!(f, "Invalid text file encoding")
            }
            FileSecurityError::SuspiciousContent(pattern) => {
                write!(f, "Suspicious content detected: {}", pattern)
            }
            FileSecurityError::MaliciousContent => {
                write!(f, "Potentially malicious content detected")
            }
            FileSecurityError::InvalidSvgFormat => {
                write!(f, "Invalid SVG format: file must start with XML declaration or SVG tag")
            }
            FileSecurityError::DangerousSvgContent(pattern) => {
                write!(f, "Dangerous SVG content detected: {}", pattern)
            }
            FileSecurityError::ExternalSvgReference(pattern) => {
                write!(f, "External reference in SVG not allowed: {}", pattern)
            }
            FileSecurityError::InvalidSvgStructure => {
                write!(f, "Invalid SVG structure: missing required SVG tags")
            }
        }
    }
}

impl std::error::Error for FileSecurityError {}