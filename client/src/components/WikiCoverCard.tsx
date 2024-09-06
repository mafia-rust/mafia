import React, { ReactElement } from 'react';
import translate from '../game/lang';
import Wiki from '../components/Wiki';
import "./wiki.css";
import { WikiArticleLink } from './WikiArticleLink';
import { useLobbyOrGameState } from './useHooks';

export default function WikiCoverCard(props: Readonly<{
    initialWikiPage?: WikiArticleLink
}>): ReactElement {
    const enabledRoles = useLobbyOrGameState(
        state => state.enabledRoles,
        ["enabledRoles"],
        []
    )!;

    return <div className='wiki-cover-card'>
        <h1>{translate("menu.wiki.title")}</h1>
        <Wiki enabledRoles={enabledRoles} initialWikiPage={props.initialWikiPage}/>
    </div>;
}
