import React, {
    ReactElement, useRef, useContext,
    useEffect, useCallback
} from "react";
import "../index.css";
import "./app.css";
import { switchLanguage } from "../game/lang";
import GlobalMenu from "./GlobalMenu";
import { loadSettingsParsed } from "../game/localStorage";
import { Theme } from "..";
import Icon from "../components/Icon";
import { Button } from "../components/Button";
import { ChatMessage } from "../components/ChatMessage";
import AudioController from "./AudioController";
import { computeKeywordData } from "../components/StyledText";
import AppContextProvider, { AppContext, AppContextType } from "./AppContext";
import MobileContextProvider from "./MobileContext";
import WebsocketContextProvider, { WebsocketContext, WebSocketContextType } from "./WebsocketContext";

export default function App(props: Readonly<{
    onMount: (appContext: AppContextType, websocketContext: WebSocketContextType) => void,
}>): ReactElement {
    return <MobileContextProvider>
        <WebsocketContextProvider>
            <AppContextProvider>
                <AppInner {...props}/>
            </AppContextProvider>
        </WebsocketContextProvider>
    </MobileContextProvider>
}

function AppInner(props: Readonly<{
    onMount: (appContext: AppContextType, websocketContext: WebSocketContextType) => void,
}>): ReactElement {
    const appContext = useContext(AppContext)!;
    const websocketContext = useContext(WebsocketContext)!;

    // Load settings
    useEffect(() => {
        const settings = loadSettingsParsed();

        AudioController.setVolume(settings.volume);
        appContext.setFontSize(settings.fontSize);
        appContext.setAccessibilityFontEnabled(settings.accessibilityFont);
        switchLanguage(settings.language);
        computeKeywordData();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    useEffect(() => {
        props.onMount(appContext, websocketContext);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [props])

    return <div className="anchor">
        <Button className="global-menu-button" 
            onClick={() => {
                if (!appContext.globalMenuOpen) {
                    appContext.openGlobalMenu()
                }else{
                    appContext.openGlobalMenu()
                }
            }}
        >
            <Icon>menu</Icon>
        </Button>
        {appContext.globalMenuOpen && <GlobalMenu />}
        {appContext.content}
        {appContext.coverCard && <CoverCard
            theme={appContext.coverCardTheme}
        >{appContext.coverCard}</CoverCard>}
        {appContext.errorCard}
    </div>
}

function CoverCard(props: Readonly<{
    children: React.ReactNode,
    theme: Theme | null
}>): ReactElement {
    const ref = useRef<HTMLDivElement>(null);
    const appController = useContext(AppContext)!;

    const escFunction = useCallback((event: KeyboardEvent) =>{
        if(event.key === "Escape") {
            appController.clearCoverCard();
        }
    }, [appController]);
    useEffect(() => {
        document.addEventListener("keydown", escFunction, false);
        return () => {
            document.removeEventListener("keydown", escFunction, false);
        };
    }, [escFunction]);
    return <div 
        className={`anchor-cover-card-background-cover ${props.theme ?? ""}`} 
        onClick={e => {
            if (e.target === ref.current) appController.clearCoverCard()
        }}
        ref={ref}
    >
        <div className="anchor-cover-card">
            <Button className="close-button" onClick={appController.clearCoverCard}>
                <Icon>close</Icon>
            </Button>
            <div className="anchor-cover-card-content">
                {props.children}
            </div>
        </div>
    </div>
}

export type ErrorData = {
    title: string,
    body: string
}

export function ErrorCard(props: Readonly<{
    error: ErrorData,
    onClose: () => void
}>) {
    return <div className="error-card" onClick={() => props.onClose()}>
        <header>
            {props.error.title}
            <button className="close">✕</button>
        </header>
        <div>{props.error.body}</div>
    </div>
}


export type AudioFile = 
    "church_bell.mp3" | 
    "alarm.mp3" | 
    "vine_boom.mp3" | 
    "sniper_shot.mp3" | 
    "normal_message.mp3" | 
    "whisper_broadcast.mp3" | 
    "start_game.mp3";

export type AudioFilePath = `audio/${AudioFile}`;

export function chatMessageToAudio(msg: ChatMessage): AudioFilePath | null {
    let file: AudioFile|null = null;

    switch(msg.variant.type){
        case "normal":
        case "voted":
            file = "normal_message.mp3";
            break;
        case "broadcastWhisper":
            file = "whisper_broadcast.mp3";
            break;
        case "playerDied": 
            file = "church_bell.mp3";
            break;
        case "deputyKilled": 
            file = "sniper_shot.mp3";
            break;
    }

    if(file){
        return `audio/${file}`;
    }else{
        return null
    }
}