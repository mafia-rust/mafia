import { ReactElement } from "react"
import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { PhaseType } from "../../../../game/gameState.d"
import Counter from "../../../../components/Counter"

export type WizardAction = 
    "Meditate" |
    "Poison" |
    "Shield" |
    "Illusion" |
    "Illuminate" |
    "Absorb" |
    "Reflect" |
    "Pyrolyze" |
    "Polymorph" |
    "Smite" |
    "Ascend"


export default function SmallWizardMenu(props: {action: WizardAction, level: number, phase: PhaseType}): ReactElement {

    const sendAction = (action: WizardAction) => {
        GAME_MANAGER.sendSetWizardAction(action);
    }

    return <>
        <Counter
            max={10}
            current={props.level}
        >
            <StyledText>{translate("role.wizard.smallRoleMenu.level", props.level)}</StyledText>
        </Counter>
        {props.phase === "night" ? <div>
            <StyledText>{translate("role.wizard.smallRoleMenu")}</StyledText>
            <ActionTypeDropdown
                action={props.action}
                onChange={(a)=>{sendAction(a)}}
            />
        </div> : null}
    </>
}

function ActionTypeDropdown(props: {
    action: WizardAction,
    onChange: (action: WizardAction) => void
}): ReactElement {
    return <select
        value={props.action}
        onChange={(e)=>{
            props.onChange(e.target.value as WizardAction)
        }}>
            <option value="poison">{translate("poison")}</option>
            <option value="string">{translate("string")}</option>
            <option value="shield">{translate("shield")}</option>
            <option value="illusion">{translate("illusion")}</option>
            <option value="illuminate">{translate("illuminate")}</option>
            <option value="absorb">{translate("absorb")}</option>
            <option value="reflect">{translate("reflect")}</option>
            <option value="pyrolyze">{translate("pyrolyze")}</option>
            <option value="polymorph">{translate("polymorph")}</option>
            <option value="smite">{translate("smite")}</option>
            <option value="ascend">{translate("ascend")}</option>
    </select>
}