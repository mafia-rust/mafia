import { ReactElement, useState } from "react";
import { SavedRoleLists, loadRoleLists } from "../game/localStorage";
import React from "react";


export default function RoleListBank(): ReactElement {

    const [roleLists, setRoleLists] = useState<SavedRoleLists>(loadRoleLists() ?? new Map());


    return <div>
        <h1>ROLE LIST BANK</h1>
        <div>
            <h2>SAVED</h2>
            <ul>
                {Array.from(roleLists.keys()).map((roleListName) => {
                    return <li key={roleListName}>{roleListName}</li>
                })}
            </ul>
        </div>
    </div>

}