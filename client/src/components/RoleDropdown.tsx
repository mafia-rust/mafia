import { ReactElement } from "react"
import { Role } from "../game/roleState.d"
import React from "react"
import ROLES from "../resources/roles.json"
import translate from "../game/lang"


type RoleDropdownProps = {
    enabledRoles?: Role[]
} & ({
    value: Role,
    onChange: (role: Role) => void,
    canChooseNone?: false
} | {
    value: Role | null,
    onChange: (role: Role | null) => void,
    canChooseNone: true
})

export default function RoleDropdown(props: RoleDropdownProps): ReactElement {
    return <select
        value={props.value??"none"}
        onChange={e => 
            props.canChooseNone===true?props.onChange(e.target.value==="none"?null:e.target.value as Role | null):
            props.onChange(e.target.value as Role)
        }
    >{
        (props.canChooseNone ? [<option value={"none"} key="none">{translate("none")}</option>] : []).concat(
            Object.keys(ROLES)
                .filter((role)=>
                    props.enabledRoles === undefined ||
                    props.enabledRoles.includes(role as Role)
                )
                .map((role)=>{
                    return <option value={role} key={role}>{translate("role."+role+".name")}</option>
                })
        )
    }</select>
}