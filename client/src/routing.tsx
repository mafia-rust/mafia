import React from "react";
import { ARTICLES, WikiArticleLink } from "./components/WikiArticleLink";
import { AnchorController } from "./menu/Anchor";
import StandaloneWiki from "./menu/main/StandaloneWiki";
import { deleteReconnectData, loadReconnectData } from "./game/localStorage";
import GAME_MANAGER from ".";
import StartMenu from "./menu/main/StartMenu";

async function routeWiki(anchorController: AnchorController, wikiPage: string) {
    window.history.replaceState({}, "", `/wiki${wikiPage}`)

    if (wikiPage === "/") {
        anchorController.setContent(<StandaloneWiki />)
    } else if (ARTICLES.includes(wikiPage.substring(1) as any)) {
        anchorController.setContent(<StandaloneWiki initialWikiPage={wikiPage.substring(1) as WikiArticleLink}/>)
    } else {
        return await routeMain(anchorController);
    }
}

async function routeLobby(anchorController: AnchorController, roomCode: string) {
    const reconnectData = loadReconnectData();

    await GAME_MANAGER.setOutsideLobbyState();
    
    window.history.replaceState({}, "", window.location.pathname);

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
        anchorController.setContent(<StartMenu/>)
    }
}

async function routeMain(anchorController: AnchorController) {
    window.history.replaceState({}, "", window.location.pathname);

    const reconnectData = loadReconnectData();
    
    if (reconnectData) {
        await GAME_MANAGER.setOutsideLobbyState();

        const success = await GAME_MANAGER.sendRejoinPacket(reconnectData.roomCode, reconnectData.playerId);
        if (!success) {
            // Don't show an error message for an auto-rejoin. The user didn't prompt it - they will be confused.
            // Reconnect data is deleted in messageListener
            await GAME_MANAGER.setDisconnectedState();
            anchorController.clearCoverCard();
            anchorController.setContent(<StartMenu/>);
        }
    } else {
        anchorController.setContent(<StartMenu/>)
    }
}

export default async function route(anchorController: AnchorController, url: Location) {
    if (url.pathname.startsWith("/wiki")) {
        return await routeWiki(anchorController, url.pathname.substring(5));
    }

    const roomCode = new URLSearchParams(url.search).get("code");
    if (roomCode !== null) {
        return await routeLobby(anchorController, roomCode);
    }
    
    return await routeMain(anchorController);
}