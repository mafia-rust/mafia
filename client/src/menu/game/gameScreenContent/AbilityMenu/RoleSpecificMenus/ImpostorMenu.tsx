import { ReactElement } from "react"
import { usePlayerState } from "../../../../../components/useHooks";
import { Role, roleJsonData, RoleState } from "../../../../../game/roleState.d";
import GAME_MANAGER from "../../../../..";
import translate from "../../../../../game/lang";
import React from "react";
import StyledText from "../../../../../components/StyledText";
import RoleOptionSelectionMenu from "../AbilitySelectionTypes/RoleOptionSelectionMenu";

export default function ImpostorMenu(props: Readonly<{
    roleState: RoleState & {type: "impostor"},
}>): ReactElement {

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
            <RoleOptionSelectionMenu
                selection={props.roleState.fakeRole??"jester"}
                enabledRoles={allChoosableRoles}
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