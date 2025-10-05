# Orchid 🌸

A personal AI Chat/Agent UI, built as a SPA/PWA with a .NET 9 backend and a Vue.js 3 frontend, fully containerized with Docker.

## 🚀 Overview

Orchid is a modern, containerized application that provides an intuitive interface for interacting with AI/LLM services. The project combines the power of .NET 9 for backend services with the flexibility of Vue.js 3 for a responsive, progressive web application frontend.

## ✨ Features

- **Personal AI Chat**: Intuitive chat interface for AI interactions
- **Progressive Web App**: Works offline and can be installed on devices
- **Fully Containerized**: Easy deployment with Docker and Docker Compose
- **Modern Tech Stack**: .NET 9 backend with Vue.js 3 frontend
- **Scalable Architecture**: Clean separation of concerns with documented decisions

## 🗂️ Core File Structure

```
orchid/
├── docs/              # Project documentation, including ADRs
│   └── adr/           # Architecture Decision Records
├── src/               # All C# and Vue.js source code
│   ├── Backend/       # .NET 9 backend API
│   └── Frontend/      # Vue.js 3 frontend SPA/PWA
├── tests/             # All test projects
│   ├── Backend.Tests/
│   └── Frontend.Tests/
├── .editorconfig      # Enforces consistent code style
├── docker-compose.yml # Defines the local development environment
├── README.md          # This file
└── CONTRIBUTING.md    # Contribution guidelines
```

## 🛠️ Getting Started

### Prerequisites

- [.NET 9 SDK](https://dotnet.microsoft.com/download/dotnet/9.0)
- [Node.js](https://nodejs.org/) (LTS version recommended)
- [Docker](https://www.docker.com/get-started) and Docker Compose

### Quick Start with Docker Compose

The easiest way to run Orchid locally is using Docker Compose:

```bash
# Clone the repository
git clone https://github.com/codeacula/orchid.git
cd orchid

# Start the application
docker-compose up
```

The application will be available at:
- Frontend: http://localhost:8080
- Backend API: http://localhost:5000

### Development Setup

#### Backend

```bash
cd src/Backend
dotnet restore
dotnet build
dotnet run
```

#### Frontend

```bash
cd src/Frontend
npm install
npm run dev
```

### Running Tests

#### Backend Tests
```bash
cd tests/Backend.Tests
dotnet test
```

#### Frontend Tests
```bash
cd tests/Frontend.Tests
npm test
```

## 📚 Documentation

- [Contributing Guidelines](CONTRIBUTING.md)
- [Architecture Decision Records](docs/adr/)
- [License](LICENSE)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on how to get started.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with ❤️ by [Codeacula](https://github.com/codeacula)
