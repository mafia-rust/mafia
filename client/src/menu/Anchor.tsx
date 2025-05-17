import React, { ReactElement, useRef, useContext, useState, useEffect, useCallback } from "react";
import "../index.css";
import "./anchor.css";
import { switchLanguage } from "../game/lang";
import GlobalMenu from "./GlobalMenu";
import { loadSettingsParsed } from "../game/localStorage";
import { Theme } from "..";
import Icon from "../components/Icon";
import { Button } from "../components/Button";
import { ChatMessage } from "../components/ChatMessage";
import AudioController from "./AudioController";
import { computeKeywordData } from "../components/StyledText";
import { AnchorContext, useAnchorContext } from "./AnchorContext";
import { MobileContext, useMobileContext } from "./MobileContext";

export default function Anchor(props: Readonly<{
    onMount: (anchorContext: AnchorContext) => void,
}>): ReactElement {

    const mobileContext = useMobileContext();
    const anchorContext = useAnchorContext();
    type TickData = {count: number, timeDelta: number}
    const [tickData, setTickData] = useState<TickData>({
        count: 0,
        timeDelta: 0
    });

    // Load settings
    useEffect(() => {
        const settings = loadSettingsParsed();

        AudioController.setVolume(settings.volume);
        anchorContext.setFontSize(settings.fontSize);
        anchorContext.setAccessibilityFontEnabled(settings.accessibilityFont);
        switchLanguage(settings.language);
        computeKeywordData();


        const TICK_TIME_DELTA = 1000;
        let tickInterval = setInterval(()=>{
            tickData.count += 1;
            tickData.timeDelta = TICK_TIME_DELTA;
            setTickData({...tickData});
        }, TICK_TIME_DELTA);

        return ()=>{
            clearInterval(tickInterval)
        }
    }, [])

    useEffect(() => {
        props.onMount(anchorContext);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [props])

    return <MobileContext.Provider value={mobileContext} >
        <AnchorContext.Provider value={anchorContext}>
            <div className="anchor">
                <Button className="global-menu-button" 
                    onClick={() => {
                        if (!anchorContext.globalMenuOpen) {
                            anchorContext.openGlobalMenu()
                        }else{
                            anchorContext.openGlobalMenu()
                        }
                    }}
                >
                    <Icon>menu</Icon>
                </Button>
                {anchorContext.globalMenuOpen && <GlobalMenu />}
                {anchorContext.content}
                {anchorContext.coverCard && <CoverCard
                    theme={anchorContext.coverCardTheme}
                >{anchorContext.coverCard}</CoverCard>}
                {anchorContext.errorCard}
            </div>
        </AnchorContext.Provider>
    </MobileContext.Provider>
}

function CoverCard(props: Readonly<{
    children: React.ReactNode,
    theme: Theme | null
}>): ReactElement {
    const ref = useRef<HTMLDivElement>(null);
    const anchorController = useContext(AnchorContext)!;

    const escFunction = useCallback((event: KeyboardEvent) =>{
        if(event.key === "Escape") {
            anchorController.clearCoverCard();
        }
    }, [anchorController]);
    useEffect(() => {
        document.addEventListener("keydown", escFunction, false);
        return () => {
            document.removeEventListener("keydown", escFunction, false);
        };
    }, [escFunction]);
    return <div 
        className={`anchor-cover-card-background-cover ${props.theme ?? ""}`} 
        onClick={e => {
            if (e.target === ref.current) anchorController.clearCoverCard()
        }}
        ref={ref}
    >
        <div className="anchor-cover-card">
            <Button className="close-button" onClick={anchorController.clearCoverCard}>
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