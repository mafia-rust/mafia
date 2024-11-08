import { ReactElement } from "react";
import React from "react";
import { usePlayerState } from "../../../../../components/useHooks";
import { Role } from "../../../../../game/roleState.d";
import { getAllRoles } from "../../../../../game/roleListState.d";
import translate from "../../../../../game/lang";
import GAME_MANAGER from "../../../../..";
import { TextDropdownArea } from "../../../../../components/TextAreaDropdown";
import Counter from "../../../../../components/Counter";
import { defaultAlibi } from "../../WillMenu";

export default function ForgerMenu (props: {}): ReactElement {
    
    const forgesRemaining = usePlayerState<number>(
        playerState => playerState.roleState.type === "forger" ? playerState.roleState.forgesRemaining : 0,
        ["yourRoleState"]
    )!;
    const savedRole = usePlayerState<Role>(
        playerState => playerState.roleState.type === "forger" ? playerState.roleState.fakeRole : "jester",
        ["yourRoleState"]
    )!;
    const savedWill = usePlayerState<string>(
        playerState => playerState.roleState.type === "forger" ? playerState.roleState.fakeWill : "",
        ["yourRoleState"]
    )!;

    let forgerRoleOptions: JSX.Element[] = [];
    for(let role of getAllRoles()){
        forgerRoleOptions.push(
            <option key={role} value={role}>{translate("role."+role+".name")}</option>
        );
    }

    return <div className="large-forger-menu">
        <div>
            <select
                value={savedRole} 
                onChange={(e)=>{
                    GAME_MANAGER.sendSetForgerWill(e.target.value as Role, savedWill);
                }}
            >
                {forgerRoleOptions}
            </select>
            <Counter max={3} current={forgesRemaining}>
                {translate("role.forger.menu.forgesRemaining", forgesRemaining)}
            </Counter>
        </div>
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