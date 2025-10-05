# Use the official .NET 9 SDK image for building
FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
WORKDIR /src

# Copy project files
COPY *.csproj ./
RUN dotnet restore

# Copy everything else and build
COPY . ./
RUN dotnet build -c Release -o /app/build

# Publish the application
FROM build AS publish
RUN dotnet publish -c Release -o /app/publish

# Use the runtime image for the final stage
FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS final
WORKDIR /app
EXPOSE 8080
COPY --from=publish /app/publish .

ENTRYPOINT ["dotnet", "Orchid.Backend.dll"]
