import { ReactElement, useState } from "react";
import { SavedRoleLists, loadRoleLists, saveRoleLists } from "../game/localStorage";
import React from "react";
import { RoleListSetter } from "../components/RolePicker";
import { RoleList, RoleOutline } from "../game/roleListState.d";
import translate from "../game/lang";
import "./roleListBank.css";
import Anchor from "./Anchor";


export default function RoleListBank(): ReactElement {

    const [roleLists, setRoleLists] = useState<SavedRoleLists>(loadRoleLists() ?? new Map());

    const [currentRoleListName, setCurrentRoleListName] = useState<string>("");
    const [currentRoleList, setCurrentRoleList] = useState<RoleList>([]);


    const onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...currentRoleList];
        newRoleList[index] = value;
        setCurrentRoleList(newRoleList);
    }
    const addOutline = () => {
        setCurrentRoleList([...currentRoleList, {type: "any"}]);
    }
    const removeOutline = (index: number) => {
        let newRoleList = [...currentRoleList];
        newRoleList.splice(index, 1);
        setCurrentRoleList(newRoleList);
    }

    const saveRoleList = () => {
        let name = currentRoleListName;
        if(!name.match(/^[a-zA-Z0-9_ \-]+$/) || name.length >= 100 || name.length <= 0) return;
        if(currentRoleList.length === 0) return;
        if(roleLists.has(name) && !window.confirm(translate("confirmOverwrite"))) return;


        console.log("Saving role list: " + name);


        let newRoleLists = new Map(roleLists);
        newRoleLists.set(name, currentRoleList);
        setRoleLists(newRoleLists);
        saveRoleLists(newRoleLists);
    }
    const loadRoleList = (roleListName: string) => {
        setCurrentRoleListName(roleListName);
        setCurrentRoleList(roleLists.get(roleListName) ?? []);
    }
    const deleteRoleList = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return;

        let newRoleLists = new Map(roleLists);
        newRoleLists.delete(roleListName);
        setRoleLists(newRoleLists);
        saveRoleLists(newRoleLists);
    }


    
    

    
    return <div className="role-list-bank">
        <h1>{translate("menu.settings.gameSettingsBank")}</h1>
        <button className="material-icons-round close-button" onClick={()=>{Anchor.clearCoverCard()}}>close</button>
        {Array.from(roleLists.keys()).map((roleListName) => {
            return <section key={roleListName}>
                <button onClick={()=>{deleteRoleList(roleListName)}}>{translate("sub")}</button>
                <button onClick={()=>{loadRoleList(roleListName)}}>{roleListName}: {roleLists.get(roleListName)?.length}</button>
            </section>
        })}

        <div>
            <input type="text" value={currentRoleListName} onChange={(e) => {
                setCurrentRoleListName(e.target.value);
            }}/>
            <button onClick={saveRoleList} className="material-icons-round">save</button>
            

            <RoleListSetter
                disabled={false}
                roleList={currentRoleList}
                onChangeRolePicker={onChangeRolePicker}
                onAddNewOutline={addOutline}
                onRemoveOutline={removeOutline}
            />
        </div>
    </div>
}