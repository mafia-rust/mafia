import { ReactElement, useContext } from "react";
import { MODIFIERS, ModifierType } from "../../game/gameState.d";
import React from "react";
import translate from "../../game/lang";
import StyledText from "../StyledText";
import { GameModeContext } from "./GameModesEditor";

export function EnabledModifiersDisplay(props: {
    disabled: boolean,
    enabledModifiers?: ModifierType[],
    onChange?: (modifiers: ModifierType[]) => void,
}): ReactElement {
    let {enabledModifiers} = useContext(GameModeContext);

    enabledModifiers = props.enabledModifiers ?? (enabledModifiers as ModifierType[]);

    const dropdownsSelected = props.disabled === true ? enabledModifiers : (enabledModifiers as (ModifierType | null)[]).concat([null])

    return <div className="chat-menu-colors selector-section">
            <h2><StyledText>{translate("modifiers")}</StyledText></h2>
            {dropdownsSelected.map((currentModifier, index) => {
                return <EnabledModifierDisplay
                    key={index}
                    modifier={currentModifier}
                    disabled={props.disabled}
                    choosableModifiers={
                        Object.values(MODIFIERS).filter((m) => !enabledModifiers.includes(m)||m===currentModifier)
                    }
                    onChange={newModifier => {
                        if (props.onChange === undefined)
                            return;
                        
                        let currentModifiers: (ModifierType | null)[] = [...enabledModifiers];
                        currentModifiers.splice(index, 1, newModifier);

                        //make sure to remove duplicates & null
                        const out = currentModifiers
                            .filter((value, index, self) => self.indexOf(value) === index)
                            .filter(modifier => modifier !== null) as ModifierType[];

                        props.onChange(out);
                    }}
                />
                
            })}
    </div>
}

function EnabledModifierDisplay(props: {
    modifier: ModifierType | null,
    choosableModifiers?: ModifierType[],
    disabled: boolean,
    onChange: (modifier: ModifierType | null) => void,
}): ReactElement {
    return <select
        value={props.modifier? props.modifier : "none"}
        disabled={props.disabled}
        onChange={e => props.onChange(
            e.target.value === "none"?null:e.target.value as ModifierType 
        )}
    >
        {
            Object.values(MODIFIERS)
                .filter(modifier => props.choosableModifiers===undefined || props.choosableModifiers.includes(modifier))
                .map((modifier) => {
                    return <option key={modifier} value={modifier}>{translate(modifier)}</option>
                })
                .concat([<option key="none" value="none">{translate("none")}</option>])
        }
    </select>
}