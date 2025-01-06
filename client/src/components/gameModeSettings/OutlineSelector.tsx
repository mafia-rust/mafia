import React, { ReactElement, useCallback, useContext } from "react";
import "./outlineSelector.css";
import translate from "../../game/lang";
import { getAllRoles, getRolesFromRoleSet, ROLE_SETS, RoleList, RoleOutline, RoleOutlineOption, simplifyRoleOutline, translateRoleOutline, translateRoleOutlineOption} from "../../game/roleListState.d";
import { Role } from "../../game/roleState.d";
import Icon from "../Icon";
import { DragAndDrop } from "../DragAndDrop";
import { GameModeContext } from "./GameModesEditor";
import Select, { SelectOptionsSearch } from "../Select";
import StyledText from "../StyledText";
import { Button } from "../Button";
import { useLobbyOrGameState } from "../useHooks";

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

function translateRoleOutlineOptionOrAny(roleOutlineOption: RoleOutlineOption | "any"): string {
    if(roleOutlineOption === "any") {
        return translate("any");
    }else
        return translateRoleOutlineOption(roleOutlineOption);
}
export function RoleOutlineOptionSelector(props: Readonly<{
    disabled?: boolean
} & ({
    excludeAny: true
    roleOutlineOption: RoleOutlineOption,
    onChange: (value: RoleOutlineOption) => void,
} | {
    excludeAny?: false
    roleOutlineOption: RoleOutlineOption | "any",
    onChange: (value: RoleOutlineOption | "any") => void,
})>): ReactElement {
    const enabledRoles = useLobbyOrGameState(
        state => state.enabledRoles,
        ["enabledRoles"],
        getAllRoles()
    )!;

    const isRoleEnabled = useCallback((role: Role) => {
        return enabledRoles.includes(role)
    }, [enabledRoles])

    const optionsSearch: SelectOptionsSearch<string> = new Map();

    optionsSearch.set("any", [
        <StyledText
            key={0}
            noLinks={!props.disabled}
        >
            {translate("any")}
        </StyledText>, 
        translate("any")
    ]);

    ROLE_SETS.forEach((roleSet) => {
        optionsSearch.set(JSON.stringify({type: "roleSet", roleSet: roleSet}), [
            <StyledText
                key={0}
                noLinks={!props.disabled}
                className={getRolesFromRoleSet(roleSet).every(role => !isRoleEnabled(role)) ? "keyword-disabled" : ""}
            >
                {translateRoleOutlineOptionOrAny({type: "roleSet", roleSet: roleSet})}
            </StyledText>, 
            translateRoleOutlineOptionOrAny({type: "roleSet", roleSet: roleSet})]
        );
    });
    
    getAllRoles().forEach((role) => {
        optionsSearch.set(JSON.stringify({type: "role", role: role}), [
            <StyledText
                key={0}
                noLinks={!props.disabled}
                className={!isRoleEnabled(role) ? "keyword-disabled" : ""}
            >
                {translateRoleOutlineOptionOrAny({type: "role", role})}
            </StyledText>,
            translateRoleOutlineOptionOrAny({type: "role", role})
        ]);
    });

    return <Select
        className="role-outline-option-selector"
        disabled={props.disabled}
        value={props.roleOutlineOption==="any"?"any":JSON.stringify(props.roleOutlineOption)}
        onChange={(value) => {
            props.onChange(
                value === "any" ? "any" : JSON.parse(value)
            );
        }}
        optionsSearch={optionsSearch}
    />
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
        {(props.disabled !== true) && <Button onClick={simplify}>
            <Icon>filter_list</Icon> {translate("simplify")}
        </Button>}
        <div className="role-list-setter-list">
            <DragAndDrop 
                items={structuredClone(roleList)}
                onDragEnd={props.setRoleList}
                disabled={props.disabled}
                render={(outline, index) => {
                    return <div key={index} className="role-list-setter-outline-div">
                        {props.disabled === true || <Icon>drag_indicator</Icon>}
                        {props.disabled === true
                            ? <div className="placard">
                                <StyledText>
                                    {translateRoleOutline(outline)}
                                </StyledText>
                            </div>
                            : <RoleOutlineSelector
                                disabled={props.disabled}
                                roleOutline={outline}
                                onChange={(value: RoleOutline) => {props.onChangeRolePicker(value, index);}}
                                key={index}
                            />
                        }
                        {props.onRemoveOutline &&
                            <button disabled={props.disabled} onClick={() => {
                                if(props.onRemoveOutline)
                                    props.onRemoveOutline(index)
                        }}><Icon>delete</Icon></button>}
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

