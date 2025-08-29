# Plugin Merkle Tree Verification System

## Overview

The Rust CMS plugin system uses Merkle trees for cryptographic verification of plugin integrity, ensuring that plugins haven't been tampered with and providing a secure chain of trust.

## What is a Merkle Tree?

A Merkle tree (also known as a hash tree) is a binary tree where:
- Each leaf node contains the hash of a data block
- Each non-leaf node contains the hash of its child nodes
- The root hash represents the entire dataset

```
        Root Hash
       /         \
   Hash(A+B)   Hash(C+D)
    /    \      /    \
Hash(A) Hash(B) Hash(C) Hash(D)
   |      |      |      |
File A  File B  File C  File D
```

## Plugin Verification Process

### 1. Plugin Packaging
When a plugin is packaged, we create a Merkle tree of all files:

```rust
// Example Merkle tree creation
pub struct PluginMerkleTree {
    root_hash: String,
    file_hashes: HashMap<String, String>,
    tree_structure: Vec<Vec<String>>,
}

impl PluginMerkleTree {
    pub fn from_plugin_files(files: &[PluginFile]) -> Self {
        let mut file_hashes = HashMap::new();
        let mut leaf_hashes = Vec::new();
        
        // Hash each file
        for file in files {
            let hash = sha256(&file.content);
            file_hashes.insert(file.path.clone(), hash.clone());
            leaf_hashes.push(hash);
        }
        
        // Build Merkle tree
        let tree_structure = build_merkle_tree(leaf_hashes);
        let root_hash = tree_structure.last().unwrap()[0].clone();
        
        Self {
            root_hash,
            file_hashes,
            tree_structure,
        }
    }
}
```

### 2. Plugin Manifest Integration
The Merkle root hash is included in the plugin manifest:

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "merkle_root": "a1b2c3d4e5f6...",
  "file_manifest": {
    "src/lib.rs": "hash1...",
    "plugin.json": "hash2...",
    "README.md": "hash3..."
  },
  "signature": "plugin_signature...",
  "certificate": "signing_certificate..."
}
```

### 3. Verification on Installation
When installing a plugin, the system verifies:

```rust
pub async fn verify_plugin_integrity(plugin_zip: &[u8]) -> Result<bool, VerificationError> {
    // 1. Extract plugin files
    let files = extract_plugin_files(plugin_zip)?;
    
    // 2. Load manifest
    let manifest = load_plugin_manifest(&files)?;
    
    // 3. Verify file hashes
    for (file_path, expected_hash) in &manifest.file_manifest {
        let file_content = files.get(file_path)
            .ok_or(VerificationError::MissingFile(file_path.clone()))?;
        
        let actual_hash = sha256(file_content);
        if actual_hash != *expected_hash {
            return Err(VerificationError::HashMismatch(file_path.clone()));
        }
    }
    
    // 4. Verify Merkle root
    let calculated_tree = PluginMerkleTree::from_plugin_files(&files);
    if calculated_tree.root_hash != manifest.merkle_root {
        return Err(VerificationError::MerkleRootMismatch);
    }
    
    // 5. Verify digital signature
    verify_plugin_signature(&manifest)?;
    
    Ok(true)
}
```

## Security Benefits

### 1. Integrity Verification
- Detects any file modifications
- Ensures plugin hasn't been corrupted
- Validates complete plugin package

### 2. Efficient Verification
- Only need to verify root hash for quick checks
- Can verify individual files without checking entire package
- Supports partial verification for large plugins

### 3. Tamper Detection
- Any change to any file changes the root hash
- Impossible to modify files without detection
- Cryptographically secure verification

### 4. Chain of Trust
- Plugins signed by trusted developers
- Certificate-based verification
- Revocation support for compromised keys

## Implementation Details

### Hash Algorithm
We use SHA-256 for all hashing operations:

```rust
use sha2::{Sha256, Digest};

fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
```

### Merkle Tree Construction
```rust
fn build_merkle_tree(leaf_hashes: Vec<String>) -> Vec<Vec<String>> {
    let mut levels = vec![leaf_hashes];
    
    while levels.last().unwrap().len() > 1 {
        let current_level = levels.last().unwrap();
        let mut next_level = Vec::new();
        
        for chunk in current_level.chunks(2) {
            let combined = if chunk.len() == 2 {
                format!("{}{}", chunk[0], chunk[1])
            } else {
                // Odd number of nodes - duplicate the last one
                format!("{}{}", chunk[0], chunk[0])
            };
            
            next_level.push(sha256(combined.as_bytes()));
        }
        
        levels.push(next_level);
    }
    
    levels
}
```

### Merkle Proof Generation
For efficient verification of individual files:

```rust
pub struct MerkleProof {
    pub file_path: String,
    pub file_hash: String,
    pub proof_hashes: Vec<String>,
    pub proof_directions: Vec<bool>, // true = right, false = left
}

