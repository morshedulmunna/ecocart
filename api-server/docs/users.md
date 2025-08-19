# Users API

## Get User

- GET `/api/users/:id` (UUID)

Response 200:

```json
{
  "id": "<uuid>",
  "username": "alice",
  "email": "alice@example.com",
  "role": "user",
  "created_at": "2024-01-01T12:34:56.789Z"
}
```

Errors: see [Error Model](crate::api_docs::errors).
