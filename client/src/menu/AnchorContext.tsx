import { createContext, useState } from "react";
import { ErrorCard, ErrorData } from "./Anchor";
import { Theme } from "..";
import React from "react";
import WikiCoverCard from "../components/WikiCoverCard";
import WikiArticle from "../components/WikiArticle";
import WebsocketComponent from "./WebsocketComponent";
import StandaloneWiki from "./main/StandaloneWiki";
import { WikiArticleLink } from "../components/WikiArticleLink";
import StartMenu from "./main/StartMenu";
import Credits from "./main/Credits";

type AnchorContentType = {
    type: "main"
}|{
    type: "manual",
    article?: WikiArticleLink
}|{
    type:"connect"
}|{
    type:"404",
    path: string
}|{
    type:"credits",
};

export type AnchorContextType = {
    setContent: (content: AnchorContentType) => void,
    
    contentType: AnchorContentType,
    content: JSX.Element | null,

    getCoverCard: () => JSX.Element | null,
    setCoverCard: (content: JSX.Element) => void,
    clearCoverCard: () => void,
    coverCard: JSX.Element | null,
    coverCardTheme: Theme | null;

    pushErrorCard: (error: ErrorData) => void,
    errorCard: JSX.Element | null,

    openGlobalMenu: () => void,
    closeGlobalMenu: () => void,
    globalMenuOpen: boolean,

    setFontSize: (fontSize: number) => void,
    setAccessibilityFontEnabled: (accessibilityFontEnabled: boolean) => void
}

export const AnchorContext = createContext<AnchorContextType | undefined>(undefined);

export function useAnchorContext(){
    const [content, setContent] = useState<JSX.Element>(<StartMenu/>);
    const [contentType, setContentType] = useState<AnchorContentType>({type: "main"});

    const [coverCard, setCoverCard] = useState<JSX.Element | null>(null);
    const [coverCardTheme, setCoverCardTheme] = useState<Theme | null>(null);

    const [errorCard, setErrorCard] = useState<JSX.Element | null>(null);

    const [globalMenuOpen, setGlobalMenuOpen] = useState<boolean>(false);

    const anchorContext: AnchorContextType = {
        content,
        contentType,
        coverCard,
        coverCardTheme,
        globalMenuOpen,
        errorCard,

        setContent: (contentType)=>{
            switch(contentType.type){
                case "main":
                    setContent(<StartMenu/>);
                break;
                case "connect":
                    setContent(<WebsocketComponent/>);
                break;
                case "manual":
                    if(contentType.article !== undefined){
                        setContent(<StandaloneWiki initialWikiPage={contentType.article}/>);
                    }else{
                        setContent(<StandaloneWiki/>);
                    }
                break;
                case "404":
                    setContent(<div className="hero" style={{ display: "flex", alignItems: "center", justifyContent: "center", gap: "1rem" }}>
                        <h1>404</h1>
                        <p>The requested path ({contentType.path}) could not be found</p>
                    </div>);
                break;
                case "credits":
                    setContent(<Credits/>);
                break;
            }
            setContentType(contentType);
        },
        getCoverCard: () => {
            return coverCard;
        },
        setCoverCard: (coverCard: JSX.Element, callback?: () => void) => {
            let coverCardTheme: Theme | null = null;
            if (coverCard.type === WikiCoverCard || coverCard.type === WikiArticle) {
                coverCardTheme = "wiki-menu-colors";
            }

            setCoverCard(coverCard);
            setCoverCardTheme(coverCardTheme);
        },
        pushErrorCard: (error: ErrorData) => {
            setErrorCard(
                <ErrorCard
                    onClose={() => setErrorCard(null)}
                    error={error}
                />
            );
        },
        clearCoverCard: () => {
            setCoverCard(null);
            setCoverCardTheme(null);
        },
        openGlobalMenu: () => setGlobalMenuOpen(true),
        closeGlobalMenu: () => setGlobalMenuOpen(false),
        setFontSize: (fontSize: number) => {
            document.documentElement.style.fontSize = `${fontSize}em`;
        },
        setAccessibilityFontEnabled: (enabled: boolean) => {
            const getFont = (font: string, enabled: boolean) => enabled === true ? 'game-accessible-font' : font;

            const iconFactor = enabled ? '1.2' : '1';

            document.documentElement.style.setProperty('--game-font', getFont('game-base-font', enabled));
            document.documentElement.style.setProperty('--kira-font', getFont('game-kira-font', enabled));
            document.documentElement.style.setProperty('--spiral-font', getFont('game-spiral-font', enabled));
            document.documentElement.style.setProperty('--title-font', getFont('game-title-font', enabled));
            document.documentElement.style.setProperty('--computer-font', getFont('computer-font', enabled));
            document.documentElement.style.setProperty('--legible-computer-font', getFont('legible-computer-font', enabled));
            document.documentElement.style.setProperty('--icon-factor', iconFactor);
        },
    };

    return anchorContext;
}