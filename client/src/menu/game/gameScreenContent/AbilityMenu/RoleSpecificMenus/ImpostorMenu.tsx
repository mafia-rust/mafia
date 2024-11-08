import { ReactElement } from "react"
import { usePlayerState } from "../../../../../components/useHooks";
import { Role, roleJsonData } from "../../../../../game/roleState.d";
import GAME_MANAGER from "../../../../..";
import translate from "../../../../../game/lang";
import RoleDropdown from "../../../../../components/RoleDropdown";
import React from "react";
import StyledText from "../../../../../components/StyledText";

export default function ImpostorMenu(): ReactElement {

    const savedFakeRole = usePlayerState<Role | null>(
        (playerState, gameState) => {
            if(playerState.roleState.type === "impostor"){
                return playerState.roleState.fakeRole;
            }
            return null
        },
        ["yourRoleState"]
    )
    const alive = usePlayerState<boolean>(
        (playerState, gameState) => {
            return gameState.players[playerState.myIndex].alive
        },
        ["playerAlive", "gamePlayers", "yourPlayerIndex"]
    )

    const allChoosableRoles : Role[] = Object.keys(roleJsonData()).filter((rle)=>
        (
            GAME_MANAGER.state.stateType === "game" &&
            GAME_MANAGER.state.enabledRoles.includes(rle as Role)
        )
    ).map((r)=>r as Role);

    return <>
        {alive && <>
            <StyledText>{translate("role.impostor.roleMenu")}</StyledText>
            <RoleDropdown
                value={savedFakeRole??"jester"} 
                enabledRoles={allChoosableRoles}
                onChange={(role)=>{
                    GAME_MANAGER.sendRetrainerRetrain(role);
                }}
            />
        </>}
    </>
}