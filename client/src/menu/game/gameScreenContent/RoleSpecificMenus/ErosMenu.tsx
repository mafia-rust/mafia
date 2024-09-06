import { ReactElement } from "react";
import StyledText from "../../../../components/StyledText";
import React from "react";
import translate from "../../../../game/lang";
import GAME_MANAGER from "../../../..";
import { useGameState, usePlayerState } from "../../../../components/useHooks";

export default function ErosMenu (props: {}): ReactElement {
    const action = usePlayerState(
        playerState => playerState.roleState.type === "eros" ? playerState.roleState.action : undefined,
        ["yourRoleState"]
    )!
    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!;

    return <>
        <StyledText>
            {translate("role.eros.roleDataText", translate(action))}
        </StyledText>
        <div>
            <select
                value={action}
                onChange={e => {
                    GAME_MANAGER.sendSetErosAction(e.target.value as "loveLink" | "kill");
                }}
            >
                <option value={"loveLink"} key={"loveLink"}>{translate("loveLink")}</option>
                {
                    dayNumber > 1 &&
                    <option value={"kill"} key={"kill"}>{translate("kill")}</option>
                }
            </select>
        </div>
    </>;
}