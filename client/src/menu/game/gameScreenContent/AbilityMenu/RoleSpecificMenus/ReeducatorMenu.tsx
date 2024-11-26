import { ReactElement } from "react"
import { useGameState, usePlayerState } from "../../../../../components/useHooks";
import { getSingleRoleJsonData, RoleState } from "../../../../../game/roleState.d";
import GAME_MANAGER from "../../../../..";
import React from "react";
import RoleOptionSelectionMenu from "../AbilitySelectionTypes/RoleOptionSelectionMenu";

export default function ReeducatorMenu(props: Readonly<{
    roleState: RoleState & {type: "reeducator"},
}>): ReactElement {

    const alive = usePlayerState<boolean>(
        (playerState, gameState) => {
            return gameState.players[playerState.myIndex].alive
        },
        ["playerAlive", "gamePlayers", "yourPlayerIndex"]
    );

    const enabledSyndicateSupportRoles = useGameState(
        (gameState) => {
            return gameState.enabledRoles.filter((role) => getSingleRoleJsonData(role).roleSets.includes("mafiaSupport"))
        },
        ["enabledRoles"]
    );

    return <>
        {alive && <>
            <RoleOptionSelectionMenu
                selection={props.roleState.convertRole}
                enabledRoles={enabledSyndicateSupportRoles}
                onChoose={(role)=>{
                    GAME_MANAGER.sendAbilityInput({
                        type: "disguiser",
                        input: role
                    });
                }}
            />
        </>}
    </>
}