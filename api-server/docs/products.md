# Products API

## List (paginated)

- GET `/api/products`
- Query:
  - `q` (string, optional) — search by name/description (ILIKE)
  - `category_id` (number, optional)
  - `min_price` (number, optional)
  - `max_price` (number, optional)
  - `page` (number, optional, default: 1)
  - `page_size` (number, optional, default: 12, max: 100). Alias: `pageSize`

Response 200:

```json
{
  "items": [
    {
      "id": 1,
      "name": "Eco Bottle",
      "description": "Reusable",
      "price": 19.99,
      "category_id": 2,
      "image_url": "/uploads/abc.png",
      "stock": 10,
      "created_at": "2024-01-01T12:00:00Z"
    }
  ],
  "page": 1,
  "page_size": 12,
  "total": 57,
  "total_pages": 5
}
```

## Create

- POST `/api/products`

Request:

```json
{
  "name": "Eco Bottle",
  "description": "Reusable",
  "price": 19.99,
  "category_id": 2,
  "stock": 10,
  "image_url": null
}
```

## Get by ID

- GET `/api/products/:id`

## Update

- PUT `/api/products/:id`

Request (partial):

```json
{
  "name": "New name",
  "price": 24.5
}
```

## Delete

- DELETE `/api/products/:id`

Response 200:

```json
{ "deleted": true }
```

## Upload Image

- POST `/api/products/upload`
- Content-Type: `multipart/form-data`

Response 200:

```json
{ "url": "/uploads/<file>" }
```
