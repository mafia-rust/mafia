import React, { ReactElement, useContext, useState } from "react";
import { ModifierType, toModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
import "./enabledModifiersSelector.css"
import { GameModeContext } from "./GameModesEditor";
import CheckBox from "../CheckBox";
import Select, { SelectOptionsSearch, set_options_typical } from "../Select";
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
    function select<K extends string>(options: (ModifierType | K)[], name: string, leftSide: boolean): ReactElement {
        if(leftSide){
            leftSideOptions.concat(options)
        }
        function modifiersOnly(options: SelectOptionsSearch<ModifierType | K>): ModifierType[] {
            let result: ModifierType[] = [];
            for(const option of options){
                const modifier = toModifierType(option[0]);
                if(modifier !== undefined){
                    result.push(modifier)
                }
            }
            return result
        }
        function selectedOption(options: SelectOptionsSearch<ModifierType | K>): ModifierType | K | "error" {
            let def: K | ModifierType | "error" = "error";
            for(const option of options) {
                const modifier = toModifierType(option[0]);
                if(modifier === undefined){
                    def = option[0]
                } else if(props.enabledModifiers.includes(modifier)) {
                    return modifier;
                }
            }
            return def;
        }
        
        const optionsMap: SelectOptionsSearch<ModifierType | K> = new Map();
        set_options_typical(optionsMap, options);

        return <>
            <td style={{paddingRight:"1em", textAlign:"right"}}>{translate(name)}</td>
            <td style={{textAlign:"left"}}><Select 
                value={selectedOption(optionsMap)}
                onChange={value => {
                    if(props.modifiable === true){
                        const modifier = toModifierType(value);
                        if(modifier !== undefined) {
                            props.onEnableModifiers([modifier])
                        } else {
                            props.onDisableModifiers(modifiersOnly(optionsMap));
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
    const [hideDisabled, setHideDisabled] = useState(true);

    let longestLeftSideOption: number = 10;
    leftSideOptions.forEach(option => {longestLeftSideOption = Math.max(longestLeftSideOption, option.length)})
    const padding: string = longestLeftSideOption.toString() +"ch";
    console.log("fuck1 "+padding)
    console.log("fuck2 "+leftSideOptions.toString())

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
                <thead>
                    <tr>
                        <td></td>
                        <td style = {{paddingRight: padding}}></td>
                        <td></td>
                        <td></td>
                    </tr>
                </thead>
                <tbody>
                    <tr>{     /* Trial Phases            |   Chat            */}
                        {select(["modifierMenu.fake.scheduledNominations", "unscheduledNominations", "noTrialPhases"], "modifierMenu.trialPhases", true)}
                        {select(["modifierMenu.fake.allChat", "noNightChat", "noChat"], "modifierMenu.chat", false)}
                    </tr><tr>{/* Guilty Vote Requirement |   Dead Can Chat   */}
                        {select(["modifierMenu.fake.popularVote", "twoThirdsMajority", "autoGuilty"], "modifierMenu.guiltyVoteRequirement", true)}
                        {checkBox("deadCanChat", false)}
                    </tr><tr>{/* Allow Abstaining        |   Whispers        */}
                        {checkBox("abstaining", false)}
                        {select(["modifierMenu.fake.broadcastWhispers", "hiddenWhispers", "noWhispers"], "modifierMenu.whispers", false)}
                    </tr><tr>{/* Skip Day 1              |   Graves          */}
                        {checkBox("skipDay1", false)}
                        {select(["modifierMenu.fake.normalGraves", "noDeathCause", "roleSetGraveKillers", "obscuredGraves"], "modifierMenu.graves", false)}
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
}