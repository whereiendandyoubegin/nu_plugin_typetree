# nu_plugin_typetree

A schema based tree for visualising piped `describe --detailed output`.

## Example:

Before
```
http get https://jsonplaceholder.typicode.com/users | first | describe
record<id: int, name: string, username: string, email: string, address: record<street: string, suite: string, city: string, zipcode: string, geo: record<lat: string, lng: string>>, phone: string, website: string, company: record<name: string, catchPhrase: string, bs: string>>
```
After
```
http get https://jsonplaceholder.typicode.com/users | first | describe --detailed | typetree

record
├─ id: int
├─ name: string
├─ username: string
├─ email: string
├─ address: record
│  ├─ street: string
│  ├─ suite: string
│  ├─ city: string
│  ├─ zipcode: string
│  └─ geo: record
│     ├─ lat: string
│     └─ lng: string
├─ phone: string
├─ website: string
└─ company: record
   ├─ name: string
   ├─ catchPhrase: string
   └─ bs: string
```
