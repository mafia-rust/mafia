import React, { ReactElement } from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { PlayerClientType } from "../../game/gameState.d";
import LobbyNamePane from "./LobbyNamePane";
import Icon from "../../components/Icon";
import { useLobbyState } from "../../components/useHooks";

export default function LobbyPlayerList(): ReactElement {
    const players = useLobbyState(
        lobbyState => lobbyState.players,
        ["playersHost", "playersLostConnection", "lobbyClients"]
    )!;
    const host = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.ready === "host",
        ["playersHost", "lobbyClients", "yourId"]
    )!;

    return <>
        <LobbyNamePane/>
        <section className="player-list player-list-menu-colors selector-section">
            <h2>{translate("menu.lobby.players")}</h2>
            <div className="list">
                <ol>
                    {[...players.entries()]
                        .filter(([_, player]) => player.clientType.type !== "spectator")
                        .map(([id, player]) => 
                            <li key={id} className={player.connection==="couldReconnect" ? "keyword-dead" : ""}>
                                <div>
                                    {player.connection === "couldReconnect" && <Icon>signal_cellular_connected_no_internet_4_bar</Icon>}
                                    {player.ready === "host" && <Icon>shield</Icon>}
                                    {player.ready === "ready" && <Icon>check</Icon>}
                                    {(player.clientType as PlayerClientType).name}
                                </div>
                                {host && <button 
                                    onClick={() => GAME_MANAGER.sendKickPlayerPacket(id)}
                                ><Icon>person_remove</Icon></button>}
                            </li>
                        )
                    }
                </ol>
            </div>
            <div className="spectators-ready">
                {translate("menu.lobby.spectatorsReady", 
                    [...players.values()].filter(p => p.clientType.type === "spectator" && p.ready !== "notReady").length,
                    [...players.values()].filter(p => p.clientType.type === "spectator").length
                )}
            </div>
        </section>
    </>
}