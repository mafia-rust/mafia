import { ReactElement } from "react";
import GAME_MANAGER from "../../../../..";
import { usePlayerState } from "../../../../../components/useHooks";
import { Role, RoleState } from "../../../../../game/roleState.d";
import Counter from "../../../../../components/Counter";
import React from "react";
import RoleDropdown from "../../../../../components/RoleDropdown";
import { getAllRoles } from "../../../../../game/roleListState.d";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";



export default function StewardMenu(
    props: {
        roleState: RoleState & {type: "steward"}
    }
): ReactElement | null {

    const sendAction = (roleChosen: Role | null) => {
        GAME_MANAGER.sendSetRoleChosen(roleChosen);
    }

    const shouldDisplay = usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "night" && gameState.players[playerState.myIndex]?.alive,
        ["playerAlive", "yourPlayerIndex", "phase", "gamePlayers"]
    )!;

    if (!shouldDisplay) {
        return null;
    }

    return <>
        <Counter max={1} current={props.roleState.stewardProtectsRemaining}><StyledText>{translate("role.steward.roleDataText", props.roleState.stewardProtectsRemaining)}</StyledText></Counter>
        <div>
            <RoleDropdown
                enabledRoles={getAllRoles()
                    .filter(role=>role!=="steward"||props.roleState.stewardProtectsRemaining!==0)
                    .filter(role=>role!==props.roleState.previousRoleChosen)
                }
                value={props.roleState.roleChosen}
                onChange={(roleOption)=>{
                    sendAction(roleOption)
                }}
                canChooseNone={true}
            />
        </div>
    </>
}