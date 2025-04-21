import { ReactElement } from 'react';
import ChatElement, { ChatMessage } from './ChatMessage';
import translate from '../game/lang';
import "./nightMessagePopup.css";

export default function NightMessagePopup(props: Readonly<{
    messages: ChatMessage[]
}>): ReactElement {
    return <div className="chat-menu chat-menu-colors night-message-popup">
        <h2>{translate("nightMessages")}</h2>
        <div className="chat-message-section">
            <div className="chat-message-list">
                {props.messages.map((msg, index) => {
                    return <ChatElement
                        key={index}
                        message={msg}
                    />;
                })}
            </div>
        </div>
    </div>
}
