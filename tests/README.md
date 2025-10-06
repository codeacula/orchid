# Orchid Tests

This directory contains all test projects for the Orchid application.

## 📁 Structure

### Backend.Tests (`/Backend.Tests`)

Contains tests for the .NET 9 backend API.

**Test Types:**
- Unit tests
- Integration tests
- API endpoint tests
- Service layer tests

**Test Framework:**
- xUnit
- Moq (for mocking)
- FluentAssertions (for readable assertions)

**Running Backend Tests:**
```bash
cd Backend.Tests
dotnet test
```

### Frontend.Tests (`/Frontend.Tests`)

Contains tests for the Vue.js 3 frontend application.

**Test Types:**
- Unit tests (components, composables, utilities)
- Integration tests
- E2E tests

**Test Framework:**
- Vitest (unit tests)
- Vue Test Utils
- Playwright or Cypress (E2E tests)

**Running Frontend Tests:**
```bash
cd Frontend.Tests
npm test
```

## 🧪 Testing Best Practices

### General Guidelines

1. **Write tests first** (TDD approach when possible)
2. **Keep tests focused** - one concept per test
3. **Use descriptive test names** - clearly state what is being tested
4. **Follow AAA pattern** - Arrange, Act, Assert
5. **Mock external dependencies** - tests should be isolated
6. **Test edge cases** - not just the happy path

### Backend Testing

- Test controllers for correct HTTP responses
- Test services for business logic
- Test repositories/data access separately
- Use in-memory database for integration tests
- Mock external API calls

### Frontend Testing

- Test components in isolation
- Test user interactions and events
- Test component rendering based on props/state
- Test composables and utilities as pure functions
- Use E2E tests for critical user flows

## 📊 Code Coverage

Code coverage reports can be generated:

### Backend
```bash
cd Backend.Tests
dotnet test /p:CollectCoverage=true /p:CoverletOutputFormat=opencover
```

### Frontend
```bash
cd Frontend.Tests
npm run test:coverage
```

## 🔍 Running Specific Tests

### Backend
```bash
# Run a specific test class
dotnet test --filter FullyQualifiedName~ClassName

# Run tests in a specific namespace
dotnet test --filter FullyQualifiedName~Namespace
```

### Frontend
```bash
# Run a specific test file
npm test -- ComponentName.test.ts

# Run tests in watch mode
npm test -- --watch
```

## 🚀 Continuous Integration

Tests are automatically run in CI/CD pipelines. Ensure all tests pass before submitting pull requests.

## 📚 Additional Resources

- [Backend Testing Documentation](Backend.Tests/README.md) (when available)
- [Frontend Testing Documentation](Frontend.Tests/README.md) (when available)
- [Contributing Guidelines](../CONTRIBUTING.md)
