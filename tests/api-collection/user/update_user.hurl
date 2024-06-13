POST {{base_url}}
Content-Type: application/json

```graphql
mutation {
  updateUser(
    input: { id: "017eba0e-ca9a-f014-fc85-1cf8a22a8999", name: "frodo" }
  ) {
    id
    name
    fullName
  }
}

```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.updateUser.name" == "frodo"


