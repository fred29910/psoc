# Contributing to PSOC

Thank you for your interest in contributing to PSOC! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Screenshots if applicable

### Suggesting Features

Feature suggestions are welcome! Please:

- Check existing feature requests first
- Provide a clear description of the feature
- Explain why this feature would be useful
- Consider the scope and complexity

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/psoc.git`
3. Install Rust: https://rustup.rs/
4. Install system dependencies (see README.md)
5. Run tests: `cargo test`
6. Run the application: `cargo run`

### Making Changes

1. Create a new branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Format your code: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Commit your changes with a clear message
8. Push to your fork: `git push origin feature/your-feature-name`
9. Create a Pull Request

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names
- Follow the existing code patterns

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass before submitting
- Aim for good test coverage

### Documentation

- Update documentation for new features
- Add doc comments for public APIs
- Update README.md if needed
- Keep CHANGELOG.md updated

## Development Workflow

### Branch Strategy

- `main`: Stable release branch
- `develop`: Integration branch for new features
- `feature/*`: Feature development branches
- `hotfix/*`: Emergency fixes
- `release/*`: Release preparation branches

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add brush tool with size adjustment
fix: resolve memory leak in layer rendering
docs: update installation instructions
test: add unit tests for color conversion
```

### Pull Request Process

1. Ensure your PR has a clear title and description
2. Reference any related issues
3. Include screenshots for UI changes
4. Ensure CI checks pass
5. Request review from maintainers
6. Address feedback promptly

## Project Structure

See `docs/project.md` for detailed information about the project structure and architecture.

## Getting Help

- Check the documentation in the `docs/` directory
- Look at existing code for examples
- Ask questions in GitHub Discussions
- Join our community chat (if available)

## Recognition

Contributors will be recognized in:
- CHANGELOG.md for significant contributions
- README.md contributors section
- Release notes

Thank you for contributing to PSOC!
