import { ReactElement } from "react"
import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { PhaseType } from "../../../../game/gameState.d"

export type PuppeteerAction = "string" | "poison"

export default function SmallPuppeteerMenu(props: {action: PuppeteerAction, marionettesRemaining: number, phase: PhaseType}): ReactElement {

    const sendAction = (action: PuppeteerAction) => {
        GAME_MANAGER.sendSetPuppeteerAction(action);
    }

    return <>
        {props.marionettesRemaining > 0 && <StyledText>{translate("role.puppeteer.smallRoleMenu", props.marionettesRemaining)}</StyledText>}
        {props.marionettesRemaining === 0 && <StyledText>{translate("role.puppeteer.smallRoleMenu.noneRemaining")}</StyledText>}
        {props.marionettesRemaining > 0 && props.phase === "night" ? <div>
            <ActionTypeDropdown
                action={props.action}
                onChange={(a)=>{sendAction(a)}}
            />
        </div> : null}
    </>
}

function ActionTypeDropdown(props: {
    action: PuppeteerAction,
    onChange: (action: PuppeteerAction) => void
}): ReactElement {
    return <select
        value={props.action}
        onChange={(e)=>{
            props.onChange(e.target.value as PuppeteerAction)
        }}>
            <option value="poison">{translate("poison")}</option>
            <option value="string">{translate("string")}</option>
    </select>
}