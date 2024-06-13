POST {{base_url}}
Content-Type: application/json

```graphql
{
  users {
    edges {
      cursor
      node {
        id
        name
      }
    }
  }
}
```

HTTP/1.1 200
[Asserts]
jsonpath "$.errors[0].message" contains "You must provide a `first` or `last`"