impl PluginMerkleTree {
    pub fn generate_proof(&self, file_path: &str) -> Option<MerkleProof> {
        let file_hash = self.file_hashes.get(file_path)?.clone();
        
        // Find leaf index
        let leaf_index = self.find_leaf_index(file_path)?;
        
        let mut proof_hashes = Vec::new();
        let mut proof_directions = Vec::new();
        let mut current_index = leaf_index;
        
        // Traverse up the tree
        for level in 0..self.tree_structure.len() - 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < self.tree_structure[level].len() {
                proof_hashes.push(self.tree_structure[level][sibling_index].clone());
                proof_directions.push(current_index % 2 == 0);
            }
            
            current_index /= 2;
        }
        
        Some(MerkleProof {
            file_path: file_path.to_string(),
            file_hash,
            proof_hashes,
            proof_directions,
        })
    }
}
```

### Proof Verification
```rust
pub fn verify_merkle_proof(
    proof: &MerkleProof,
    root_hash: &str,
) -> bool {
    let mut current_hash = proof.file_hash.clone();
    
    for (sibling_hash, is_right) in proof.proof_hashes.iter()
        .zip(proof.proof_directions.iter()) {
        
        let combined = if *is_right {
            format!("{}{}", current_hash, sibling_hash)
        } else {
            format!("{}{}", sibling_hash, current_hash)
        };
        
        current_hash = sha256(combined.as_bytes());
    }
    
    current_hash == root_hash
}
```

## Plugin Signing Process

### 1. Developer Certificate
Developers must obtain a signing certificate:

```bash
# Generate private key
openssl genrsa -out plugin-signing.key 4096

# Generate certificate signing request
openssl req -new -key plugin-signing.key -out plugin-signing.csr

# Submit CSR to certificate authority
# Receive signed certificate: plugin-signing.crt
```

### 2. Plugin Signing
```rust
use rsa::{RsaPrivateKey, PaddingScheme};
use rsa::pkcs1::DecodeRsaPrivateKey;

pub fn sign_plugin_manifest(
    manifest: &PluginManifest,
    private_key_pem: &str,
) -> Result<String, SigningError> {
    let private_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem)?;
    
    // Create signature payload
    let payload = serde_json::to_string(manifest)?;
    let payload_hash = sha256(payload.as_bytes());
    
    // Sign the hash
    let signature = private_key.sign(
        PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256)),
        payload_hash.as_bytes(),
    )?;
    
    Ok(base64::encode(signature))
}
```

### 3. Signature Verification
```rust
use rsa::{RsaPublicKey, PaddingScheme};

pub fn verify_plugin_signature(
    manifest: &PluginManifest,
    public_key_pem: &str,
) -> Result<bool, VerificationError> {
    let public_key = RsaPublicKey::from_pkcs1_pem(public_key_pem)?;
    
    // Recreate signature payload
    let mut unsigned_manifest = manifest.clone();
    unsigned_manifest.signature = None;
    let payload = serde_json::to_string(&unsigned_manifest)?;
    let payload_hash = sha256(payload.as_bytes());
    
    // Verify signature
    let signature = base64::decode(&manifest.signature.as_ref().unwrap())?;
    
    public_key.verify(
        PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256)),
        payload_hash.as_bytes(),
        &signature,
    ).map_err(|_| VerificationError::InvalidSignature)?;
    
    Ok(true)
}
```

## Certificate Management

### Certificate Store
The CMS maintains a certificate store for trusted developers:

```rust
pub struct CertificateStore {
    trusted_certificates: HashMap<String, Certificate>,
    revoked_certificates: HashSet<String>,
}

impl CertificateStore {
    pub fn is_certificate_trusted(&self, cert_fingerprint: &str) -> bool {
        self.trusted_certificates.contains_key(cert_fingerprint) &&
        !self.revoked_certificates.contains(cert_fingerprint)
    }
    
    pub fn add_trusted_certificate(&mut self, cert: Certificate) {
        let fingerprint = cert.fingerprint();
        self.trusted_certificates.insert(fingerprint, cert);
    }
    
