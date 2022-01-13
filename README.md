# Okapi

> Video/audio streaming service backend written in rust! As seems to be a yearly tradition for me I am once again rewriting my old streaming service backend ([armadillo](https://github.com/ffamilyfriendly/armadillo/)) that was a rewrite of my earliest streaming service backend ([bruh](https://github.com/ffamilyfriendly/bruh)). 

## Todo
- [ ] User accounts
    - [X] Sign up system
      - [X] without invites
      - [X] with invite
    - [ ] permissions manager
      - [X] parse permissions
      - [ ] set permissions
    - [X] remove user account
      - [X] as user (can only delete self)
      - [X] as admin (can delete anyone)
    - [ ] revoke access token
    - [ ] update access token

- [X] Invites
  - [X] generate invite
  - [X] use invite when signign up
  - [X] expired invite
  - [X] limited uses invite
  - [X] delete invite
  - [X] give perms to user (make sure perms does not exceed those of the author of the invite)

- [ ] Watch History (per user)
  - [ ] keep track of current time for entity
  - [ ] remove current time for entity
  - [ ] store session(s)? start time -> end time

- Content
  - Base Types
    - [X] Entity
      - [X] creation
      - [X] getting
      - [X] deletion
      - [X] editing
    - [ ] Metadata
      - [X] creation
      - [ ] ~~getting~~ *I see no need for this endpoint as you get the metadata thru the entity endpoint anyhow*
      - [X] deletion
      - [X] editing
    - [X] Sources
      - [X] creation
      - [X] getting
      - [X] deletion
      - [X] editing
  - [ ] Search
    - [ ] entity
    - [ ] metadata
    - [ ] combined
  - File transfer 
    - [ ] send file
    - [ ] send partials
    - [ ] check public/private content. Do not send private content to non-user