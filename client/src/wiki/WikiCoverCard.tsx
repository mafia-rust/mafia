import React, { ReactElement } from 'react';
import translate from '../game/lang';
import Wiki from './Wiki';
import "./wiki.css";
import { WikiArticleLink } from './WikiArticleLink';
import { getAllRoles } from '../stateContext/stateType/roleListState';
import { useLobbyOrGameState } from '../stateContext/useHooks';
import { MODIFIERS, ModifierType } from '../stateContext/stateType/modifiersState';

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
