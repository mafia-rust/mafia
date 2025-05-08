import React, { ReactElement, useContext } from "react";
import { ModifierType, toModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
import { GameModeContext } from "./GameModesEditor";
import CheckBox from "../CheckBox";
import Select, { SelectOptionsSearch, set_option_typical as setOptionTypical, set_options_typical as setOptionsTypical } from "../Select";

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
            onEnableDisableModifiers={(enable: ModifierType[], disable: ModifierType[])=>{
                if (props.onChange) {
                    props.onChange(
                        enabledModifiers
                            .filter(modifier => !disable.includes(modifier))
                            .concat(enable)
                    )
                }
            }}
            onDisableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    props.onChange(enabledModifiers.filter(modifier => !modifiers.includes(modifier)))
                }
            }}
            onEnableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    props.onChange(
                        enabledModifiers.concat(modifiers)
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
        onEnableDisableModifiers:(enable: ModifierType[], disable: ModifierType[]) => void,
        onDisableModifiers: (modifiers: ModifierType[]) => void,
        onEnableModifiers: (modifiers: ModifierType[]) => void,
        disabled?: boolean,
    } |
    {
        modifiable?: false,
    }
)

type ModifierSelectionType =
    {
        type: "dropdown",
        fakeModifier: string,
        modifiers: ModifierType[],
    } | {
        type: "checkbox",
        modifier: ModifierType,
        inverted: boolean,
    }
    
class ModifierSelection {
    name: string
    data: ModifierSelectionType
    constructor(name: string, data: ModifierSelectionType) {
        this.name = name
        this.data = data
    }
    generateDisplay(props: EnabledModifiersDisplayProps, columnOptions: string[] | undefined) {
        switch(this.data.type){
            case "dropdown":
                const data = this.data;
                const defaultValue = "modifierMenu.fake."+data.fakeModifier;

                if(columnOptions !== undefined) {
                    data.modifiers.forEach(modifier=>{columnOptions.push(modifier)})
                    columnOptions.push(defaultValue)
                }

                function selectedOption(): ModifierType | typeof defaultValue {
                    let selected = data.modifiers.find(modifier => {return props.enabledModifiers.includes(modifier)});
                    return selected === undefined ? defaultValue : selected
                }
                
                const optionsMap: SelectOptionsSearch<ModifierType | typeof defaultValue> = new Map();
                setOptionTypical(optionsMap, defaultValue)
                setOptionsTypical(optionsMap, data.modifiers);

                return <>
                    <td style={{paddingRight:"1em", textAlign:"right"}}>{translate("modifierMenu."+this.name)}</td>
                    <td style={{textAlign:"left"}}><Select 
                        value={selectedOption()}
                        onChange={value => {
                            if(props.modifiable === true){
                                const modifier = toModifierType(value);
                                if(modifier !== undefined) {
                                    props.onEnableDisableModifiers([modifier], data.modifiers)
                                } else {
                                    props.onDisableModifiers(data.modifiers);
                                }
                            }
                        }}
                        optionsSearch = {optionsMap}
                    /></td>
                </>            

            case "checkbox":
                const data_ = this.data;
                return <>
                    <td style={{paddingRight:"1em", textAlign:"right"}}>{translate(this.name)}</td>
                    <td style={{textAlign:"left"}}><CheckBox
                    checked={props.enabledModifiers.includes(data_.modifier) !== data_.inverted}
                    onChange={checked => {
                        if (props.modifiable) {
                            if (checked !== data_.inverted) {
                                props.onEnableModifiers([data_.modifier]);
                            } else {
                                props.onDisableModifiers([data_.modifier]);
                            }
                        }
                    } } /></td>
                </>
        }
    }
    static dropdown(name: string, fakeModifier: string, modifiers: ModifierType[]): ModifierSelection {
        return new ModifierSelection(
                name, 
            {
                type: "dropdown",
                fakeModifier: fakeModifier,
                modifiers: modifiers
            }
        )
    }
    static checkbox(modifier: ModifierType, inverted: boolean, name: string ="modifierMenu."+modifier): ModifierSelection {
        return new ModifierSelection(
                name, 
            {
                type: "checkbox",
                modifier: modifier,
                inverted: inverted,
            }
        )
    }
}

const MODIFIER_SELECTIONS = [
    ModifierSelection.dropdown("trialPhases", "scheduledNominations", ["unscheduledNominations", "noTrialPhases"]),
    ModifierSelection.checkbox("abstaining", false),
    ModifierSelection.dropdown("guiltyVoteRequirement", "popularVote", ["twoThirdsMajority", "autoGuilty"]),
    ModifierSelection.dropdown("graveInfo", "roleGraveKillers", ["noDeathCause", "roleSetGraveKillers", "obscuredGraves"]),
    ModifierSelection.checkbox("skipDay1", false),
    ModifierSelection.dropdown("chat", "allChat", ["noNightChat", "noChat"]),
    ModifierSelection.checkbox("deadCanChat", false),
    ModifierSelection.dropdown("whispers", "broadcastWhispers", ["hiddenWhispers", "noWhispers"]),
] as const


export function EnabledModifiersDisplay(props: EnabledModifiersDisplayProps): ReactElement {
    let columnPairs = 2;
    let columnsOptions: string[][] = [];
    for(let i = 1; i<columnPairs; i++){//intentionally is 1 less
        columnsOptions.push([])
    }
    function get_padding(): ReactElement {
        function columnPairPadding(columnOptions: string[]): ReactElement {
            let longest: number = 3;
            columnOptions.forEach(option => {longest = Math.max(longest, translate(option).length)})
            longest += 2;
            return <>
                <td></td>
                <td style = {{paddingRight: longest.toString() +"ch"}}></td>
            </>
        }
        let paddings: ReactElement =  <></>;
        for(let i = 0; i+1<columnPairs; i++){
            paddings = <>{paddings}{columnPairPadding(columnsOptions[i])}{}</>;
        }
        return <tfoot>
            {paddings}
            <td></td>
            <td></td>
        </tfoot>
    }
    let tbody = <></>
    
    for(let rowIndex = 0; rowIndex<=MODIFIER_SELECTIONS.length/columnPairs; rowIndex++){
        let row = <></>;
        for(let i = 0; i<columnPairs; i++){
            let selection = MODIFIER_SELECTIONS[i+columnPairs*rowIndex];
            if(selection == undefined) {
                row = <>{row}<td></td><td></td></>
            } else {
                row = <>{row}{selection.generateDisplay(props, columnsOptions[i])}</>
            }
        }
        tbody = <>{tbody}{<tr>{row}</tr>}</>
    }

    return <div>
        <table style={{width:"100%"}}>
            <tbody >
                {tbody}
            </tbody>
            {get_padding()}
        </table>
    </div>
}