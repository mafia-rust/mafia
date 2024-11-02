import { ReactElement } from "react"
import { Role, roleJsonData } from "../game/roleState.d"
import React from "react"
import translate from "../game/lang"
import Select from "./Select"
import StyledText from "./StyledText"


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

    const optionMap: Record<string, React.ReactNode> = {};

    if (props.canChooseNone)
        optionMap["none"] = <StyledText noLinks={true}>{translate("none")}</StyledText>
    
    for (const role in roleJsonData()) {
        if (props.enabledRoles === undefined || props.enabledRoles.includes(role as Role)) {
            optionMap[role] = <StyledText noLinks={true}>{translate("role."+role+".name")}</StyledText>
        }
    }


    return <Select
        value={props.value??"none"}
        onChange={value => 
            props.canChooseNone===true?props.onChange(value==="none"?null:value as Role | null):
            props.onChange(value as Role)
        }
        options={optionMap}
    />
}