import React, { ReactElement, useEffect, useState } from 'react';
import translate from '../game/lang';
import GAME_MANAGER from '..';
import Wiki from '../components/Wiki';
import { Role } from '../game/roleState.d';
import { StateListener } from '../game/gameManager.d';
import "./wiki.css";

export default function WikiCoverCard(): ReactElement {
    let defaultDisabledRoles: Role[];
    switch (GAME_MANAGER.state.stateType) {
        case "disconnected":
        case "outsideLobby":
            defaultDisabledRoles = [];
            break;
        case "game":
        case "spectator":
        case "lobby":
            defaultDisabledRoles = GAME_MANAGER.state.excludedRoles;
            break;
    }
    const [disabledRoles, setDisabledRoles] = useState(defaultDisabledRoles);

    useEffect(() => {
        const listener: StateListener = (type) => {
            if (type === "excludedRoles" && (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby")) {
                setDisabledRoles(GAME_MANAGER.state.excludedRoles);
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, []);

    return <div className='wiki-cover-card'>
        <h1>{translate("menu.wiki.title")}</h1>
        <Wiki disabledRoles={disabledRoles} />
    </div>;
}
