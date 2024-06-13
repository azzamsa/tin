POST {{base_url}}
Content-Type: application/json

```graphql
mutation {
  createUser(input: { name: "aragorn", email: "aragon@email.com" }) {
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
jsonpath "$.data.createUser.name" == "aragorn"


