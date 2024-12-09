import { ReactElement } from "react";
import React from "react";
import { Role, roleJsonData, RoleState } from "../../../../../game/roleState.d";
import translate from "../../../../../game/lang";
import StyledText from "../../../../../components/StyledText";
import GAME_MANAGER from "../../../../..";
import Counter from "../../../../../components/Counter";

export default function CounterfeiterMenu (props: Readonly<{
    roleState: RoleState & {type: "counterfeiter"}
}>): ReactElement {
    const action = props.roleState.action;
    const forgesRemaining = props.roleState.forgesRemaining;

    let forgerRoleOptions: JSX.Element[] = [];
    for(let role of Object.keys(roleJsonData()) as Role[]){
        forgerRoleOptions.push(
            <option key={role} value={role}>{translate("role."+role+".name")}</option>
        );
    }

    return <div className="large-forger-menu">
        <div className="large-forger-menu-option">
            <StyledText>{translate("role.counterfeiter.roleDataText", translate(action))}</StyledText>
            <select
                value={action}
                onChange={e => {
                    GAME_MANAGER.sendSetCounterfeiterAction(e.target.value as "forge" | "noForge");
                }}
                >
                <option value={"noForge"} key={"noForge"}>{translate("noForge")}</option>
                {
                    forgesRemaining > 0 &&
                    <option value={"forge"} key={"forge"}>{translate("forge")}</option>
                }
            </select>
        </div>
        <Counter max={3} current={forgesRemaining}>
            {translate("role.forger.roleDataText", forgesRemaining)}
        </Counter>
    </div>;
}