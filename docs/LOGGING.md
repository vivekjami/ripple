# Logging Reference

Ripple uses the `tracing` ecosystem for structured logging.

## Log Levels

| Level | Use Case |
|-------|----------|
| `error` | Errors requiring immediate attention |
| `warn` | Potentially problematic situations |
| `info` | General operational messages |
| `debug` | Detailed debugging information |
| `trace` | Very verbose tracing output |

Set the level via `LOG_LEVEL` environment variable. Only messages at the configured level and above will be emitted.

## Output Formats

### Text (default)

```
2026-02-10T12:00:00.000Z  INFO ripple::main: Ripple v0.1.0 starting up
2026-02-10T12:00:00.001Z  INFO ripple::main: Configuration loaded successfully
2026-02-10T12:00:00.002Z  INFO ripple::main: Metrics system initialized
```

### JSON

Set `LOG_FORMAT=json`:

```json
{"timestamp":"2026-02-10T12:00:00.000Z","level":"INFO","target":"ripple::main","message":"Ripple v0.1.0 starting up"}
```

## File Output

Set `LOG_FILE_PATH=./logs/ripple.log` to write logs to a file in addition to the console.

The log directory is created automatically if it does not exist.

## Structured Fields

Each log entry includes:

| Field | Description |
|-------|-------------|
| `timestamp` | ISO 8601 timestamp |
| `level` | Log level |
| `target` | Source module (e.g., `ripple::config`) |
| `message` | Log message text |

## Configuration

```bash
LOG_LEVEL=info          # trace, debug, info, warn, error
LOG_FORMAT=text         # text, json
LOG_FILE_PATH=          # optional file path
```

## Best Practices

- Use `info` for production
- Use `debug` during development
- Use `trace` only when investigating specific issues
- Enable JSON format for log aggregation systems (ELK, Loki)
