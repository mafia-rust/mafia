import React, { ReactElement } from "react";
import GAME_MANAGER from "../..";
import translate from "../../game/lang";
import Icon from "../../components/Icon";
import { useLobbyState } from "../../components/useHooks";
import { Button } from "../../components/Button";



export default function LobbyNamePane(): ReactElement {
    const isSpectator = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.clientType.type === "spectator",
        ["lobbyClients", "yourId"]
    )!;

    const ready = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.ready,
        ["lobbyClients", "playersHost", "playersReady", "yourId"]
    )!;

    return <section className="player-list-menu-colors selector-section lobby-name-pane">
        {!isSpectator && <NameSelector/>}
        <div className="name-pane-buttons">
            <Button onClick={() => GAME_MANAGER.sendSetSpectatorPacket(!isSpectator)}>
                {isSpectator
                    ? <><Icon>sports_esports</Icon> {translate("switchToPlayer")}</>
                    : <><Icon>visibility</Icon> {translate("switchToSpectator")}</>}
            </Button>
            {ready !== "host" && <Button
                onClick={() => {GAME_MANAGER.sendReadyUpPacket(ready === "notReady")}}
            >
                {ready === "ready"
                    ? <><Icon>clear</Icon> {translate("menu.lobby.button.unready")}</>
                    : <><Icon>check</Icon> {translate("menu.lobby.button.readyUp")}</>}
            </Button>}
        </div>
    </section>
}

function NameSelector(): ReactElement {
    const [enteredName, setEnteredName] = React.useState("");

    return <div className="name-pane-selector">
        <div className="lobby-name">
            <section><h2>{GAME_MANAGER.getMyName() ?? ""}</h2></section>
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
    </div>
}