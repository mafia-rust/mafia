import { ReactElement } from "react";
import "./lobbyChatMenu.css";
import translate from "../../game/lang";
import { ChatMessageSection, ChatTextInput } from "../game/gameScreenContent/ChatMenu";


export default function LobbyChatMenu(props: Readonly<{spectator: boolean}>): ReactElement {
    return <section className="lobby-chat-menu chat-menu-colors selector-section">
        <h2>{translate("menu.chat.title")}</h2>
        <div className="lobby-menu-chat">
            <ChatMessageSection/>
        </div>
        <ChatTextInput disabled={props.spectator}/>
    </section>
}