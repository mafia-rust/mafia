import React, { ReactElement, useCallback, useEffect, useMemo, useState } from "react";
import translate from "../game/lang";
import "./wiki.css";
import { Role, getMainRoleSetFromRole } from "../game/roleState.d";
import GAME_MANAGER, { regEscape } from "..";
import WikiArticle, { getSearchStrings } from "./WikiArticle";
import { ARTICLES, WikiArticleLink, getArticleTitle } from "./WikiArticleLink";
import StyledText from "./StyledText";
import Icon from "./Icon";
import { ContentMenu, MenuController } from "../menu/game/GameScreen";
import { AnchorController } from "../menu/Anchor";
import WikiCoverCard from "./WikiCoverCard";
import { getAllRoles, RoleSet } from "../game/roleListState.d";
import { useLobbyOrGameState } from "./useHooks";


export function setWikiSearchPage(page: WikiArticleLink, anchorController: AnchorController, menuController?: MenuController) {
    if (GAME_MANAGER.wikiArticleCallbacks.length === 0) {
        if (menuController?.canOpen(ContentMenu.WikiMenu)) {
            menuController.openMenu(ContentMenu.WikiMenu, () => {
                GAME_MANAGER.setWikiArticle(page);
            });
        } else {
            anchorController.setCoverCard(<WikiCoverCard initialWikiPage={page}/>)
        }
    } else {
        GAME_MANAGER.setWikiArticle(page);
    }
}


export default function Wiki(props: Readonly<{
    enabledRoles?: Role[],
    initialWikiPage?: WikiArticleLink,
    onPageChange?: (page: WikiArticleLink | null) => void,
}>): ReactElement {

    const [searchQuery, setSearchQuery] = useState("");
    const [article, setArticle] = useState<WikiArticleLink | null>(props.initialWikiPage ?? null);
    const [history, setHistory] = useState<WikiArticleLink[]>([]);

    useEffect(() => {
        props.onPageChange && props.onPageChange(article);
    }, [article, props])

    const chooseArticle = useCallback((page: WikiArticleLink | null) => {
        if (page !== null) {
            if(history[history.length - 1] !== page)
                setHistory([...history, page]);
            if(history.length > 50)
                setHistory(history.slice(1));
            
        }
        setArticle(page);
    }, [history]);

    useEffect(() => {
        GAME_MANAGER.addSetWikiArticleCallback(chooseArticle);

        return () => GAME_MANAGER.removeSetWikiArticleCallback(chooseArticle);
    }, [setArticle, chooseArticle]);

    // This makes sure you can call GAME_MANAGER.setWikiArticle immediately after this component is added
    GAME_MANAGER.addSetWikiArticleCallback(chooseArticle);
    setTimeout(() => {
        GAME_MANAGER.removeSetWikiArticleCallback(chooseArticle);
    }, 100)

    function goBack() {
        if (history.length > 1) {
            let newHistory = [...history];
            newHistory.pop();
            setHistory(newHistory);
            setArticle(newHistory[newHistory.length - 1]);
        }else{
            setHistory([]);
            setArticle(null);
        }
    }

    return <div className="wiki-search">
        <WikiSearchBar 
            searchQuery={searchQuery}
            onSearchChange={setSearchQuery}
            onBack={goBack}
            onClear={() => {setSearchQuery(""); setArticle(null); setHistory([]);}}
        />
        {
            article === null ?
            <WikiSearchResults 
                searchQuery={searchQuery}
                onChooseArticle={chooseArticle}
            />
            :
            <WikiArticle article={article}/>
        }
    </div>
}

function WikiSearchBar(props: {
    searchQuery: string,
    onSearchChange: (search: string) => void,
    onBack: () => void,
    onClear: () => void,
}): ReactElement {
    return <div className="wiki-search-bar">
        <button tabIndex={-1} onClick={() => props.onBack()}>
            <Icon>arrow_back</Icon>
        </button>
        <input type="text" value={props.searchQuery}
            onChange={(e)=>{
                props.onSearchChange(e.target.value.trimStart())}
            }
            placeholder={translate("menu.wiki.search.placeholder")}
        />
        <button tabIndex={-1} onClick={props.onClear}>
            <Icon>close</Icon>
        </button>
    </div>
}

