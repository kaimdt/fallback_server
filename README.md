# Fallback Server

Ein minimaler, asynchroner Rust-Server für die Leryon-Infrastruktur.

## Features

- **Minimale Abhängigkeiten**: Nur Tokio, Axum, Serde
- **Hohe Performance**: >10.000 Anfragen/Sekunde
- **Schneller Start**: Startet in unter 1 Sekunde
- **Konsistente Fehlerantwort**: Liefert immer dieselbe JSON-Fehlermeldung
- **Traefik-kompatibel**: Perfekt als globaler Fallback-Service
- **Docker-ready**: Optimiertes Multi-Stage Dockerfile

## API Response

Der Server antwortet auf alle Anfragen mit:

```json
{
  "error": "Service Unavailable",
  "message": "The requested service is temporarily unavailable. Please try again later.",
  "status": 503
}
```

HTTP Status Code: `503 Service Unavailable`

## Verwendung

### Lokal mit Cargo

```bash
# Build
cargo build --release

# Run
./target/release/fallback_server

# Mit Custom Port
PORT=3000 ./target/release/fallback_server
```

### Docker

```bash
# Build
docker build -t fallback_server .

# Run
docker run -p 8080:8080 fallback_server

# Mit Custom Port
docker run -p 3000:3000 -e PORT=3000 fallback_server
```

### Docker Compose

```bash
docker-compose up -d
```

## Traefik Integration

Beispiel-Konfiguration als globaler Fallback:

```yaml
http:
  routers:
    fallback:
      rule: "PathPrefix(`/`)"
      priority: 1
      service: fallback-service
  
  services:
    fallback-service:
      loadBalancer:
        servers:
          - url: "http://fallback_server:8080"
```

## Konfiguration

- `PORT`: Server-Port (Standard: 8080)

## Performance

Der Server wurde für maximale Performance optimiert:

- Release-Build mit LTO und Optimierungsstufe 3
- Minimale Speichernutzung
- Asynchrone Request-Verarbeitung mit Tokio
- Keine unnötigen Abhängigkeiten

## Lizenz

MIT
