import React, { ReactElement, useContext, useState } from "react";
import { ModifierType, toModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
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
    function select<K extends string>(options: (ModifierType | K)[]): ReactElement {
        function modifiersOnly<K extends string>(options: SelectOptionsSearch<ModifierType | K>): ModifierType[] {
            let result: ModifierType[] = [];
            for(const option of options){
                const modifier = toModifierType(option[0]);
                if(modifier !== undefined){
                    result.push(modifier)
                }
            }
            return result
        }
        function selectedOption<K extends string>(options: SelectOptionsSearch<ModifierType | K>): ModifierType | K | "error" {
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

        return <Select 
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
        />
    }
    function checkBox(modifier: ModifierType, inverted: boolean): ReactElement {
        return <CheckBox 
            checked={props.enabledModifiers.includes(modifier)} 
            onChange={checked => {if(props.modifiable) {
                if(checked !== inverted){
                    props.onEnableModifiers([modifier])
                } else {
                    props.onDisableModifiers([modifier])
                }
            }}}
        />
    }
    const [hideDisabled, setHideDisabled] = useState(true);

    const leftSideOptions: string[] = ["scheduledNominations", "unscheduledNominations", "noTrialPhases", "popularVote", "twoThirdsMajority", "autoGuilty"];
    let longestLeftSideOption: number = 10;
    leftSideOptions.forEach(option => {longestLeftSideOption = Math.max(longestLeftSideOption, option.length)})
    const padding: string = longestLeftSideOption.toString() +"ch";

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
            <table>
                <tr>
                    <td></td>
                    <td style = {{paddingRight: padding}}></td>
                    <td></td>
                    <td></td>
                </tr>
                <tbody>
                    <tr>        {/* Trial Phases            |   Chat            */}
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Trial Phases:</td>
                        <td style={{textAlign:"left"}}>{select(["scheduledNominations", "unscheduledNominations", "noTrialPhases"])}</td>
        
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Chat:</td>
                        <td style={{textAlign:"left"}}>{select(["allChat", "noNightChat", "noChat"])}</td>
                    </tr><tr>   {/* Guilty Vote Requirement |   Dead Can Chat   */}
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Guilty Vote Requirement:</td>
                        <td style={{textAlign:"left"}}>{select(["popularVote", "twoThirdsMajority", "autoGuilty"])}</td>

                        <td style={{paddingRight: "1em", textAlign: "right"}}>Dead Can Chat:</td>
                        <td style={{textAlign:"left"}}>{checkBox("deadCanChat", false)}</td>
                    </tr><tr>   {/* Allow Abstaining        |   Whispers        */}
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Allow Abstaining:</td>
                        <td style={{textAlign:"left"}}>{checkBox("abstaining", false)}</td>
                        
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Whispers:</td>
                        <td style={{textAlign:"left"}}>{select(["broadcastWhispers", "hiddenWhispers", "noWhispers"])}</td>
                    </tr><tr>   {/* Skip Day 1              |   Graves          */}
                        <td style={{paddingRight: "1em", textAlign: "right"}}>Skip Day 1:</td>
                        <td style={{textAlign:"left"}}>{checkBox("skipDay1", false)}</td>

                        <td style={{paddingRight: "1em", textAlign: "right"}}>Graves:</td>
                        <td style={{textAlign:"left"}}>{select(["normalGraves", "noDeathCause", "roleSetGraveKillers", "obscuredGraves"])}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
}