import React, { ReactElement, useCallback, useContext, useState } from "react"
import translate from "../../game/lang"
import StyledText from "../StyledText"
import { RoleOrRoleSet, getAllRoles, getRolesFromRoleOrRoleSet } from "../../game/roleListState.d";
import { Role } from "../../game/roleState.d";
import { RoleOrRoleSetSelector } from "./OutlineSelector";
import "./disabledRoleSelector.css"
import Icon from "../Icon";
import { Button } from "../Button";
import { GameModeContext } from "./GameModesEditor";




export default function EnabledRoleSelector(props: Readonly<{
    disabled?: boolean,
    onDisableRoles: (role: Role[]) => void,
    onEnableRoles: (role: Role[]) => void,
    onIncludeAll: () => void
}>): ReactElement {
    const {enabledRoles} = useContext(GameModeContext);

    const [roleOrRoleSet, setRoleOrRoleSet] = useState<RoleOrRoleSet>({ type: "roleSet", roleSet: "town" });

    const disableOutlineOption = (outline: RoleOrRoleSet) => {
        props.onDisableRoles(getRolesFromRoleOrRoleSet(outline));
    }

    const enableOutlineOption = (outline: RoleOrRoleSet) => {
        props.onEnableRoles(getRolesFromRoleOrRoleSet(outline));
    }

    const disableAll = () => {
        props.onDisableRoles(getAllRoles());
    }

    return <div className="role-specific-colors selector-section">
        <h2>{translate("menu.lobby.enabledRoles")}</h2>
        {(props.disabled !== true) && <div>
            <Button
                onClick={props.onIncludeAll}
                disabled={props.disabled}
                ><Icon>deselect</Icon> {translate("menu.enabledRoles.includeAll")}</Button>
            <Button
                onClick={disableAll}
                disabled={props.disabled}
            ><Icon>select_all</Icon> {translate("menu.enabledRoles.excludeAll")}</Button>

            <div className="disabled-role-selector-area">
                <Button
                    onClick={()=>{enableOutlineOption(roleOrRoleSet)}}
                    disabled={props.disabled}
                >{translate("menu.enabledRoles.include")}</Button>
                <Button
                    onClick={()=>{disableOutlineOption(roleOrRoleSet)}}
                    disabled={props.disabled}
                >{translate("menu.enabledRoles.exclude")}</Button>
                <RoleOrRoleSetSelector
                    disabled={props.disabled}
                    roleOrRoleSet={roleOrRoleSet}
                    onChange={setRoleOrRoleSet}
                />
            </div>
        </div>}

        <EnabledRolesDisplay 
            enabledRoles={enabledRoles}
            modifiable={!props.disabled}
            onDisableRoles={props.onDisableRoles}
            onEnableRoles={props.onEnableRoles}
            disabled={props.disabled}
        />
    </div>
}

type EnabledRolesDisplayProps = {
    enabledRoles: Role[],
} & (
    {
        modifiable: true,
        onDisableRoles: (role: Role[]) => void,
        onEnableRoles: (role: Role[]) => void,
        disabled?: boolean,
    } |
    {
        modifiable?: false,
    }
)

export function EnabledRolesDisplay(props: EnabledRolesDisplayProps): ReactElement {
    const isEnabled = useCallback((role: Role) => props.enabledRoles.includes(role), [props.enabledRoles]);

    const roleTextElement = (role: Role) => {

        return <StyledText 
            noLinks={props.modifiable ?? false}
            className={!isEnabled(role) ? "keyword-disabled" : undefined}
        >
            {translate("role."+role+".name")}
        </StyledText>
    }

    return <div>
        {getAllRoles().map((role, i) => 
            props.modifiable 
                ? <Button key={i}
                    disabled={props.disabled}
                    onClick={() => (!isEnabled(role) ? props.onEnableRoles : props.onDisableRoles)([role])}
                >
                    {roleTextElement(role)}
                </Button> 
                : <div key={i} className={"placard" + (!isEnabled(role) ? " disabled" : "")}>
                    {roleTextElement(role)}
                </div>
            
        )}
    </div>
}