    pub fn revoke_certificate(&mut self, cert_fingerprint: &str) {
        self.revoked_certificates.insert(cert_fingerprint.to_string());
    }
}
```

### Certificate Revocation
Support for certificate revocation lists (CRL):

```rust
pub async fn check_certificate_revocation(
    cert_fingerprint: &str,
) -> Result<bool, RevocationError> {
    // Check local revocation list
    if LOCAL_REVOCATION_LIST.contains(cert_fingerprint) {
        return Ok(true);
    }
    
    // Check online CRL
    let crl_url = format!("https://crl.rustcms.org/check/{}", cert_fingerprint);
    let response = reqwest::get(&crl_url).await?;
    
    if response.status().is_success() {
        let revocation_status: RevocationStatus = response.json().await?;
        Ok(revocation_status.is_revoked)
    } else {
        Err(RevocationError::CrlUnavailable)
    }
}
```

## Performance Considerations

### Caching
Cache verification results to avoid repeated calculations:

```rust
pub struct VerificationCache {
    verified_plugins: HashMap<String, VerificationResult>,
    cache_ttl: Duration,
}

impl VerificationCache {
    pub fn get_cached_result(&self, plugin_hash: &str) -> Option<&VerificationResult> {
        self.verified_plugins.get(plugin_hash)
            .filter(|result| result.verified_at.elapsed() < self.cache_ttl)
    }
    
    pub fn cache_result(&mut self, plugin_hash: String, result: VerificationResult) {
        self.verified_plugins.insert(plugin_hash, result);
    }
}
```

### Parallel Verification
Verify multiple files in parallel:

```rust
use tokio::task::JoinSet;

pub async fn verify_plugin_files_parallel(
    files: &HashMap<String, Vec<u8>>,
    expected_hashes: &HashMap<String, String>,
) -> Result<bool, VerificationError> {
    let mut join_set = JoinSet::new();
    
    for (file_path, file_content) in files {
        let expected_hash = expected_hashes.get(file_path)
            .ok_or_else(|| VerificationError::MissingHash(file_path.clone()))?
            .clone();
        let content = file_content.clone();
        let path = file_path.clone();
        
        join_set.spawn(async move {
            let actual_hash = sha256(&content);
            (path, actual_hash == expected_hash)
        });
    }
    
    while let Some(result) = join_set.join_next().await {
        let (file_path, is_valid) = result?;
        if !is_valid {
            return Err(VerificationError::HashMismatch(file_path));
        }
    }
    
    Ok(true)
}
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("Missing file: {0}")]
    MissingFile(String),
    
    #[error("Hash mismatch for file: {0}")]
    HashMismatch(String),
    
    #[error("Merkle root mismatch")]
    MerkleRootMismatch,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Certificate not trusted")]
    UntrustedCertificate,
    
    #[error("Certificate revoked")]
    RevokedCertificate,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
```

## CLI Tools

### Plugin Verification Tool
```bash
# Verify a plugin package
rustcms-plugin verify my-plugin.zip

# Generate Merkle tree for plugin
rustcms-plugin merkle-tree ./my-plugin-source/

# Sign a plugin
rustcms-plugin sign ./my-plugin-source/ --key signing.key --cert signing.crt

# Check certificate status
rustcms-plugin check-cert --fingerprint abc123...
```

### Example Output
```
$ rustcms-plugin verify my-plugin.zip

✓ Plugin package extracted successfully
✓ Manifest loaded and validated
✓ File integrity verified (5/5 files)
✓ Merkle root verified: a1b2c3d4e5f6...
✓ Digital signature verified
✓ Certificate trusted and not revoked

Plugin verification: PASSED
Plugin: My Awesome Plugin v1.0.0
Author: John Doe <john@example.com>
Signed: 2024-01-15 10:30:00 UTC
```

## Best Practices

### For Plugin Developers
1. **Sign all releases** with a trusted certificate
2. **Use semantic versioning** for clear version tracking
3. **Include comprehensive file manifests**
4. **Test verification** before publishing
5. **Keep signing keys secure** and rotate regularly

### For CMS Administrators
1. **Only install verified plugins** from trusted sources
2. **Regularly update certificate stores**
3. **Monitor revocation lists**
4. **Enable verification logging**
5. **Set up automated verification checks**

### For Security Teams
1. **Audit certificate authorities** regularly
2. **Monitor for suspicious plugins**
3. **Implement defense in depth**
4. **Regular security assessments**
5. **Incident response procedures**

---

The Merkle tree verification system provides a robust foundation for plugin security, ensuring that users can trust the plugins they install while maintaining performance and usability.
