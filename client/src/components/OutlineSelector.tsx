import React from "react";
import "./outlineSelector.css";
import translate from "../game/lang";
import ROLES from "../resources/roles.json";
import { FACTIONS, ROLE_SETS, RoleList, RoleOutline, RoleOutlineOption, simplifyRoleOutline, translateRoleOutlineOption} from "../game/roleListState.d";
import { Role } from "../game/roleState.d";

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

        let options = this.props.roleOutline.options;
        options.push({
            type: "role",
            role: "amnesiac"
        });

        this.props.onChange({
            type: "roleOutlineOptions",
            options: options
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
                            >
                                {translate("sub")}
                            </button>
                        </div>
                    )
                })}
                <button
                    disabled={this.props.disabled}
                    onClick={() => {
                        this.handleAddUnion();
                    }}
                >
                    {translate("add")}
                </button>
            </div>
            
        }
        
    }
}

type RoleOutlineOptionSelectorProps = {
    roleOutlineOption: RoleOutlineOption | "any",
    onChange: (value: RoleOutlineOption | "any") => void,
    disabled?: boolean,
}
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
                if(e.target.value === "any") {
                    this.props.onChange("any");
                } else {
                    this.props.onChange(
                        JSON.parse(e.target.options[e.target.selectedIndex].value)
                    );
                }
            }
        }>
            <option key={"any"} value="any">
                {this.translateRoleOutlineOptionOrAny("any")}
            </option>
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
    roleList: RoleList,
    onChangeRolePicker: (value: RoleOutline, index: number) => void,
    onAddNewOutline?: (() => void),
    onRemoveOutline?: ((index: number) => void),
}) {


    const simplify = () => {
        for(let i = 0; i < props.roleList.length; i++) {
            props.onChangeRolePicker(simplifyRoleOutline(props.roleList[i]), i);
        }
    }


    return <section className="graveyard-menu-colors selector-section">
        <h2>{translate("menu.lobby.roleList")}</h2>
        <button disabled={props.disabled} onClick={simplify}>
            {translate("simplify")}
        </button>
        {props.roleList.map((outline, index) => {
            return <div key={index} className="role-list-setter-outline-div">

                {props.onRemoveOutline ? 
                    <button disabled={props.disabled} onClick={() => {
                        if(props.onRemoveOutline)
                            props.onRemoveOutline(index)
                }}>{translate("sub")}</button> : null}
                
                <RoleOutlineSelector
                    disabled={props.disabled}
                    roleOutline={outline}
                    onChange={(value: RoleOutline) => {props.onChangeRolePicker(value, index);}}
                    key={index}
                />

            </div>
        })}
        {props.onAddNewOutline ? 
            <button disabled={props.disabled} onClick={props.onAddNewOutline}>
                {translate("add")}
            </button> : null}
    </section>
}

