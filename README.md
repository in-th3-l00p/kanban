# Kanban

Rust kanban implementation using file storage, for the purpose of storing my tasks for in a local manner, straight in the GitHub repository.

## implementation details

This project will serve as a library for interacting with this json file, but will also provide a binary with a CLI frontend.

The way it works is using *projects*, similar to npm or cargo projects. Each project will have multiple *kanban boards*, each with a different set of tasks.

### JSON schema

```json
{
  "name": "kanban board name",
  "description": "description of this kanban board",
  "author": "you might wanna know who made this",
  "boards": [
    {
      "name": "name of the board",
      "description": "whatever",
      "created_at": "iso8601 datetime utc representation",
      "updated_at": "iso8601 datetime utc representation",
      "columns": {
        "todo": [
          {
            "id": "slug of the task's name",
            "name": "name of the task",
            "description": "do the dishes",
            "created_at": "iso8601 datetime utc representation",
            "updated_at": "iso8601 datetime utc representation"
          }
        ],
        "in-progress":  [],
        "testing": [],
        "review": [],
        "complete": []
      }
    }
  ]
}
```
