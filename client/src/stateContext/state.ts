import { GameBrowserState } from "./gameBrowserState";
import { GameState } from "./gameState";
import { LobbyState } from "./lobbyState";

export type State = {type: "disconnected"} | GameBrowserState | GameState | LobbyState;

