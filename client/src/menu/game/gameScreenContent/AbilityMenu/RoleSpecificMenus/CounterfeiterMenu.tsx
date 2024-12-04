import { ReactElement } from "react";
import React from "react";
import { usePlayerState } from "../../../../../components/useHooks";
import { Role, roleJsonData } from "../../../../../game/roleState.d";
import translate from "../../../../../game/lang";
import StyledText from "../../../../../components/StyledText";
import GAME_MANAGER from "../../../../..";
import { TextDropdownArea } from "../../../../../components/TextAreaDropdown";
import Counter from "../../../../../components/Counter";
import { defaultAlibi } from "../../WillMenu";

export default function CounterfeiterMenu (props: {}): ReactElement {
    
    const action = usePlayerState<"forge"|"noForge">(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.action : "noForge",
        ["yourRoleState"]
    )!;
    const forgesRemaining = usePlayerState<number>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.forgesRemaining : 0,
        ["yourRoleState"]
    )!;
    const savedRole = usePlayerState<Role>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.fakeRole : "jester",
        ["yourRoleState"]
    )!;
    const savedWill = usePlayerState<string>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.fakeWill : "",
        ["yourRoleState"]
    )!;

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
        <div className="large-forger-menu-option">
            <StyledText>{translate("role.counterfeiter.roleDataText", translate("role."+savedRole+".name"))}</StyledText>
            <select
                value={savedRole} 
                onChange={(e)=>{
                    GAME_MANAGER.sendSetForgerWill(e.target.value as Role, savedWill);
                }}
            >
                {forgerRoleOptions}
            </select>
        </div>
        <Counter max={3} current={forgesRemaining}>
            {translate("role.forger.roleDataText", forgesRemaining)}
        </Counter>
        <TextDropdownArea
            open={true}
            titleString={translate("forge")}
            savedText={savedWill}
            onSave={(text)=>{
                GAME_MANAGER.sendSetForgerWill(savedRole, text===""?defaultAlibi():text);
            }}
            cantPost={false}
        />
    </div>;
}