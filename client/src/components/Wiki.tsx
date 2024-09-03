import React, { ReactElement, useCallback, useEffect, useState } from "react";
import translate from "../game/lang";
import "./wiki.css";
import { Role, getFactionFromRole } from "../game/roleState.d";
import GAME_MANAGER, { regEscape } from "..";
import WikiArticle, { getSearchStrings } from "./WikiArticle";
import { ARTICLES, WikiArticleLink, getArticleTitle } from "./WikiArticleLink";
import StyledText from "./StyledText";
import Icon from "./Icon";




export default function Wiki(props: {
    enabledRoles?: Role[],
    initialWikiPage?: WikiArticleLink
}): ReactElement {

    const [searchQuery, setSearchQuery] = useState("");
    const [article, setArticle] = useState<WikiArticleLink | null>(props.initialWikiPage ?? null);
    const [history, setHistory] = useState<WikiArticleLink[]>([]);

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
                article={article}
                enabledRoles={props.enabledRoles}
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

function WikiSearchResults(props: {
    searchQuery: string,
    article: WikiArticleLink | null,
    enabledRoles?: Role[],
    onChooseArticle: (article: WikiArticleLink) => void
}): ReactElement {

    function getSearchResults(search: string): WikiArticleLink[] {
        const out = [
            ...ARTICLES.filter((page) => {return RegExp(regEscape(search.trim()), 'i').test(getArticleTitle(page))}), 
            ...ARTICLES.filter((page) => {return getSearchStrings(page).some((str) => RegExp(regEscape(search.trim()), 'i').test(str))})
        ];
        return out.filter((item, index) => out.indexOf(item) === index);
    }

    let searchResultsHtml = [];
    const results = getSearchResults(props.searchQuery);
    if (props.searchQuery === ""){
        let standardHeaderAdded = false;
        let lastArticleRoleFaction = null;
        for(let page of results){

            let articleType = page.split("/")[0];

            if(!standardHeaderAdded && articleType === "standard"){
                searchResultsHtml.push(<h3 key={articleType} className="wiki-search-divider"><StyledText>{translate(articleType)}</StyledText></h3>);
                standardHeaderAdded = true;
            }

            if(articleType === "role"){
                const role = page.split("/")[1] as Role;
                const faction = getFactionFromRole(role);

                if(faction !== lastArticleRoleFaction){
                    searchResultsHtml.push(<h3 key={faction} className="wiki-search-divider"><StyledText>{translate(faction)}</StyledText></h3>);
                    lastArticleRoleFaction = faction;
                }
            }

            let className = undefined;
            if(
                page.includes("role/") &&
                props.enabledRoles !== undefined && props.enabledRoles.length !== 0 && !props.enabledRoles.map(role => `role/${role}`).includes(page)
            ) {
                className = "keyword-disabled";
            }

            searchResultsHtml.push(
                <WikiSearchResult key={page} page={page} className={className} onChooseArticle={() => props.onChooseArticle(page)}/>
            );
        }
    }else{
        for(let page of results){

            let className = undefined;
            if(
                page.includes("role/") &&
                props.enabledRoles !== undefined && props.enabledRoles.length !== 0 && !props.enabledRoles.map(role => `role/${role}`).includes(page)
            ) {
                className = "keyword-disabled";
            }

            searchResultsHtml.push(
                <WikiSearchResult key={page} page={page} className={className} onChooseArticle={() => props.onChooseArticle(page)}/>
            );
        }
    }

    

    return <div className="wiki-results" tabIndex={-1}>
        {searchResultsHtml}
    </div>
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