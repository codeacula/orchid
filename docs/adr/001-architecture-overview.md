# 1. Architecture Overview

Date: 2025-01-05

## Status

Accepted

## Context

We need to establish a robust, scalable architecture for Orchid, a personal AI Chat/Agent UI application. The application should be:
- Easy to develop and maintain
- Fully containerized for consistent deployment
- Built with modern technologies
- Capable of running as a Progressive Web App (PWA)
- Suitable for personal use with potential for scaling

## Decision

We will adopt a containerized Single Page Application (SPA) architecture with:

### Backend
- **Framework**: .NET 9
- **Type**: RESTful API
- **Responsibilities**: 
  - AI/LLM service integration
  - Business logic
  - Data persistence
  - Authentication and authorization

### Frontend
- **Framework**: Vue.js 3 (with Composition API)
- **Type**: SPA/PWA
- **Responsibilities**:
  - User interface
  - Chat interactions
  - Client-side state management
  - Offline capabilities

### Infrastructure
- **Containerization**: Docker for both frontend and backend
- **Orchestration**: Docker Compose for local development
- **Deployment**: Container-ready for various platforms (cloud or self-hosted)

### Project Structure
```
orchid/
├── docs/              # Documentation and ADRs
├── src/               # Source code
│   ├── Backend/       # .NET 9 backend
│   └── Frontend/      # Vue.js 3 frontend
├── tests/             # Test projects
├── .editorconfig      # Code style enforcement
└── docker-compose.yml # Development environment
```

## Consequences

### Positive Consequences

- **Separation of Concerns**: Clear boundary between frontend and backend enables independent development and scaling
- **Modern Stack**: Both .NET 9 and Vue.js 3 are current, well-supported frameworks with active communities
- **Container-First**: Docker ensures consistent environments across development, testing, and production
- **Progressive Enhancement**: PWA capabilities allow offline usage and native-like experience
- **Developer Experience**: Hot-reload for frontend, fast compilation for backend, and unified development environment via Docker Compose
- **Testing**: Clear boundaries make unit and integration testing straightforward
- **Documentation**: ADR structure ensures architectural decisions are recorded and traceable

### Negative Consequences

- **Initial Setup Complexity**: Requires Docker knowledge and setup
- **Resource Usage**: Running multiple containers may be resource-intensive on some development machines
- **Learning Curve**: Developers need familiarity with both .NET and Vue.js ecosystems

## Alternatives Considered

### Monolithic MVC Application
- **Pros**: Simpler deployment, single framework
- **Cons**: Tighter coupling, harder to scale frontend independently, limited PWA capabilities

### Server-Side Rendered (SSR) with Next.js or Nuxt
- **Pros**: Better initial load performance, SEO benefits
- **Cons**: More complex architecture, SEO less important for personal chat UI, increased server load

### Full-Stack JavaScript (Node.js + Vue.js)
- **Pros**: Single language across stack
- **Cons**: .NET provides better performance and typing for API services, team preference for C# backend

### Microservices Architecture
- **Pros**: Ultimate scalability and service isolation
- **Cons**: Overkill for a personal chat application, increased operational complexity

## References

- [.NET 9 Documentation](https://docs.microsoft.com/en-us/dotnet/)
- [Vue.js 3 Documentation](https://vuejs.org/)
- [Docker Documentation](https://docs.docker.com/)
- [Progressive Web Apps (PWA)](https://web.dev/progressive-web-apps/)
