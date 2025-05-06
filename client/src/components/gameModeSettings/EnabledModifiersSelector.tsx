import React, { ReactElement, useContext, useState } from "react";
import { ModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
import { GameModeContext } from "./GameModesEditor";
import CheckBox from "../CheckBox";
import Select, { SelectOptionsSearch, set_options_typical } from "../Select";
import { conflicts_with } from "../../game/gameState.d";

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
                    const conflicting = modifiers.flatMap(modifier => conflicts_with(modifier));
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
    const [hideDisabled, setHideDisabled] = useState(true);

    const trialPhasesOptionMap: SelectOptionsSearch<ModifierType | "scheduledNominations"> = new Map();
    set_options_typical(trialPhasesOptionMap, ["scheduledNominations", "unscheduledNominations", "noTrialPhases"])

    const voteOptionMap: SelectOptionsSearch<ModifierType | "popularVote"> = new Map();
    set_options_typical(voteOptionMap, ["popularVote", "twoThirdsMajority", "autoGuilty"])
    
    const whisperOptionMap: SelectOptionsSearch<ModifierType | "broadcastWhispers"> = new Map();
    set_options_typical(whisperOptionMap, ["broadcastWhispers", "hiddenWhispers", "noWhispers"])

    const chatOptionMap: SelectOptionsSearch<ModifierType | "allChat"> = new Map();
    set_options_typical(chatOptionMap, ["allChat", "noNightChat", "noChat"])

    const graveOptionMap: SelectOptionsSearch<ModifierType | "normalGraves"> = new Map();
    set_options_typical(graveOptionMap, ["normalGraves", "noDeathCause", "roleSetGraveKillers", "obscuredGraves"])

    const leftSideOptions: string[] = []; 
    trialPhasesOptionMap.forEach(option => {leftSideOptions.push(option[1])});
    voteOptionMap.forEach(option => {leftSideOptions.push(option[1])});

    let longestLeftSideOption: number = 10;
    leftSideOptions.forEach(option => {longestLeftSideOption = Math.max(longestLeftSideOption, option.length)})
    const padding: string = longestLeftSideOption.toString() +"ch";

    return <div>
        {!props.modifiable && <label className="centered-label">
            {translate("hideDisabled")}
            <CheckBox
                checked={hideDisabled}
                onChange={checked => setHideDisabled(checked)}
            />
        </label>}
        {/* <div>
            {MODIFIERS
                .filter(role => isEnabled(role) || !hideDisabled || props.modifiable)
                .sort((a, b) => props.modifiable ? 0 : (isEnabled(a) ? -1 : 1) - (isEnabled(b) ? -1 : 1))
                .map((modifier, i) => 
                props.modifiable 
                    ? <Button key={modifier}
                        disabled={props.disabled}
                        onClick={() => (!isEnabled(modifier) ? props.onEnableModifiers : props.onDisableModifiers)([modifier])}
                    >
                        {modifierTextElement(modifier)}
                    </Button> 
                    : <div key={modifier} className={"placard" + (!isEnabled(modifier) ? " disabled" : "")}>
                        {modifierTextElement(modifier)}
                    </div>
            )}
        </div> */}
        <div>
            <table>
                <tr>        {/* Trial Phases            |   Chat            */}
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Trial Phases:</td>
                    <td style={{textAlign:"left"}}><Select 
                        value={
                            props.enabledModifiers.includes("noTrialPhases") ? 
                            "noTrialPhases" : (
                                props.enabledModifiers.includes("unscheduledNominations") ? 
                                "unscheduledNominations" :
                                "scheduledNominations"
                            )
                        }
                        onChange={value => {
                            if(props.modifiable === true){
                                if(value === "unscheduledNominations" || value === "noTrialPhases") {
                                    props.onEnableModifiers([value])
                                } else {
                                    props.onDisableModifiers(["unscheduledNominations", "noTrialPhases"]);
                                }
                            }
                        }}
                        optionsSearch = {trialPhasesOptionMap}
                    /></td>
    
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Chat:</td>
                    <td style={{textAlign:"left"}}><Select
                        value={
                            props.enabledModifiers.includes("noChat") ? 
                            "noChat" : (
                                props.enabledModifiers.includes("noNightChat") ? 
                                "noNightChat" :
                                "allChat"
                            )
                        }
                        onChange={value => {
                            if(props.modifiable === true){
                                if(value === "noChat" || value === "noNightChat") {
                                    props.onEnableModifiers([value])
                                } else {
                                    props.onDisableModifiers(["noChat", "noNightChat"]);
                                }
                            }
                        }}
                        optionsSearch = {chatOptionMap}
                    /></td>
                </tr><tr>   {/* Guilty Vote Requirement |   Dead Can Chat   */}
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Guilty Vote Requirement:</td>
                    <td style={{textAlign:"left"}}><Select
                        value={
                            props.enabledModifiers.includes("autoGuilty") ? 
                            "autoGuilty" : (
                                props.enabledModifiers.includes("twoThirdsMajority") ? 
                                "twoThirdsMajority" :
                                "popularVote"
                            )
                        }
                        onChange={value => {
                            if(props.modifiable === true){
                                if(value === "autoGuilty" || value === "twoThirdsMajority") {
                                    props.onEnableModifiers([value])
                                } else {
                                    props.onDisableModifiers(["autoGuilty", "twoThirdsMajority"]);
                                }
                            }
                        }}
                        optionsSearch = {voteOptionMap}
                    /></td>

                    <td style={{paddingRight: "1em", textAlign: "right"}}>Dead Can Chat:</td>
                    <td style={{textAlign:"left"}}><CheckBox 
                        checked={props.enabledModifiers.includes("deadCanChat")} 
                        onChange={checked => {if(props.modifiable) {
                            if(checked){
                                props.onEnableModifiers(["deadCanChat"])
                            } else {
                                props.onDisableModifiers(["deadCanChat"])
                            }
                        }}}
                    /></td>
                </tr><tr>   {/* Allow Abstaining        |   Whispers        */}
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Allow Abstaining:</td>
                    <td style={{textAlign:"left", paddingRight: padding}}><CheckBox
                        checked={props.enabledModifiers.includes("abstaining")}
                        onChange={checked => {if(props.modifiable) {
                            if(checked){
                                props.onEnableModifiers(["abstaining"])
                            } else {
                                props.onDisableModifiers(["abstaining"])
                            }
                        }}}
                    /></td>
                    
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Whispers:</td>
                    <td style={{textAlign:"left"}}><Select 
                        value={
                            props.enabledModifiers.includes("noWhispers") ? 
                            "noWhispers" : (
                                props.enabledModifiers.includes("hiddenWhispers") ? 
                                "hiddenWhispers" :
                                "broadcastWhispers"
                            )
                        }
                        onChange={value => {
                            if(props.modifiable === true){
                                if(value === "noWhispers" || value === "hiddenWhispers") {
                                    props.onEnableModifiers([value])
                                } else {
                                    props.onDisableModifiers(["noWhispers", "hiddenWhispers"]);
                                }
                            }
                        }}
                        optionsSearch = {whisperOptionMap}
                    /></td>
                </tr><tr>   {/* Skip Day 1              |   Graves          */}
                    <td style={{paddingRight: "1em", textAlign: "right"}}>Skip Day 1:</td>
                    <td style={{textAlign:"left"}}><CheckBox 
                        checked={props.enabledModifiers.includes("skipDay1")} 
                        onChange={checked => {if(props.modifiable) {
                            if(checked){
                                props.onEnableModifiers(["skipDay1"])
                            } else {
                                props.onDisableModifiers(["skipDay1"])
                            }
                        }}}
                    /></td>

                    <td style={{paddingRight: "1em", textAlign: "right"}}>Graves:</td>
                    <td style={{textAlign:"left"}}><Select 
                        value={
                            props.enabledModifiers.includes("obscuredGraves") ? 
                            "obscuredGraves" : (
                                props.enabledModifiers.includes("noDeathCause") ? 
                                "noDeathCause" :
                                (
                                    props.enabledModifiers.includes("roleSetGraveKillers") ?
                                    "roleSetGraveKillers" :
                                    "normalGraves"
                                )
                            )
                        }
                        onChange={value => {
                            if(props.modifiable === true){
                                if(value === "obscuredGraves" || value === "noDeathCause" || value === "roleSetGraveKillers") {
                                    props.onEnableModifiers([value])
                                } else {
                                    props.onDisableModifiers(["obscuredGraves", "noDeathCause", "roleSetGraveKillers"]);
                                }
                            }
                        }}
                        optionsSearch = {graveOptionMap}
                    /></td>
                </tr>
            </table>
        </div>
    </div>
}