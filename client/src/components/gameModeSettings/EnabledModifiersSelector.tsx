import React, { ReactElement, useContext, useState } from "react";
import { ModifierType, toModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
import "./enabledModifiersSelector.css"
import { GameModeContext } from "./GameModesEditor";
import CheckBox from "../CheckBox";
import Select, { SelectOptionsSearch, set_option_typical, set_options_typical } from "../Select";
import { conflictsWith } from "../../game/gameState.d";

export function EnabledModifiersSelector(props: Readonly<{
    disabled?: boolean,
    enabledModifiers?: ModifierType[],
    onChange?: (modifiers: ModifierType[]) => void,
}>): ReactElement {
    let { enabledModifiers } = useContext(GameModeContext);
    enabledModifiers = props.enabledModifiers ?? enabledModifiers;
    

    return <div className="chat-menu-colors selector-section">
        <h2>{translate("modifiers")}</h2>
        <EnabledModifiersDisplay
            disabled={props.disabled===undefined ? false : props.disabled}
            modifiable={!props.disabled}
            enabledModifiers={enabledModifiers}
            onDisableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    props.onChange(enabledModifiers.filter(modifier => !modifiers.includes(modifier)))
                }
            }}
            onEnableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    const conflicting = modifiers.flatMap(modifier => conflictsWith(modifier));
                    props.onChange(
                        enabledModifiers
                            .filter(modifier => !conflicting.includes(modifier))
                            .concat(modifiers)
                    )
                }
            }}
        />
    </div>
}

type EnabledModifiersDisplayProps = {
    enabledModifiers: ModifierType[],
} & (
    {
        modifiable: true,
        onDisableModifiers: (modifiers: ModifierType[]) => void,
        onEnableModifiers: (modifiers: ModifierType[]) => void,
        disabled?: boolean,
    } |
    {
        modifiable?: false,
    }
)

export function EnabledModifiersDisplay(props: EnabledModifiersDisplayProps): ReactElement { 
    let leftSideOptions: string[] = [""];
    function select<K extends string>(defaultValue: K, modifiers: ModifierType[], name: string, leftSide: boolean): ReactElement {
        const actualDefaultValue = "modifierMenu.fake."+defaultValue;
        if(leftSide){
            modifiers.forEach(modifier=>{leftSideOptions.push(modifier)})
            leftSideOptions.push(actualDefaultValue)
        }
        function selectedOption(): ModifierType | typeof actualDefaultValue {
            let selected = modifiers.find(modifier => {return props.enabledModifiers.includes(modifier)});
            return selected === undefined ? actualDefaultValue : selected
        }
        
        const optionsMap: SelectOptionsSearch<ModifierType | typeof actualDefaultValue> = new Map();
        set_option_typical(optionsMap, actualDefaultValue)
        set_options_typical(optionsMap, modifiers);

        return <>
            <td style={{paddingRight:"1em", textAlign:"right"}}>{translate("modifierMenu."+name)}</td>
            <td style={{textAlign:"left"}}><Select 
                value={selectedOption()}
                onChange={value => {
                    if(props.modifiable === true){
                        const modifier = toModifierType(value);
                        if(modifier !== undefined) {
                            props.onEnableModifiers([modifier])
                        } else {
                            props.onDisableModifiers(modifiers);
                        }
                    }
                }}
                optionsSearch = {optionsMap}
            /></td>
        </>
    }
    function checkBox(modifier: ModifierType, inverted: boolean): ReactElement {
        return <>
            <td style={{paddingRight:"1em", textAlign:"right"}}>{translate("modifierMenu."+modifier)}</td>
            <td style={{textAlign:"left"}}><CheckBox
            checked={props.enabledModifiers.includes(modifier) !== inverted}
            onChange={checked => {
                if (props.modifiable) {
                    if (checked !== inverted) {
                        props.onEnableModifiers([modifier]);
                    } else {
                        props.onDisableModifiers([modifier]);
                    }
                }
            } } /></td>
        </>
    }

    function get_padding(): ReactElement {
        let longest: number = 10;
        leftSideOptions.forEach(option => {longest = Math.max(longest, translate(option).length)})
        longest += 2;
        const padding: string = longest.toString() +"ch";
        return <tfoot>
            <tr>
                <td></td>
                <td style = {{paddingRight: padding}}></td>
                <td></td>
                <td></td>
            </tr>
        </tfoot>
    }
    const [hideDisabled, setHideDisabled] = useState(true);

    const voteOptionMap: SelectOptionsSearch<ModifierType | "popularVote"> = new Map();
    set_options_typical(voteOptionMap, ["popularVote", "twoThirdsMajority", "autoGuilty"])



    return <div>
        {!props.modifiable && <label className="centered-label">
            {translate("hideDisabled")}
            <CheckBox
                checked={hideDisabled}
                onChange={checked => setHideDisabled(checked)}
            />
        </label>}
        <div>
            <table className="modifier-table">
                <tbody>
                    <tr>{     /* Trial Phases            |   Chat            */}
                        {select("scheduledNominations", ["unscheduledNominations", "noTrialPhases"], "trialPhases", true)}
                        {select("allChat", ["noNightChat", "noChat"], "chat", false)}
                    </tr><tr>{/* Guilty Vote Requirement |   Dead Can Chat   */}
                        {select("popularVote", ["twoThirdsMajority", "autoGuilty"], "guiltyVoteRequirement", true)}
                        {checkBox("deadCanChat", false)}
                    </tr><tr>{/* Allow Abstaining        |   Whispers        */}
                        {checkBox("abstaining", false)}
                        {select("broadcastWhispers", ["hiddenWhispers", "noWhispers"], "whispers", false)}
                    </tr><tr>{/* Skip Day 1              |   Graves          */}
                        {checkBox("skipDay1", false)}
                        {select("normalGraves", ["noDeathCause", "roleSetGraveKillers", "obscuredGraves"], "graves", false)}
                    </tr>
                </tbody>
                {get_padding()}
            </table>
        </div>
    </div>
}