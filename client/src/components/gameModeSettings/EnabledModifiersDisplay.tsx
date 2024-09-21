import { ReactElement } from "react";
import { MODIFIERS, ModifierType } from "../../game/gameState.d";
import React from "react";
import translate from "../../game/lang";
import StyledText from "../StyledText";

export function EnabledModifiersDisplay(props: {
    disabled: boolean,
    enabledModifiers: ModifierType[],
    onChange?: (modifiers: ModifierType[]) => void,
}): ReactElement {

    const dropdownsSelected = props.disabled === true ? props.enabledModifiers : (props.enabledModifiers as (ModifierType | null)[]).concat([null])

    return <div className="chat-menu-colors selector-section">
            <h2><StyledText>{translate("modifiers")}</StyledText></h2>
            {dropdownsSelected.map((modifier, index) => {
                return <>
                    <EnabledModifierDisplay
                        key={index}
                        modifier={modifier}
                        disabled={props.disabled}
                        choosableModifiers={
                            Object.values(MODIFIERS).filter((m) => !props.enabledModifiers.includes(m)||m===modifier)
                        }
                        onChange={modifier => {
                            if (props.onChange === undefined)
                                return;
                            
                            let currentModifiers = props.enabledModifiers;

                            if (modifier === null) {
                                currentModifiers.splice(index, 1);
                            } else {
                                currentModifiers.push(modifier);
                            }

                            //make sure to remove duplicates
                            currentModifiers = currentModifiers.filter((value, index, self) => self.indexOf(value) === index);
                            currentModifiers = currentModifiers.filter(modifier => modifier !== null);

                            props.onChange(currentModifiers);
                        }}/>
                </>
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