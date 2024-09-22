import { ReactElement } from "react"
import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { PhaseType } from "../../../../game/gameState.d"
import Counter from "../../../../components/Counter"

export type RecruiterAction = "recruit" | "kill"

export default function RecruiterMenu(props: {action: RecruiterAction, remaining: number, phase: PhaseType, dayNumber: number}): ReactElement {

    const sendAction = (action: RecruiterAction) => {
        GAME_MANAGER.sendSetRecruiterAction(action);
    }

    return <>
        <StyledText>{translate("role.recruiter.smallRoleMenu", translate(props.action))}</StyledText>
        <Counter
            max={Math.max(5, props.remaining)}
            current={props.remaining}
        >
            <StyledText>{translate("role.recruiter.smallRoleMenu.recruitsRemaining", props.remaining)}</StyledText>
        </Counter>
        {props.remaining > 0 && props.dayNumber!==1 && props.phase === "night" ? <div>
            <ActionTypeDropdown
                action={props.action}
                onChange={(a)=>{sendAction(a)}}
            />
        </div> : null}
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
            {<option value="kill">{translate("kill")}</option>}
            <option value="recruit">{translate("recruit")}</option>
    </select>
}