# Okapi

> Video/audio streaming service backend written in rust! As seems to be a yearly tradition for me I am once again rewriting my old streaming service backend ([armadillo](https://github.com/ffamilyfriendly/armadillo/)) that was a rewrite of my earliest streaming service backend ([bruh](https://github.com/ffamilyfriendly/bruh)). 

## Todo
- [ ] User accounts
    - [ ] Sign up system
      - [X] without invites
      - [ ] with invite
    - [ ] permissions manager
      - [X] parse permissions
      - [ ] set permissions
    - [X] remove user account
      - [X] as user (can only delete self)
      - [X] as admin (can delete anyone)
    - [ ] revoke access token
    - [ ] update access token

- [ ] Invites
  - [X] generate invite
  - [ ] use invite when signign up
  - [ ] expired invite
  - [ ] limited uses invite
  - [ ] delete invite
  - [ ] give perms to user (make sure perms does not exceed those of the author of the invite)