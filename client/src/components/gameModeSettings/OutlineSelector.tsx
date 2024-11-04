import React, { useContext } from "react";
import "./outlineSelector.css";
import translate from "../../game/lang";
import { ROLE_SETS, RoleList, RoleOutline, RoleOutlineOption, simplifyRoleOutline, translateRoleOutlineOption} from "../../game/roleListState.d";
import { Role, roleJsonData } from "../../game/roleState.d";
import Icon from "../Icon";
import { DragAndDrop } from "../DragAndDrop";
import { GameModeContext } from "./GameModesEditor";
import Select, { SelectOptionsRecord } from "../Select";
import StyledText from "../StyledText";

type RoleOutlineSelectorProps = {
    roleOutline: RoleOutline,
    onChange: (value: RoleOutline) => void,
    disabled?: boolean,
}

export default class RoleOutlineSelector extends React.Component<RoleOutlineSelectorProps> {
    
    handleRoleOutlineOptionChange(
        index: number,
        value: RoleOutlineOption | "any"
    ){
        if(value === "any") {
            this.props.onChange({
                type: "any",
            });
            return
        }

        if(this.props.roleOutline.type === "roleOutlineOptions") {
            let options = [...this.props.roleOutline.options];
            options[index] = value;
            this.props.onChange({
                type: "roleOutlineOptions",
                options: options
            });
        } else {
            this.props.onChange({
                type: "roleOutlineOptions",
                options: [value]
            });
        }
    }
    handleAddUnion() {
        if(this.props.roleOutline.type !== "roleOutlineOptions") {return}


        this.props.onChange({
            type: "roleOutlineOptions",
            options: [...this.props.roleOutline.options, {
                type: "role",
                role: "wildcard"
            }]
        });
        
    }

    render(): React.ReactNode {
        if(this.props.roleOutline.type === "any") {
            return <div className="role-picker">
                <RoleOutlineOptionSelector
                    disabled={this.props.disabled}
                    roleOutlineOption={"any"}
                    onChange={(o) => {
                        this.handleRoleOutlineOptionChange(0, o);
                    }}
                />
            </div>
        }else{
            return <div className="role-picker">
                {this.props.roleOutline.options.map((option, index) => {
                    return (
                        <div key={index} className="role-picker-option">
                            <RoleOutlineOptionSelector
                                disabled={this.props.disabled}
                                roleOutlineOption={option}
                                onChange={(o) => {
                                    this.handleRoleOutlineOptionChange(index, o);
                                }}
                            />
                            <button
                                disabled={this.props.disabled}
                                onClick={() => {
                                    if(this.props.roleOutline.type !== "roleOutlineOptions") {return}
                                    let options = [...this.props.roleOutline.options];
                                    options.splice(index, 1);
                                    if(options.length === 0) {
                                        this.props.onChange({
                                            type: "any",
                                        });
                                        return
                                    }
                                    this.props.onChange({
                                        type: "roleOutlineOptions",
                                        options: options
                                    });
                                }}
                            ><Icon size="tiny">remove</Icon></button>
                        </div>
                    )
                })}
                <button
                    disabled={this.props.disabled}
                    onClick={() => {
                        this.handleAddUnion();
                    }}
                ><Icon size="tiny">add</Icon></button>
            </div>
            
        }
        
    }
}

type RoleOutlineOptionSelectorProps = {
    disabled?: boolean
} & ({
    excludeAny: true
    roleOutlineOption: RoleOutlineOption,
    onChange: (value: RoleOutlineOption) => void,
} | {
    excludeAny?: false
    roleOutlineOption: RoleOutlineOption | "any",
    onChange: (value: RoleOutlineOption | "any") => void,
})

function translateRoleOutlineOptionOrAny(roleOutlineOption: RoleOutlineOption | "any"): string {
    if(roleOutlineOption === "any") {
        return translate("any");
    }else
        return translateRoleOutlineOption(roleOutlineOption);
}
export class RoleOutlineOptionSelector extends React.Component<RoleOutlineOptionSelectorProps> {

    
    render(): React.ReactNode {


        const optionsMap: SelectOptionsRecord<string> = {};

        optionsMap["any"] = [<StyledText
            noLinks={!this.props.disabled}
        >{translate("any")}</StyledText>, translate("any")];

        ROLE_SETS.forEach((roleSet) => {
            optionsMap[JSON.stringify({type: "roleSet", roleSet: roleSet})] = [<StyledText
                noLinks={!this.props.disabled}
            >{translateRoleOutlineOptionOrAny({type: "roleSet", roleSet: roleSet})}</StyledText>, translateRoleOutlineOptionOrAny({type: "roleSet", roleSet: roleSet})];
        });
        
        Object.keys(roleJsonData()).forEach((role) => {
            optionsMap[JSON.stringify({type: "role", role: role})] = [<StyledText
                noLinks={!this.props.disabled}
            >{translateRoleOutlineOptionOrAny({type: "role", role: role as Role})}</StyledText>, translateRoleOutlineOptionOrAny({type: "role", role: role as Role})];
        });

        return <Select
            className="role-outline-option-selector"
            disabled={this.props.disabled}
            value={this.props.roleOutlineOption==="any"?"any":JSON.stringify(this.props.roleOutlineOption)}
            onChange={(value) => {
                this.props.onChange(
                    value === "any" ? "any" : JSON.parse(value)
                );
            }}
            optionsSearch={optionsMap}
        />
    }
}

export function OutlineListSelector(props: {
    disabled?: boolean,
    onChangeRolePicker: (value: RoleOutline, index: number) => void,
    onAddNewOutline?: (() => void),
    onRemoveOutline?: ((index: number) => void),
    setRoleList: (newRoleList: RoleList) => void,
}) {
    const {roleList} = useContext(GameModeContext);

    const simplify = () => {
        props.setRoleList(roleList.map(simplifyRoleOutline));
    }

    return <section className="graveyard-menu-colors selector-section">
        <h2>{translate("menu.lobby.roleList")}: {roleList.length}</h2>
        <button disabled={props.disabled} onClick={simplify}>
            <Icon>filter_list</Icon> {translate("simplify")}
        </button>
        <div className="role-list-setter-list">
            <DragAndDrop 
                items={roleList}
                onDragEnd={props.setRoleList}
                disabled={props.disabled}
                render={(outline, index) => {
                    return <div key={index} className="role-list-setter-outline-div">
                        {props.disabled === true || <Icon>drag_indicator</Icon>}
                        <RoleOutlineSelector
                            disabled={props.disabled}
                            roleOutline={outline}
                            onChange={(value: RoleOutline) => {props.onChangeRolePicker(value, index);}}
                            key={index}
                        />
                        {props.onRemoveOutline ? 
                            <button disabled={props.disabled} onClick={() => {
                                if(props.onRemoveOutline)
                                    props.onRemoveOutline(index)
                        }}><Icon>delete</Icon></button> : null}
                    </div>
                }}
            />
            <div className="role-list-setter-outline-div role-list-setter-add-button-div">
                {props.onAddNewOutline ? 
                    <button disabled={props.disabled} onClick={props.onAddNewOutline}>
                        <Icon>add</Icon>
                    </button> : null}
            </div>
        </div>
    </section>
}

