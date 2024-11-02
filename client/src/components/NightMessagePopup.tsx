import React, { ReactElement } from 'react';
import ChatElement, { ChatMessage } from './ChatMessage';
import translate from '../game/lang';
import "./nightMessagePopup.css";
import { Virtuoso } from 'react-virtuoso';

export default function NightMessagePopup(props: Readonly<{
    messages: ChatMessage[]
}>): ReactElement {

    return <div className="chat-menu chat-menu-colors night-message-popup">
        <h2>{translate("nightMessages")}</h2>
        <Virtuoso
            alignToBottom={true}
            totalCount={props.messages.length}
            followOutput={'smooth'}
            itemContent={(index) => <ChatElement
                key={index}
                message={props.messages[index]}
            />}
        />
    </div>
}
