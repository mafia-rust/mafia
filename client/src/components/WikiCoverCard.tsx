import React, { ReactElement, useEffect, useState } from 'react';
import translate from '../game/lang';
import GAME_MANAGER from '..';
import Wiki from '../components/Wiki';
import { Role } from '../game/roleState.d';
import { StateListener } from '../game/gameManager.d';
import "./wiki.css";
import { WikiArticleLink } from './WikiArticleLink';

export default function WikiCoverCard(props: Readonly<{
    initialWikiPage?: WikiArticleLink
}>): ReactElement {
    let defaultEnabledRoles: Role[];
    switch (GAME_MANAGER.state.stateType) {
        case "disconnected":
        case "outsideLobby":
            defaultEnabledRoles = [];
            break;
        case "game":
        case "lobby":
            defaultEnabledRoles = GAME_MANAGER.state.enabledRoles;
            break;
    }
    const [enabledRoles, setEnabledRoles] = useState(defaultEnabledRoles);

    useEffect(() => {
        const listener: StateListener = (type) => {
            if (type === "enabledRoles" && (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby")) {
                setEnabledRoles(GAME_MANAGER.state.enabledRoles);
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, []);

    return <div className='wiki-cover-card'>
        <h1>{translate("menu.wiki.title")}</h1>
        <Wiki enabledRoles={enabledRoles} initialWikiPage={props.initialWikiPage}/>
    </div>;
}
