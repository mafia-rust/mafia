import { GameBrowserState } from "./stateType/gameBrowserState";
import { GameState } from "./stateType/gameState";
import { LobbyState } from "./stateType/lobbyState";

export type State = {type: "disconnected"} | GameBrowserState | GameState | LobbyState;