function WikiSearchResults(props: Readonly<{
    searchQuery: string,
    onChooseArticle: (article: WikiArticleLink) => void
}>): ReactElement {
    const enabledRoles = useLobbyOrGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"],
        getAllRoles()
    )!;

    const getSearchResults = useCallback((search: string) => {
        const out = [
            ...ARTICLES.filter((page) => {return RegExp(regEscape(search.trim()), 'i').test(getArticleTitle(page))}), 
            ...ARTICLES.filter((page) => {return getSearchStrings(page).some((str) => RegExp(regEscape(search.trim()), 'i').test(str))})
        ];
        return out
            .filter((item, index) => out.indexOf(item) === index)
            .sort((a, b) => wikiPageSortFunction(a, b));
    }, []);

    const results = useMemo(() => 
        getSearchResults(props.searchQuery),
    [props.searchQuery, getSearchResults])

    return <div className="wiki-results" tabIndex={-1}>
        {props.searchQuery === ""
            ? <WikiMainPage enabledRoles={enabledRoles} articles={results} onChooseArticle={props.onChooseArticle}/>
            : results.map(page => {
                let className = undefined;
                if(
                    page.includes("role/") &&
                    !enabledRoles.map(role => `role/${role}`).includes(page)
                ) {
                    className = "keyword-disabled";
                }
    
                return <WikiSearchResult key={page} 
                    page={page} 
                    className={className} 
                    onChooseArticle={() => props.onChooseArticle(page)}
                    />
            })}
    </div>
}

function WikiMainPage(props: Readonly<{
    articles: WikiArticleLink[],
    enabledRoles: Role[],
    onChooseArticle: (article: WikiArticleLink) => void
}>): ReactElement {
    const articlePartitions = useMemo(() => 
        partitionWikiPages(props.articles),
    [props.articles]);

    return <>
        {articlePartitions.roleSets.map(roleSetPartition => <>
            <h3 key={roleSetPartition.roleSet} className="wiki-search-divider">
                <StyledText>{translate(roleSetPartition.roleSet)}</StyledText>
            </h3>
            {roleSetPartition.pages.map(page => {
                const enabled = props.enabledRoles.map(role => `role/${role}`).includes(page);
                return <WikiSearchResult key={page} 
                    page={page} 
                    className={enabled ? "" : "keyword-disabled"} 
                    onChooseArticle={() => props.onChooseArticle(page)}
                />;
            })}
        </>)}
        <h3 key={"standard"} className="wiki-search-divider">
            <StyledText>{translate("standard")}</StyledText>
        </h3>
        <div className="alphabetized-articles">
            {articlePartitions.standard.map(letterPartition => <div key={letterPartition.letterCategory}>
                <span className="letter">{letterPartition.letterCategory}</span>
                {letterPartition.pages.map(page => 
                    <WikiSearchResult key={page} page={page} onChooseArticle={() => props.onChooseArticle(page)}/>
                )}
            </div>)}
        </div>
    </>
}

type WikiPagePartitions = {
    roleSets: {
        roleSet: RoleSet,
        pages: WikiArticleLink[]
    }[],
    standard: {
        letterCategory: string,
        pages: WikiArticleLink[]
    }[]
}

function partitionWikiPages(wikiPages: WikiArticleLink[]): WikiPagePartitions {
    const partitions: WikiPagePartitions = { roleSets: [], standard: [] };

    for (const wikiPage of wikiPages) {
        const articleType = wikiPage.split("/")[0];

        if (articleType === "role") {
            const role = wikiPage.split("/")[1] as Role;
            const roleSet = getMainRoleSetFromRole(role);

            const roleSetPartition = partitions.roleSets.find(p => p.roleSet === roleSet)
            if (roleSetPartition) {
                roleSetPartition.pages.push(wikiPage);
            } else {
                partitions.roleSets.push({ roleSet, pages: [wikiPage] });
            }
        } else {
            const title = getArticleTitle(wikiPage)
            const firstLetter = title.length === 0 ? "#" : title[0];
            const letterCategory = /[a-zA-Z]/.test(firstLetter) ? firstLetter : "#";
            
            const letterPartition = partitions.standard.find(p => p.letterCategory === letterCategory)
            if (letterPartition) {
                letterPartition.pages.push(wikiPage);
            } else {
                partitions.standard.push({ letterCategory, pages: [wikiPage] });
            }
        }
    }

    return partitions;
}

function wikiPageSortFunction(first: WikiArticleLink, second: WikiArticleLink): number {
    const firstRole = getRoleFromWikiPage(first);
    const secondRole = getRoleFromWikiPage(second);

    if (firstRole && secondRole) {
        return getAllRoles().indexOf(firstRole) - getAllRoles().indexOf(secondRole)
    } else if (firstRole) {
        return -1;
    } else if (secondRole) {
        return 1;
    } else {
        return getArticleTitle(first).localeCompare(getArticleTitle(second));
    }
}

function getRoleFromWikiPage(page: WikiArticleLink): Role | null {
    if (page.startsWith('role/')) {
        return page.substring(5) as Role;
    } else {
        return null;
    }
}

function WikiSearchResult(props: {
    page: WikiArticleLink,
    className?: string,
    onChooseArticle: (article: WikiArticleLink) => void
}): ReactElement {
    return <button key={props.page} onClick={() => props.onChooseArticle(props.page)}>
        <StyledText noLinks={true} className={props.className}>
            {getArticleTitle(props.page)}
        </StyledText>
</button>
}