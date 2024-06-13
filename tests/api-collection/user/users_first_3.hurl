POST {{base_url}}
Content-Type: application/json

```graphql
{
  users(first: 3) {
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
    totalCount
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200


