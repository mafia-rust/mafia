

enum Faction{
    Mafia,
    Town,
    Neutral,
    Coven,
}
enum FactionAlignment{
    MafiaKilling,
    MafiaDeception,
    MafiaSupport,

    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralBenign,
    NeutralChaos,

    CovenEvil
}

struct RoleList{
    role_list: Vec<RoleListObject>
}
impl Vec<RoleListObject>{
    
}

enum RoleListObject{
    Exact(Role),
    FactionAlignment(FactionAlignment),
    Faction(Faction),
    Any
}
impl RoleListObject{
    pub fn get_random_role()->Role{

    }
    pub fn get_possible_roles()->Role{

    }
}