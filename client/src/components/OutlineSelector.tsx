import React, { useContext } from "react";
import "./outlineSelector.css";
import translate from "../game/lang";
import ROLES from "../resources/roles.json";
import { FACTIONS, ROLE_SETS, RoleList, RoleOutline, RoleOutlineOption, simplifyRoleOutline, translateRoleOutlineOption} from "../game/roleListState.d";
import { Role } from "../game/roleState.d";
import Icon from "./Icon";
import { DragAndDrop } from "./DragAndDrop";
import { GameModeContext } from "./GameModesEditor";

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

export class RoleOutlineOptionSelector extends React.Component<RoleOutlineOptionSelectorProps> {

    translateRoleOutlineOptionOrAny(roleOutlineOption: RoleOutlineOption | "any"): string {
        if(roleOutlineOption === "any") {
            return translate("any");
        }else
            return translateRoleOutlineOption(roleOutlineOption);
    }
    render(): React.ReactNode {
        return <select
            disabled={this.props.disabled}
            value={JSON.stringify(this.props.roleOutlineOption)} 
            onChange={(e) => {
                if(e.target.value === "any" && this.props.excludeAny !== true) {
                    this.props.onChange("any");
                } else {
                    this.props.onChange(
                        JSON.parse(e.target.options[e.target.selectedIndex].value)
                    );
                }
            }
        }>
            {this.props.excludeAny || <option key={"any"} value="any">
                {this.translateRoleOutlineOptionOrAny("any")}
            </option>}
            {FACTIONS.map((faction) => {
                return <option key={faction} value={JSON.stringify({type: "faction", faction: faction})}>
                    {this.translateRoleOutlineOptionOrAny({type: "faction", faction: faction})}
                </option>
            })}
            {ROLE_SETS.map((roleSet) => {
                return <option key={roleSet} value={JSON.stringify({type: "roleSet", roleSet: roleSet})}>
                        {this.translateRoleOutlineOptionOrAny({type: "roleSet", roleSet: roleSet})}
                </option>
            })}
            {Object.keys(ROLES).map((role) => {
                return <option key={role} value={JSON.stringify({type: "role", role: role})}>
                        {this.translateRoleOutlineOptionOrAny({type: "role", role: role as Role})}
                </option>
            })}
        </select>        
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
        <h2>{translate("menu.lobby.roleList")}</h2>
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

