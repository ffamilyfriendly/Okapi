pub enum UserPermissions {
    Administrator = 1 << 0,
    GenerateInvite = 1 << 1,
    PrivateContent = 1 << 2
}

pub fn has_permission(p: u16, perm: UserPermissions) -> bool {

    // bypass permission check if user is admin
    let admin_as_int: u16 = UserPermissions::Administrator as u16;
    if (p & admin_as_int) == admin_as_int { return true; }

    let perm_as_int: u16 = perm as u16;
    (p & perm_as_int) == perm_as_int 
}

pub fn has_any_permission(p: u16, perms: Vec<UserPermissions>) -> bool {
    let mut rv = false;
    for perm in perms {
        if has_permission(p, perm) {
            rv = true;
            break;
        }
    }
    rv
}

pub fn has_all_permissions(p: u16, perms: Vec<UserPermissions>) -> bool {
    let mut rv = true;
    for perm in perms {
        if !has_permission(p, perm) {
            rv = false;
            break;
        }
    }
    rv
}