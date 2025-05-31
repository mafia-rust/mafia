import React from "react";
import { ARTICLES, WikiArticleLink } from "./components/WikiArticleLink";
import { deleteReconnectData, loadReconnectData } from "./game/localStorage";
import GAME_MANAGER from ".";
import GameModesEditor from "./components/gameModeSettings/GameModesEditor";
import parseFromJson from "./components/gameModeSettings/gameMode/dataFixer";
import { isFailure } from "./components/gameModeSettings/gameMode/parse";
import { AnchorContextType } from "./menu/AnchorContext";

function uriAsFileURI(path: string): string {
    if (path.endsWith('/')) {
        return path.substring(0, path.length - 1);
    } else {
        return path;
    }
}

async function routeWiki(anchorController: AnchorContextType, page: string) {
    const wikiPage = uriAsFileURI(page);

    if(wikiPage === "") {
        anchorController.setContent({
            type:"manual"
        });
    }else if(ARTICLES.includes(wikiPage.substring(1) as any)){
        anchorController.setContent({
            type:"manual",
            article: wikiPage.substring(1) as WikiArticleLink
        });
    }else{
        return await route404(anchorController, `/wiki${page}`);
    }
}

async function routeLobby(anchorController: AnchorContextType, roomCode: string) {
    const reconnectData = loadReconnectData();

    if (!await GAME_MANAGER.setOutsideLobbyState()) {
        anchorController.setContent({type:"main"});
        return;
    }
    
    window.history.replaceState({}, "", '/');

    let success: boolean;
    try {
        const code = parseInt(roomCode, 18)
        if (reconnectData) {
            success = await GAME_MANAGER.sendRejoinPacket(code, reconnectData.playerId);
            

            if(!success) {
                deleteReconnectData();
                success = await GAME_MANAGER.sendJoinPacket(code);
            }
        }else{
            success = await GAME_MANAGER.sendJoinPacket(code);
        }
    } catch {
        success = false;
    }
    
    if (!success) {
        await GAME_MANAGER.setDisconnectedState();
        anchorController.clearCoverCard();
        anchorController.setContent({type:"main"})
    }
}

async function routeGameMode(anchorController: AnchorContextType, gameModeString: string) {
    window.history.replaceState({}, "", "/");
    
    let gameMode: any;
    try {
        gameMode = JSON.parse(gameModeString);
    } catch {
        return await route404(anchorController, `/gameMode/?mode=${gameModeString}`);
    }

    const verifiedGameMode = parseFromJson("ShareableGameMode", gameMode);

    if (isFailure(verifiedGameMode)) {
        console.log(verifiedGameMode.reason);
        console.log(verifiedGameMode.snippet);
        return await route404(anchorController, `/gameMode/?mode=${gameModeString}`);
    } else {
        anchorController.setContent({type:"main"})
        anchorController.setCoverCard(<GameModesEditor initialGameMode={verifiedGameMode.value}/>)
    }
}

async function route404(anchorController: AnchorContextType, path: string) {
    anchorController.setContent(
        <div className="hero" style={{ display: "flex", alignItems: "center", justifyContent: "center", gap: "1rem" }}>
            <h1>404</h1>
            <p>The requested path ({path}) could not be found</p>
        </div>
    )
}

async function routeMainButFirstTryUsingReconnectData(anchorController: AnchorContextType) {
    window.history.replaceState({}, "", "/");

    const reconnectData = loadReconnectData();
    
    if (!reconnectData) {
        anchorController.setContent({type:"main"})
        return;
    }

    if (!await GAME_MANAGER.setOutsideLobbyState()) {
        anchorController.setContent({type:"main"});
        return;
    }

    if (!await GAME_MANAGER.sendRejoinPacket(reconnectData.roomCode, reconnectData.playerId)) {
        anchorController.setContent({type:"main"});
        deleteReconnectData();
        return;
    }

    // This is where we *should* handle joining the lobby, but it's handled in messageListener... grumble grumble
}

export default async function route(anchorController: AnchorContextType, url: Location) {

    if (url.pathname.startsWith("/wiki")) {
        return await routeWiki(anchorController, url.pathname.substring(5));
    } else if (url.pathname.startsWith("/connect")) {
        const roomCode = new URLSearchParams(url.search).get("code");
        if (roomCode !== null) {
            return await routeLobby(anchorController, roomCode);
        }
    } else if (url.pathname.startsWith("/gameMode")) {
        const gameMode = new URLSearchParams(url.search).get("mode");
        if (gameMode !== null) {
            return await routeGameMode(anchorController, gameMode);
        }
    }

    

    if (url.pathname && url.pathname !== '/') {
        return await route404(anchorController, url.pathname);
    }
    
    return await routeMainButFirstTryUsingReconnectData(anchorController);
}