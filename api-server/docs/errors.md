# Error Model

All errors return a structured JSON payload.

Example:

```json
{
  "code": "BAD_REQUEST",
  "message": "Invalid payload",
  "timestamp": "2024-01-01T12:34:56.789Z",
  "trace_id": "e7b7b4b6-4a9b-4e7e-9f0a-6f4a2c8d1234",
  "details": {
    "issues": [{ "field": "email", "message": "invalid email" }]
  }
}
```

Codes → HTTP status:

- BAD_REQUEST: 400
- VALIDATION_ERROR: 400
- RATE_LIMIT_CONFLICT: 400
- UNAUTHORIZED: 401
- FORBIDDEN: 403
- NOT_FOUND: 404
- CONFLICT: 409
- SERVICE_UNAVAILABLE: 503
- TIMEOUT: 504
- INTERNAL_ERROR: 500
