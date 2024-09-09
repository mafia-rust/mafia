import { ReactElement, useEffect } from "react"
import { Role } from "../../../../game/roleState.d"
import React from "react"
import RoleDropdown from "../../../../components/RoleDropdown"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { StateListener } from "../../../../game/gameManager.d"
import { usePlayerState } from "../../../../components/useHooks"

export type OjoAction = {
    type: "none"
} | {
    type: "kill",
    role: Role
} | {
    type: "see",
    role: Role
}

export default function SmallOjoMenu(props: {action: OjoAction}): ReactElement | null {

    const sendAction = (action: OjoAction) => {
        GAME_MANAGER.sendSetOjoAction(action);
    }
    const [dayNumber, setDayNumber] = React.useState(()=>{
        if(GAME_MANAGER.state.stateType === "game"){
            return GAME_MANAGER.state.dayNumber;
        }
        return 1;
    });

    useEffect(()=>{
        const listener: StateListener = (type)=>{
            if(type === "phase" && GAME_MANAGER.state.stateType === "game"){
                setDayNumber(GAME_MANAGER.state.dayNumber);
            }
        }

        GAME_MANAGER.addStateListener(listener);
        return ()=>GAME_MANAGER.removeStateListener(listener);
    }, [setDayNumber])

    const shouldDisplay = usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "night" && gameState.players[playerState.myIndex]?.alive,
        ["playerAlive", "yourPlayerIndex", "phase", "gamePlayers"]
    )!;

    if (!shouldDisplay) {
        return null;
    }

    return <>
        <StyledText>{translate("role.ojo.smallRoleMenu")}</StyledText>
        <div>
            <ActionTypeDropdown
                action={props.action}
                onChange={(a)=>{sendAction(a)}}
                canKill={dayNumber!==1}
            />
            {props.action.type === "none" ? null : <RoleDropdown value={props.action.role} onChange={(role)=>{
                if(props.action.type === "none") return;
                sendAction({...props.action, role: role})
            }}/>}
        </div>
    </>
}

function ActionTypeDropdown(props: {
    canKill?: boolean,
    action: OjoAction,
    onChange: (action: OjoAction) => void
}): ReactElement {
    return <select
        value={props.action.type}
        onChange={(e)=>{
            if(e.target.value === "none"){
                props.onChange({type: "none"});
                return;
            }

            switch(props.action.type){
                case "none":
                    props.onChange({type: e.target.value as "kill" | "see", role: "wildcard"})
                    break;
                case "see":
                case "kill":
                    props.onChange({type: e.target.value as "kill" | "see", role: props.action.role})
                    break;
            }
        }}>
            <option value="none">{translate("none")}</option>
            <option value="see">{translate("see")}</option>
            {props.canKill ? <option value="kill">{translate("kill")}</option> : null}
    </select>
}