import { ReactElement } from "react"
import { Role, roleJsonData, RoleState } from "../../../../game/roleState.d"
import React from "react"
import RoleDropdown from "../../../../components/RoleDropdown"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { usePlayerState } from "../../../../components/useHooks"
import Counter from "../../../../components/Counter"


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
                enabledRoles={(Object.keys(roleJsonData()) as Role[])
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