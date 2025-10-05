# Contributing to Orchid

Thank you for your interest in contributing to Orchid! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Coding Standards](#coding-standards)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to create a welcoming environment for all contributors.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment (see below)
4. Create a new branch for your changes
5. Make your changes following our coding standards
6. Test your changes thoroughly
7. Submit a pull request

## Development Setup

### Prerequisites

- [.NET 9 SDK](https://dotnet.microsoft.com/download/dotnet/9.0)
- [Node.js](https://nodejs.org/) (LTS version recommended)
- [Docker](https://www.docker.com/get-started) and Docker Compose
- A code editor (VS Code, Visual Studio, or your preferred editor)

### Local Development Environment

Start the development environment using Docker Compose:

```bash
docker-compose up
```

This will start:
- Backend API server
- Frontend development server
- Any required services (database, cache, etc.)

### Running Tests

#### Backend Tests
```bash
cd src/Backend
dotnet test
```

#### Frontend Tests
```bash
cd src/Frontend
npm test
```

## Project Structure

```
orchid/
├── docs/              # Documentation and ADRs
│   └── adr/           # Architecture Decision Records
├── src/               # Source code
│   ├── Backend/       # .NET 9 backend
│   └── Frontend/      # Vue.js 3 frontend
├── tests/             # Test projects
│   ├── Backend.Tests/
│   └── Frontend.Tests/
├── .editorconfig      # Code style configuration
├── docker-compose.yml # Local development environment
└── README.md          # Project overview
```

## Making Changes

1. **Create a branch**: Use a descriptive name (e.g., `feature/add-chat-ui`, `fix/api-timeout`)
2. **Keep changes focused**: Each pull request should address a single concern
3. **Write tests**: Include tests for new features or bug fixes
4. **Update documentation**: Update relevant documentation, including ADRs for architectural changes
5. **Follow coding standards**: Adhere to the project's coding conventions (enforced by `.editorconfig`)

## Coding Standards

### Backend (C#)

- Follow the conventions in `.editorconfig`
- Use meaningful variable and method names
- Add XML documentation comments for public APIs
- Keep methods small and focused
- Use dependency injection for services
- Follow SOLID principles

### Frontend (Vue.js)

- Follow the conventions in `.editorconfig`
- Use TypeScript for type safety
- Follow Vue.js 3 Composition API patterns
- Keep components small and reusable
- Use meaningful component and prop names
- Add comments for complex logic

### Commit Messages

Write clear, descriptive commit messages:

```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat: add user authentication to chat interface

Implemented JWT-based authentication for the chat API
endpoint. Users can now log in and their sessions are
persisted across page reloads.

Closes #123
```

## Submitting Changes

1. **Push your changes** to your fork
2. **Create a pull request** from your branch to the main repository
3. **Provide a clear description** of what your changes do and why
4. **Reference related issues** using `Closes #123` or `Fixes #456`
5. **Wait for review** and address any feedback
6. **Ensure CI passes** before requesting final review

## Reporting Issues

When reporting issues, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Screenshots (if applicable)
- Environment details (OS, browser, etc.)
- Any relevant logs or error messages

## Questions?

If you have questions about contributing, feel free to:
- Open an issue with the `question` label
- Check existing documentation in the `/docs` folder
- Review Architecture Decision Records (ADRs) for context

Thank you for contributing to Orchid! 🌸
