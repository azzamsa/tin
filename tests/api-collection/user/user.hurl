POST {{base_url}}
Content-Type: application/json

```graphql
{
  user(id: "017eb8d1-a5b5-9443-2d94-b6ad7787bf0e") {
    id
    name
    email
    fullName
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200


