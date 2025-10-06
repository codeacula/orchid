# Orchid Source Code

This directory contains all source code for the Orchid application.

## 📁 Structure

### Backend (`/Backend`)

The backend is built with .NET 9 and provides RESTful API endpoints for the frontend.

**Key Technologies:**
- .NET 9
- ASP.NET Core Web API
- Entity Framework Core (for data access)
- JWT Authentication

**Responsibilities:**
- AI/LLM service integration
- Business logic and data processing
- Data persistence
- Authentication and authorization
- API endpoints for frontend

### Frontend (`/Frontend`)

The frontend is built with Vue.js 3 and provides a modern, responsive user interface as a Progressive Web App (PWA).

**Key Technologies:**
- Vue.js 3 (Composition API)
- TypeScript
- Vite (build tool)
- Pinia (state management)
- Vue Router
- PWA capabilities

**Responsibilities:**
- User interface components
- Chat interface
- State management
- API communication
- Offline support
- Client-side routing

## 🚀 Development

### Backend Development

```bash
cd Backend
dotnet restore
dotnet build
dotnet run
```

The API will be available at `http://localhost:5000` by default.

### Frontend Development

```bash
cd Frontend
npm install
npm run dev
```

The development server will be available at `http://localhost:8080` by default.

### Using Docker Compose

For full-stack development with all services:

```bash
# From the repository root
docker-compose up
```

This starts both backend and frontend services with hot-reload enabled.

## 🧪 Testing

Tests for both backend and frontend are located in the `/tests` directory at the repository root.

## 📦 Building for Production

### Backend

```bash
cd Backend
dotnet publish -c Release -o ./publish
```

### Frontend

```bash
cd Frontend
npm run build
```

The production build will be in the `dist` directory.

### Docker Images

Build production Docker images:

```bash
# Backend
cd Backend
docker build -t orchid-backend .

# Frontend
cd Frontend
docker build -t orchid-frontend .
```

## 📚 Additional Resources

- [Project Documentation](../docs/README.md)
- [Architecture Decision Records](../docs/adr/)
- [Contributing Guidelines](../CONTRIBUTING.md)
