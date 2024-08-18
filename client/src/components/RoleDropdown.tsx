import { ReactElement } from "react"
import { Role } from "../game/roleState.d"
import React from "react"
import ROLES from "../resources/roles.json"
import translate from "../game/lang"

export default function RoleDropdown(props: {
    value: Role,
    onChange: (role: Role) => void,
    enabledRoles?: Role[]
}): ReactElement {
    return <select
        value={props.value}
        onChange={e => props.onChange(e.target.value as Role)}
    >{
        Object.keys(ROLES)
            .filter((role)=>props.enabledRoles?.includes(role as Role))
            .map((role)=>{
                return <option value={role} key={role}>{translate("role."+role+".name")}</option>
            })
    }</select>
}