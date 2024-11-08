import { ReactElement } from "react";
import GAME_MANAGER from "../../../../..";
import Counter from "../../../../../components/Counter";
import React from "react";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";
import { PhaseType } from "../../../../../game/gameState.d";

export type RecruiterAction = "recruit" | "kill"

export default function RecruiterMenu(props: {action: RecruiterAction, remaining: number, phase: PhaseType, dayNumber: number}): ReactElement {

    const sendAction = (action: RecruiterAction) => {
        GAME_MANAGER.sendSetRecruiterAction(action);
    }

    return <>
        <StyledText>{translate("role.recruiter.smallRoleMenu", translate(props.action))}</StyledText>
        {props.remaining > 0 && props.dayNumber !== 1 ? <div>
            <ActionTypeDropdown
                action={props.action}
                onChange={(a)=>{sendAction(a)}}
            />
        </div> : null}
        <Counter
            max={Math.max(5, props.remaining)}
            current={props.remaining}
        >
            <StyledText>{translate("role.recruiter.smallRoleMenu.recruitsRemaining", props.remaining)}</StyledText>
        </Counter>
    </>
}

function ActionTypeDropdown(props: {
    action: RecruiterAction,
    onChange: (action: RecruiterAction) => void
}): ReactElement {
    return <select
        value={props.action}
        onChange={(e)=>{
            props.onChange(e.target.value as RecruiterAction)
        }}>
            <option value="kill">{translate("kill")}</option>
            <option value="recruit">{translate("recruit")}</option>
    </select>
}