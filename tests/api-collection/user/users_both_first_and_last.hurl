POST {{base_url}}
Content-Type: application/json

```graphql
{
  users(first: 3, last: 3) {
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
jsonpath "$.errors[0].message" contains "Passing both `first` and `last` for pagination is not supported"

