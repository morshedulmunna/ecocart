# Categories API

## List

- GET `/api/categories`

Response 200:

```json
[{ "id": 1, "name": "Bags", "description": "eco friendly" }]
```

## Create (admin)

- POST `/api/categories`
- Requires `Authorization: Bearer <token>` with role `admin`.

Request:

```json
{ "name": "Bags", "description": "eco friendly" }
```

## Get by ID

- GET `/api/categories/:id`

## Update (admin)

- PUT `/api/categories/:id`

Request (partial):

```json
{ "name": "New name", "description": "Optional" }
```

## Delete (admin)

- DELETE `/api/categories/:id`

Response 200:

```json
{ "deleted": true }
```
