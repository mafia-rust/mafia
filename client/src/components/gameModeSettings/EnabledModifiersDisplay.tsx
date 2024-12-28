import { ReactElement, useContext } from "react";
import { MODIFIERS, ModifierType } from "../../game/gameState.d";
import React from "react";
import translate from "../../game/lang";
import StyledText from "../StyledText";
import { GameModeContext } from "./GameModesEditor";
import Select, { SelectOptionsSearch } from "../Select";

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

    const optionMap: SelectOptionsSearch<ModifierType | "none"> = new Map();
    optionMap.set("none", [
        <StyledText noLinks={true}>{translate("none")}</StyledText>,
        translate("none")
    ]);
    for (let modifier of (props.choosableModifiers ?? MODIFIERS)) {
        optionMap.set(modifier, [
            <StyledText noLinks={true}>{translate(modifier)}</StyledText>,
            translate(modifier)
        ]);
    }

    return <Select
        value={props.modifier === null ?  "none" : props.modifier}
        disabled={props.disabled}
        optionsSearch={optionMap}
        onChange={e => props.onChange(
            e === "none" ? null : e as ModifierType 
        )}
    />
}