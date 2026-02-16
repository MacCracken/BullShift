# Contributing to BullShift

Thank you for your interest in contributing to BullShift! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

1. **Review Security Requirements**
   - Read the [Security Audit Report](security-audit.md)
   - Understand the security architecture and requirements
   - All contributions must follow security best practices

2. **Read Documentation**
   - [Development Setup Guide](development_setup.md) - Environment setup
   - [Architecture Guide](architecture.md) - Project structure
   - [Roadmap](roadmap.md) - Feature planning

3. **Set Up Development Environment**
   ```bash
   # Clone and setup
   git clone <repository-url>
   cd bullshift
   ./build.sh
   
   # Install dependencies
   cd rust && cargo build --release
   cd ../flutter && flutter pub get
   ```

## 📋 Contribution Types

### 🐛 Bug Reports
- Use the issue template for bug reports
- Include platform, Flutter version, and Rust version
- Provide reproduction steps and expected behavior
- Include error logs and screenshots if applicable

### ✨ Feature Requests
- Use the feature request template
- Describe the use case and problem you're solving
- Suggest implementation approach if possible
- Consider impact on security and performance

### 🔧 Code Contributions

#### Before You Start
1. Check existing issues and pull requests
2. Discuss major changes in an issue first
3. Ensure your contribution aligns with project goals
4. Verify the issue isn't already being worked on

#### Development Process
1. **Fork the Repository**
   ```bash
   git clone https://github.com/yourusername/bullshift.git
   cd bullshift
   ```

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

3. **Make Your Changes**
   - Follow existing code style and patterns
   - Add comments for complex logic
   - Update documentation as needed
   - Write tests for new functionality

4. **Run Tests and Quality Checks**
   ```bash
   # Rust tests
   cd rust && cargo test && cargo clippy
   
   # Flutter tests
   cd flutter && flutter test && flutter analyze
   
   # Code formatting
   cargo fmt
   flutter format .
   ```

5. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add new trading indicator"
   # Use conventional commits
   ```

## 📝 Coding Standards

### Rust Code
- Follow `rustfmt` formatting
- Use `cargo clippy` for linting
- Include doc comments for public APIs
- Handle errors properly with `Result` types
- Prefer `thiserror` for custom errors

### Flutter/Dart Code
- Follow `flutter format` formatting
- Use `flutter analyze` for linting
- Use proper state management (Provider pattern)
- Include dartdoc comments for public APIs
- Handle async operations safely

### Security Requirements
- Never commit secrets or API keys
- Use secure credential storage (AES-256)
- Follow FFI safety guidelines
- Validate all user inputs
- Use `Random.secure()` for cryptographic operations

### Testing
- Write unit tests for business logic
- Add widget tests for UI components
- Include integration tests for critical flows
- Aim for 80%+ test coverage
- Test error scenarios

## 🔄 Pull Request Process

### Before Submitting
1. **Rebase** your branch on latest main
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Run Full Test Suite**
   ```bash
   cargo test
   flutter test
   ```

3. **Update Documentation**
   - Update relevant documentation files
   - Add examples for new features
   - Update CHANGELOG.md

4. **Self-Review**
   - Check for security issues
   - Verify code follows project standards
   - Ensure tests pass locally
   - Confirm documentation is updated

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Widget tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Security
- [ ] No secrets committed
- [ ] Security review completed
- [ ] FFI safety checked
- [ ] Input validation added

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] CHANGELOG.md updated
```

### Review Process
1. **Automated Checks**
   - CI/CD pipeline runs tests
   - Code quality checks
   - Security scanning

2. **Manual Review**
   - Code review by maintainers
   - Security review for sensitive changes
   - Architecture review for major changes

3. **Approval Requirements**
   - At least one maintainer approval
   - Security review for authentication/trading changes
   - All automated checks must pass

## 🏗️ Development Guidelines

### Feature Development
1. **Start with Issue Discussion** - Create issue for new features
2. **Design Before Code** - Plan architecture and security implications
3. **Iterative Development** - Make small, focused commits
4. **Test-Driven** - Write tests before implementation when possible
5. **Documentation-First** - Update docs as you code

### Security-First Development
- All trading-related features require security review
- Never handle credentials in plain text
- Use secure communication channels
- Validate all external inputs
- Follow principle of least privilege

### Performance Considerations
- Profile before optimizing
- Avoid unnecessary cloning in Rust
- Use Flutter performance tools
- Consider mobile constraints
- Monitor memory usage

## 🐛 Common Issues

### Build Failures
- Ensure Rust and Flutter versions match requirements
- Clean build cache: `cargo clean && flutter clean`
- Check platform dependencies

### Test Failures
- Update test fixtures if data format changes
- Mock external dependencies
- Check platform-specific behavior

### Security Concerns
- Never commit `.env` files
- Use secure credential storage
- Validate FFI boundaries
- Review data handling

## 📞 Getting Help

### Resources
- [Documentation](docs/)
- [Issue Templates](.github/ISSUE_TEMPLATE/)
- [Discussions](https://github.com/yourrepo/bullshift/discussions)

### Contact
- Create an issue for questions
- Tag maintainers for urgent matters
- Use discussions for general questions

## 📄 License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

## 🙏 Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes for significant contributions
- Annual contributor highlights

Thank you for contributing to BullShift! 🚀