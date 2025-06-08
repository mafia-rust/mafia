import React, { ReactElement } from 'react';
import translate from '../game/lang';
import Wiki from '../components/Wiki';
import "./wiki.css";
import { WikiArticleLink } from './WikiArticleLink';
import { MODIFIERS, ModifierType } from '../game/gameState.d';
import { getAllRoles } from '../stateContext/roleListState';
import { useLobbyOrGameState } from '../menu/lobby/LobbyContext';

export default function WikiCoverCard(props: Readonly<{
    initialWikiPage?: WikiArticleLink
}>): ReactElement {
    const enabledRoles = useLobbyOrGameState(state => state.enabledRoles)??getAllRoles();
    const enabledModifiers = useLobbyOrGameState(state => state.enabledModifiers)??MODIFIERS as any as ModifierType[];

    return <div className='wiki-cover-card'>
        <h1>{translate("menu.wiki.title")}</h1>
        <Wiki enabledRoles={enabledRoles} enabledModifiers={enabledModifiers} initialWikiPage={props.initialWikiPage}/>
    </div>;
}
