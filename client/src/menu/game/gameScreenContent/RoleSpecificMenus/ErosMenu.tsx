import { ReactElement, useEffect } from "react";
import StyledText from "../../../../components/StyledText";
import React from "react";
import translate from "../../../../game/lang";
import GAME_MANAGER from "../../../..";
import { StateListener } from "../../../../game/gameManager.d";

export default function ErosMenu (props: {}): ReactElement {
    const [action, setAction] = React.useState<"loveLink" | "kill">("loveLink");
    const [dayNumber, setDayNumber] = React.useState<number>(0);

    useEffect(() => {
        const listener: StateListener = (type) => {
            if(
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.clientState.type === "player" &&
                GAME_MANAGER.state.clientState.roleState?.type === "eros"
            ){
                setAction(GAME_MANAGER.state.clientState.roleState.action);
                setDayNumber(GAME_MANAGER.state.dayNumber);
            }                
        };

        GAME_MANAGER.addStateListener(listener);
        return () => {
            GAME_MANAGER.removeStateListener(listener);
        };
    }, []);

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