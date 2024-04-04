import { ReactElement, useState } from "react"
import translate from "../game/lang"
import React from "react"
import StyledText from "./StyledText"
import { RoleOutlineOption, getAllRoles, getRolesFromOutlineOption } from "../game/roleListState.d";
import { Role } from "../game/roleState.d";
import { RoleOutlineOptionSelector } from "./OutlineSelector";
import "./disabledRoleSelector.css"
import Icon from "./Icon";
import { Button } from "./Button";




export default function DisabledRoleSelector(props: {
    disabled?: boolean,
    disabledRoles: Role[],
    onDisableRoles: (role: Role[]) => void,
    onEnableRoles: (role: Role[]) => void,
    onIncludeAll: () => void
}): ReactElement {

    const [roleOutlineOption, setRoleOutlineOption] = useState<RoleOutlineOption | "any">("any");

    const disableOutlineOption = (outline: RoleOutlineOption | "any") => {
        let roles;
        if (outline === "any") roles = getAllRoles()
        else roles = getRolesFromOutlineOption(outline);
        props.onDisableRoles(roles);
    }

    return <div className="role-specific-colors selector-section">
        <h2>{translate("menu.lobby.excludedRoles")}</h2>
        <div>
            <Button
                onClick={props.onIncludeAll}
                disabled={props.disabled}
            ><Icon>deselect</Icon> {translate("menu.excludedRoles.includeAll")}</Button>

            <div className="disabled-role-selector-area">
                <Button
                    onClick={()=>{disableOutlineOption(roleOutlineOption)}}
                    disabled={props.disabled}
                ><Icon>select_all</Icon> {translate("menu.excludedRoles.exclude")}</Button>
                <RoleOutlineOptionSelector
                    disabled={props.disabled}
                    roleOutlineOption={roleOutlineOption}
                    onChange={setRoleOutlineOption}
                />
            </div>
        </div>

        <div>
            {Array.from(props.disabledRoles.values()).map((value, i)=>{
                return <Button key={i}
                    disabled={props.disabled}
                    onClick={()=>{props.onEnableRoles([value])}}
                >
                    <StyledText noLinks={true}>
                        {translate("role."+value+".name")}
                    </StyledText>
                </Button>
            })}
        </div>
    </div>
}