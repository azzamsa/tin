POST {{base_url}}
Content-Type: application/json

```graphql
{
  users(
    first: 2
    after: "randomZkMWIxMjZhw"
  ) {
    edges {
      cursor
      node {
        id
        name
      }
    }
    pageInfo {
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }
  }
}
```

HTTP/1.1 200
[Asserts]
jsonpath "$.errors[0].message" contains "Invalid cursor"

