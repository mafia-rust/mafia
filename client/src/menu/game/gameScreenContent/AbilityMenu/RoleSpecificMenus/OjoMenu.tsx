import { ReactElement } from "react"
import React from "react"
import { useGameState } from "../../../../../components/useHooks";
import GAME_MANAGER from "../../../../..";
import { Role, RoleState } from "../../../../../game/roleState.d";
import RoleDropdown from "../../../../../components/RoleDropdown";
import translate from "../../../../../game/lang";
import TwoRoleOutlineOptionInputMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionInputMenu";
import { TwoRoleOutlineOptionInput } from "../../../../../game/abilityInput";


export default function OjoMenu(
    props: {
        roleState: RoleState & {type: "ojo"}
    }
): ReactElement | null {

    const sendRoleChosen = (roleChosen: Role | null) => {
        GAME_MANAGER.sendSetRoleChosen(roleChosen);
    }

    const dayNumber = useGameState(
        state=>state.dayNumber,
        ["phase"]
    )!;

    const onInput = (chosenOutlines: TwoRoleOutlineOptionInput) => {
        const input = {
            type: "ojoInvestigate" as const,
            input: chosenOutlines
        };
        GAME_MANAGER.sendAbilityInput(input);
    }

    return <>
        <TwoRoleOutlineOptionInputMenu
            previouslyGivenResults={props.roleState.previouslyGivenResults}
            chosenOutlines={props.roleState.chosenOutline}
            onChoose={onInput}
        />
        {(dayNumber > 1) && <div>
            {translate("role.ojo.attack")}
            <RoleDropdown
                value={props.roleState.roleChosen}
                onChange={(roleOption)=>{
                    sendRoleChosen(roleOption)
                }}
                canChooseNone={true}
            />
        </div>}
    </>
}