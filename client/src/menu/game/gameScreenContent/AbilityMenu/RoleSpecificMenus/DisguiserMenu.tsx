import { ReactElement } from "react"
import { usePlayerState } from "../../../../../components/useHooks";
import { RoleState } from "../../../../../game/roleState.d";
import GAME_MANAGER from "../../../../..";
import React from "react";
import RoleOptionSelectionMenu from "../AbilitySelectionTypes/RoleOptionSelectionMenu";

export default function DisguiserMenu(props: Readonly<{
    roleState: RoleState & {type: "disguiser"},
}>): ReactElement {

    const alive = usePlayerState<boolean>(
        (playerState, gameState) => {
            return gameState.players[playerState.myIndex].alive
        },
        ["playerAlive", "gamePlayers", "yourPlayerIndex"]
    )

    return <>
        {alive && <>
            <RoleOptionSelectionMenu
                selection={props.roleState.disguisedRole}
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