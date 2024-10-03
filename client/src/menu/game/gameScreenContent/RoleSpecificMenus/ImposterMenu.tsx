import { ReactElement } from "react"
import RoleDropdown from "../../../../components/RoleDropdown";
import StyledText from "../../../../components/StyledText";
import translate from "../../../../game/lang";
import React from "react";
import { Role } from "../../../../game/roleState.d";
import ROLES from "../../../../resources/roles.json";
import GAME_MANAGER from "../../../..";
import { usePlayerState } from "../../../../components/useHooks";

export default function ImposterMenu(): ReactElement {

    const savedFakeRole = usePlayerState<Role | null>(
        (playerState, gameState) => {
            if(playerState.roleState.type === "imposter"){
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

    const allChoosableRoles : Role[] = Object.keys(ROLES).filter((rle)=>
        (
            GAME_MANAGER.state.stateType === "game" &&
            GAME_MANAGER.state.enabledRoles.includes(rle as Role)
        )
    ).map((r)=>r as Role);

    return <>
        {alive && <>
            <StyledText>{translate("role.imposter.roleMenu")}</StyledText>
            <RoleDropdown
                value={savedFakeRole??"jester"} 
                enabledRoles={allChoosableRoles}
                onChange={(role)=>{
                    GAME_MANAGER.sendRoleActionChoice({
                        type: "imposter", 
                        action: {
                            type: "setFakeRole",
                            role: role
                        }
                    })
                }}
            />
        </>}
    </>
}