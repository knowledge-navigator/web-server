# Knowledge Navigator Web Server REST API
Backend API gateway for the Knowledge Navigator service.

## Endpoints
See: https://github.com/dainank/know-nav-web-server-api/blob/main/src/main.rs

## Documenting
- `///` — One-line doc comment
- `/** ... */` — Block doc comment
- `//!` and `/*! ... */` — Apply doc comment to the previous block instead of the one underneath
- `//` — One-line comment (not being published)
- `/* ... */` — Block comment (not being published)

## Data Structure
```rs
Organization    [organization.rs]
└───Knowledge Navigator "Y7"    [knowledge_nav.rs]
└───Knowledge Navigator "Y8"    [knowledge_nav.rs]
└───Knowledge Navigator "Y9"    [knowledge_nav.rs]
└───Knowledge Navigator "Y10"   [knowledge_nav.rs]
└───Knowledge Navigator "Y11"   [knowledge_nav.rs]
    └───Course" Language Paper 1"   [course.rs]
    └───Course "Poetry"             [course.rs]
    └───Course "Maths Foundation"   [course.rs]
    └───Course "Maths Higher"       [course.rs]
        └───Info Chunk "Area"                   [info_chunk.rs]
        └───Info Chunk "Trigonometric Ratios"   [info_chunk.rs]
            └───Single Chunk "Sin"  [single_chunk.rs]
            └───Single Chunk "Cos"  [single_chunk.rs]
            └───Single Chunk "Tan"  [single_chunk.rs]
                │   id: "SC1"
                │   content: "tanΘ = opposite / adjacent"
                │   image: null
```

Certain 'groups' can be moderated if a teacher account has been added to the moderator list:
- `Organization`
    - name
    - description
    - moderators
    - members
    - knowledge navigators; only:
        - 'creating'
        - deleting
- `Knowledge Navigator`
    - year group
    - title
    - description
    - moderators
    - courses; only:
        - 'creating'
        - deleting
- `Course`
    - subject
    - title
    - description
    - moderators
    - info chunks

## Database
https://www.youtube.com/watch?v=Impf-Xm6oeA