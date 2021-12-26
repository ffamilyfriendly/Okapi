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

The content title below will be very generic and will in reality be multiple different entities but for the sake of simplicity I'll keep one abstract main checklist for content as a whole

figma board for general data structure: https://www.figma.com/file/NIEsEIonhqf0VZlX8PiGa5/Okapi-Content?node-id=0%3A1


- [ ] Content