import { ReactElement } from "react"
import { Role } from "../game/roleState.d"
import React from "react"
import translate from "../game/lang"
import Select, { SelectOptionsRecord } from "./Select"
import StyledText from "./StyledText"
import { getAllRoles } from "../game/roleListState.d"


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

    const optionMap: SelectOptionsRecord<Role | "none"> = {};

    if (props.canChooseNone)
        optionMap["none"] = [<StyledText noLinks={true}>{translate("none")}</StyledText>, translate("none")]
    
    for (const role in getAllRoles()) {
        if (props.enabledRoles === undefined || props.enabledRoles.includes(role as Role)) {
            optionMap[role as Role] = [<StyledText noLinks={true}>{translate("role."+role+".name")}</StyledText>, translate("role."+role+".name")]
        }
    }


    return <Select
        value={props.value??"none"}
        onChange={value => 
            props.canChooseNone===true?props.onChange(value==="none"?null:value as Role | null):
            props.onChange(value as Role)
        }
        optionsSearch={optionMap}
    />
}