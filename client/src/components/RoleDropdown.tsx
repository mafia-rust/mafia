import { ReactElement } from "react"
import { Role } from "../game/roleState.d"
import React from "react"
import ROLES from "../resources/roles.json"
import translate from "../game/lang"

export default function RoleDropdown(props: {
    value: Role,
    onChange: (role: Role) => void,
    disabledRoles?: Role[]
}): ReactElement {


    let options = Object.keys(ROLES)
        .filter((role)=>!props.disabledRoles?.includes(role as Role))
        .map((role)=>{
        return <option value={role}>{translate("role."+role+".name")}</option>
    })
    
    return <select
        value={props.value}
        onChange={(e)=>{
            props.onChange(e.target.value as Role)
        }}>
            {options}
    </select>
}