# Security Policy

## Supported Versions

We actively support the following versions of Everything Plus with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in Everything Plus, please report it responsibly.

### How to Report

1. **Do NOT create a public GitHub issue** for security vulnerabilities
2. **Email security concerns** to: [security@yourcompany.com](mailto:security@yourcompany.com)
3. **Include the following information:**
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact assessment
   - Any proof-of-concept code (if applicable)
   - Your contact information

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Regular Updates**: We will keep you informed of our progress throughout the investigation
- **Resolution**: We aim to resolve security issues within 90 days of disclosure

### Security Measures

Everything Plus implements several security measures:

#### Data Protection
- **Local Storage Only**: All file indexes and metadata are stored locally
- **No Cloud Transmission**: No data is sent to external servers
- **Encrypted Storage**: Database files use SQLite's built-in security features
- **File Permissions**: Respects Windows file system permissions

#### Application Security
- **Sandboxed Execution**: Runs with minimal required privileges
- **Input Validation**: All search queries are sanitized and validated
- **Memory Safety**: Rust's memory safety guarantees prevent common vulnerabilities
- **Dependency Scanning**: Regular audits of third-party dependencies

#### System Integration
- **No Elevated Privileges**: Runs under user account permissions
- **Safe File Operations**: Read-only access to indexed files
- **Registry Protection**: Minimal Windows registry modifications
- **Network Isolation**: No network access required for core functionality

### Known Security Considerations

#### File System Access
- Everything Plus requires read access to indexed directories
- Users should carefully consider which directories to index
- Sensitive directories can be excluded via settings

#### Search Results
- File names and paths are displayed in search results
- Users in shared environments should be aware of potential information disclosure
- Hidden files are excluded by default but can be enabled

#### Database Security
- The local SQLite database contains file metadata
- Database files should be protected with appropriate file system permissions
- Regular backups should be encrypted if stored externally

### Best Practices for Users

1. **Limit Indexed Directories**: Only index directories you need to search
2. **Exclude Sensitive Paths**: Add sensitive directories to the exclude list
3. **Regular Updates**: Keep Everything Plus updated to the latest version
4. **Secure Backups**: Encrypt database backups if stored externally
5. **Network Drives**: Be cautious when indexing network-mounted drives

### Vulnerability Disclosure Timeline

1. **Day 0**: Vulnerability reported to security team
2. **Day 1-2**: Acknowledgment and initial triage
3. **Day 3-7**: Detailed investigation and impact assessment
4. **Day 8-30**: Development of fix and testing
5. **Day 31-60**: Release preparation and validation
6. **Day 61-90**: Public disclosure and patch release

### Security Updates

Security updates will be:
- Released as soon as possible after verification
- Clearly marked as security releases
- Accompanied by detailed changelogs (after responsible disclosure period)
- Available through the standard update mechanism

### Third-Party Dependencies

We regularly audit our dependencies for security vulnerabilities:

#### Frontend Dependencies
- React ecosystem packages
- Tailwind CSS and UI components
- Development and build tools

#### Backend Dependencies
- Rust crates from crates.io
- System integration libraries
- Database drivers

#### Dependency Management
- Automated vulnerability scanning with `cargo audit` and `npm audit`
- Regular updates to latest stable versions
- Security patches applied promptly

### Contact Information

For security-related inquiries:
- **Email**: security@yourcompany.com
- **PGP Key**: [Available on request]
- **Response Time**: Within 48 hours

For general issues:
- **GitHub Issues**: https://github.com/your-username/everything-plus/issues
- **Documentation**: See README.md and docs/

### Legal

We follow responsible disclosure practices and ask that security researchers do the same. We will not pursue legal action against researchers who:

1. Report vulnerabilities in good faith
2. Do not access or modify user data beyond what is necessary to demonstrate the vulnerability
3. Do not violate any laws or regulations
4. Do not disclose the vulnerability publicly until we have had reasonable time to address it

### Attribution

We believe in recognizing security researchers who help make Everything Plus more secure. With your permission, we will:

- Credit you in our security advisory
- Include your name in our release notes
- Provide a reference letter if requested

Thank you for helping keep Everything Plus and its users secure.
