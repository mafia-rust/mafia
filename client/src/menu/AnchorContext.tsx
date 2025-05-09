import { createContext, JSXElementConstructor, useCallback, useEffect, useMemo, useState } from "react";
import { ErrorCard, ErrorData } from "./Anchor";
import { Theme } from "..";
import LoadingScreen from "./LoadingScreen";
import React from "react";
import WikiCoverCard from "../components/WikiCoverCard";
import WikiArticle from "../components/WikiArticle";

export type AnchorContext = {
    reload: () => void,

    setContent: (content: JSX.Element) => void,
    contentType: string | JSXElementConstructor<any>,
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

export const AnchorContext = createContext<AnchorContext | undefined>(undefined);

export function useAnchorContext(){
    const [content, setContent] = useState<JSX.Element>(<LoadingScreen type="default"/>);
    const [setChildrenCallbacks, setSetContentCallbacks] = useState<(() => void)[]>([]);
    useEffect(() => {
        for (const callback of setChildrenCallbacks) {
            callback()
        }
        if (setChildrenCallbacks.length !== 0) {
            setSetContentCallbacks([])
        }
    }, [content, setChildrenCallbacks]);

    const [coverCard, setCoverCard] = useState<JSX.Element | null>(null);
    const [coverCardTheme, setCoverCardTheme] = useState<Theme | null>(null);
    const [setCoverCardCallbacks, setSetCoverCardCallbacks] = useState<(() => void)[]>([])
    useEffect(() => {
        for (const callback of setCoverCardCallbacks) {
            callback()
        }
        if (setCoverCardCallbacks.length !== 0) {
            setSetCoverCardCallbacks([])
        }
    }, [coverCard, setCoverCardCallbacks]);

    const [errorCard, setErrorCard] = useState<JSX.Element | null>(null);
    const [setErrorCardCallbacks, setSetErrorCardCallbacks] = useState<(() => void)[]>([])
    useEffect(() => {
        for (const callback of setErrorCardCallbacks) {
            callback()
        }
        if (setErrorCardCallbacks.length !== 0) {
            setSetErrorCardCallbacks([])
        }
    }, [errorCard, setErrorCardCallbacks]);

    const [globalMenuOpen, setGlobalMenuOpen] = useState<boolean>(false);

    const reload = useCallback(() => {
        setSetContentCallbacks(setChildrenCallbacks =>
            setChildrenCallbacks.concat(() => {
                setContent(() => content);
            }
        ));
        setContent(<LoadingScreen type="default"/>);

        setSetCoverCardCallbacks(setCoverCardCallbacks => 
            setCoverCardCallbacks.concat(() => {
                setCoverCard(() => coverCard)
            }
        ));
        setCoverCard(null);

        setSetErrorCardCallbacks(setErrorCardCallbacks =>
            setErrorCardCallbacks.concat(() => {
                setErrorCard(() => errorCard)
            })
        );
        setErrorCard(null);
    }, [content, coverCard, errorCard]);

    const anchorContext: AnchorContext = {
        reload,
        setContent,
        contentType: content.type,
        getCoverCard: () => {
            return coverCard;
        },
        setCoverCard: (coverCard: JSX.Element, callback?: () => void) => {
            let coverCardTheme: Theme | null = null;
            if (coverCard.type === WikiCoverCard || coverCard.type === WikiArticle) {
                coverCardTheme = "wiki-menu-colors";
            }

            if (callback) {
                setSetCoverCardCallbacks(setCoverCardCallbacks => setCoverCardCallbacks.concat(callback));
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
        content,
        coverCard,
        coverCardTheme,
        globalMenuOpen,
        errorCard,
    };

    return anchorContext;
}