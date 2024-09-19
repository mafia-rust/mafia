import { ReactElement } from "react"
import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { PhaseState, PhaseType } from "../../../../game/gameState.d"
import Counter from "../../../../components/Counter"
import { RoleState } from "../../../../game/roleState.d"

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


export default function SmallWizardMenu(
    props: {

        roleState: RoleState & { type: "wizard" }
        phase: PhaseType
    }
): ReactElement {

    const sendAction = (action: WizardAction) => {
        GAME_MANAGER.sendSetWizardAction(action);
    }

    return <>
        <StyledText>{translate("role.wizard.smallRoleMenu.level", props.roleState.level)}</StyledText>
        <Counter
            max={10}
            current={props.roleState.level}
        >
        </Counter>

        {props.phase === "night" ? <div>
            <StyledText>{translate("role.wizard.smallRoleMenu.choose")}</StyledText>
            <ActionTypeDropdown
                level={props.roleState.level}
                action={props.roleState.action}
                onChange={(a)=>{sendAction(a)}}
            />
        </div> : null}
    </>
}

function ActionTypeDropdown(props: {
    action: WizardAction,
    level: number,
    onChange: (action: WizardAction) => void
}): ReactElement {
    return <select
        value={props.action}
        onChange={(e)=>{
            props.onChange(e.target.value as WizardAction)
        }}>
            {props.level >= 0 ? <option value="meditate">{translate("meditate")}</option> : null}
            {props.level >= 0 ? <option value="shield">{translate("shield")}</option> : null}
            {props.level >= 0 ? <option value="illusion">{translate("illusion")}</option> : null}
            {props.level >= 1 ? <option value="poison">{translate("poison")}</option> : null}
            {props.level >= 2 ? <option value="illuminate">{translate("illuminate")}</option> : null}
            {props.level >= 3 ? <option value="absorb">{translate("absorb")}</option> : null}
            {props.level >= 4 ? <option value="reflect">{translate("reflect")}</option> : null}
            {props.level >= 5 ? <option value="pyrolyze">{translate("pyrolyze")}</option> : null}
            {props.level >= 6 ? <option value="polymorph">{translate("polymorph")}</option> : null}
            {props.level >= 7 ? <option value="smite">{translate("smite")}</option> : null}
            {props.level >= 8 ? <option value="ascend">{translate("ascend")}</option> : null}


    </select>
}