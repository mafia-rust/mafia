import { ReactElement } from "react";
import { Role } from "../../../../../game/roleState.d";
import RoleDropdown from "../../../../../components/RoleDropdown";
import React from "react";

export default function RoleOptionSelectionMenu(props: Readonly<{
    selection: Role | null,
    enabledRoles?: (Role| null)[],
    onChoose: (role: Role | null)=>void,
}>): ReactElement {
    
    if(props.enabledRoles === undefined || props.enabledRoles.includes(null)){
        return <div>
            <RoleDropdown
                value={
                    ((props.selection===undefined||props.selection===null)? "jester" : props.selection) as Role
                }
                enabledRoles={props.enabledRoles?.filter(role=>role!==null) as Role[]}
                onChange={props.onChoose}
                canChooseNone={true}
            />
        </div>
    }else{
        return <div>
            <RoleDropdown
                value={
                    ((props.selection===undefined||props.selection===null)? "jester" : props.selection) as Role
                }
                enabledRoles={props.enabledRoles?.filter(role=>role!==null) as Role[]}
                onChange={props.onChoose}
                canChooseNone={false}
            />
        </div>
    }
}