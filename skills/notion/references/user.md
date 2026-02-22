# User Commands

## `notion user me`

Get the current bot user.

**Endpoint:** `GET /v1/users/me`

```bash
notion user me
notion --raw user me | jq '.name'
```

## `notion user get <id>`

Get a user by ID.

**Endpoint:** `GET /v1/users/{id}`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | User ID     |

```bash
notion user get 12345678-abcd-1234-abcd-123456789abc
```

## `notion user list`

List all users.

**Endpoint:** `GET /v1/users`

Supports `--page-size` and `--start-cursor` for pagination.

```bash
notion user list
notion user list --page-size 50
notion --raw user list | jq '.results[] | {name, type}'
```
