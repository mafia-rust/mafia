import React, { ReactElement } from "react";
import GAME_MANAGER from "../..";
import translate from "../../game/lang";
import Icon from "../../components/Icon";
import { useLobbyState } from "../../components/useHooks";



export default function LobbyNamePane(): ReactElement {
    const isSpectator = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.clientType.type === "spectator",
        ["lobbyClients", "yourId"]
    )!;

    return <section className="player-list-menu-colors selector-section">
        {!isSpectator && <NameSelector/>}
        {isSpectator && <button
            onClick={()=>{GAME_MANAGER.sendSetSpectatorPacket(false)}}
        ><Icon>sports_esports</Icon> {translate("switchToPlayer")}</button>}
    </section>
}

function NameSelector(): ReactElement {
    const [enteredName, setEnteredName] = React.useState("");

    return <>
        <div className="lobby-name">
            <section><h2>{GAME_MANAGER.getMyName() ?? ""}</h2></section>
            <button
                onClick={()=>{GAME_MANAGER.sendSetSpectatorPacket(true)}}
            ><Icon>visibility</Icon> {translate("switchToSpectator")}</button>
        </div>
        <div className="name-box">
            <input type="text" value={enteredName}
                onChange={(e)=>{setEnteredName(e.target.value)}}
                placeholder={translate("menu.lobby.field.namePlaceholder")}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        GAME_MANAGER.sendSetNamePacket(enteredName);
                }}
            />
            <button onClick={()=>{
                GAME_MANAGER.sendSetNamePacket(enteredName)
            }}>{translate("menu.lobby.button.setName")}</button>
        </div>
    </>
}