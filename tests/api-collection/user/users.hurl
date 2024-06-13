POST {{base_url}}
Content-Type: application/json

```graphql
{
  users(first: 7) {
    edges {
      node {
        name
      }
    }
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.users.edges" isEmpty


