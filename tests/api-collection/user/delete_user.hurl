POST {{base_url}}
Content-Type: application/json

```graphql
mutation {
  deleteUser(id: "017eb8d1-a5b5-9443-2d94-b6ad7787bf0e") {
    id
    name
    fullName
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.deleteUser.name" == "frodo"


