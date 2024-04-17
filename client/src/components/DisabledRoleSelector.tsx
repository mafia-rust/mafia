import { ReactElement, useCallback, useContext, useState } from "react"
import translate from "../game/lang"
import React from "react"
import StyledText from "./StyledText"
import { RoleOutlineOption, getAllRoles, getRolesFromOutlineOption } from "../game/roleListState.d";
import { Role } from "../game/roleState.d";
import { RoleOutlineOptionSelector } from "./OutlineSelector";
import "./disabledRoleSelector.css"
import Icon from "./Icon";
import { Button } from "./Button";
import { GameModeContext } from "./GameModesEditor";




export default function DisabledRoleSelector(props: {
    disabled?: boolean,
    onDisableRoles: (role: Role[]) => void,
    onEnableRoles: (role: Role[]) => void,
    onIncludeAll: () => void
}): ReactElement {
    const {disabledRoles} = useContext(GameModeContext);

    const [roleOutlineOption, setRoleOutlineOption] = useState<RoleOutlineOption>({ type: "faction", faction: "town" });

    const disableOutlineOption = (outline: RoleOutlineOption) => {
        props.onDisableRoles(getRolesFromOutlineOption(outline));
    }

    const enableOutlineOption = (outline: RoleOutlineOption) => {
        props.onEnableRoles(getRolesFromOutlineOption(outline));
    }

    const disableAll = () => {
        props.onDisableRoles(getAllRoles());
    }

    return <div className="role-specific-colors selector-section">
        <h2>{translate("menu.lobby.excludedRoles")}</h2>
        <div>
            <Button
                onClick={props.onIncludeAll}
                disabled={props.disabled}
            ><Icon>deselect</Icon> {translate("menu.excludedRoles.includeAll")}</Button>
            <Button
                onClick={disableAll}
                disabled={props.disabled}
            ><Icon>select_all</Icon> {translate("menu.excludedRoles.excludeAll")}</Button>

            <div className="disabled-role-selector-area">
                <Button
                    onClick={()=>{enableOutlineOption(roleOutlineOption)}}
                    disabled={props.disabled}
                >{translate("menu.excludedRoles.include")}</Button>
                <Button
                    onClick={()=>{disableOutlineOption(roleOutlineOption)}}
                    disabled={props.disabled}
                >{translate("menu.excludedRoles.exclude")}</Button>
                <RoleOutlineOptionSelector
                    excludeAny={true}
                    disabled={props.disabled}
                    roleOutlineOption={roleOutlineOption}
                    onChange={setRoleOutlineOption}
                />
            </div>
        </div>

        <DisabledRolesDisplay 
            disabledRoles={disabledRoles}
            modifiable={true}
            onDisableRoles={props.onDisableRoles}
            onEnableRoles={props.onEnableRoles}
            disabled={props.disabled}
        />
    </div>
}

type DisabledRoleDisplayProps = {
    disabledRoles: Role[],
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

export function DisabledRolesDisplay(props: DisabledRoleDisplayProps): ReactElement {
    const isDisabled = useCallback((role: Role) => props.disabledRoles.includes(role), [props.disabledRoles]);

    const roleTextElement = (role: Role) => {
        return <StyledText 
            noLinks={props.modifiable}
            className={isDisabled(role) ? "keyword-disabled" : undefined}
        >
            {translate("role."+role+".name")}
        </StyledText>
    }

    return <div>
        {getAllRoles().map((role, i) => 
            props.modifiable 
                ? <Button key={i}
                    disabled={props.disabled}
                    onClick={() => (isDisabled(role) ? props.onEnableRoles : props.onDisableRoles)([role])}
                >
                    {roleTextElement(role)}
                </Button> 
                : <div key={i} className={"disabled-role-element" + (isDisabled(role) ? " disabled" : "")}>
                    {roleTextElement(role)}
                </div>
            
        )}
    </div>
}