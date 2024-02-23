import { ReactElement, useState } from "react"
import translate from "../game/lang"
import React from "react"
import StyledText from "./StyledText"
import { RoleOutline, getRolesFromOutline } from "../game/roleListState.d";
import { Role } from "../game/roleState.d";
import RoleOutlineSelector from "./OutlineSelector";
import "./disabledRoleSelector.css"




export default function DisabledRoleSelector(props: {
    disabled?: boolean,
    disabledRoles: Role[],
    onDisableRoles: (role: Role[]) => void,
    onEnableRoles: (role: Role[]) => void,
    onIncludeAll: () => void
}): ReactElement {

    const [roleOutline, setRoleOutline] = useState<RoleOutline>({type:"any"});

    const disableOutline = (outline: RoleOutline) => {
        let roles = getRolesFromOutline(outline);
        props.onDisableRoles(roles);
    }

    return <div className="role-specific-colors selector-section">
        <h2>{translate("menu.lobby.excludedRoles")}</h2>
        <div>
            <button
                onClick={props.onIncludeAll}
                disabled={props.disabled}
            >{translate("menu.excludedRoles.includeAll")}</button>

            <button 
                onClick={()=>{disableOutline(roleOutline)}}
                disabled={props.disabled}
            >{translate("menu.excludedRoles.exclude")}</button>

            <RoleOutlineSelector
                disabled={props.disabled}
                roleOutline={roleOutline}
                onChange={setRoleOutline}
            />
        </div>

        <div>
            {Array.from(props.disabledRoles.values()).map((value, i)=>{
                return <button key={i}
                    disabled={props.disabled}
                    onClick={()=>{props.onEnableRoles([value])}}
                >
                    <StyledText noLinks={true}>
                        {translate("role."+value+".name")}
                    </StyledText>
                </button>
            })}
        </div>
    </div>
}