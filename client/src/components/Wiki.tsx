import React, { ReactElement, useCallback, useEffect, useState } from "react";
import translate from "../game/lang";
import "./wikiSearch.css";
import { Role, getFactionFromRole } from "../game/roleState.d";
import GAME_MANAGER, { regEscape } from "..";
import WikiArticle, { getSearchStrings } from "./WikiArticle";
import { ARTICLES, WikiArticleLink, getArticleTitle } from "./WikiArticleLink";
import StyledText from "./StyledText";




export default function Wiki(props: {
    disabledRoles?: Role[]
}): ReactElement {

    const [searchQuery, setSearchQuery] = useState("");
    const [article, setArticle] = useState<WikiArticleLink | null>(null);
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
        GAME_MANAGER.setSetWikiArticleFunction(chooseArticle);
    }, [setArticle, chooseArticle]);
    GAME_MANAGER.setSetWikiArticleFunction(chooseArticle);

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
            onClear={() => {setSearchQuery(""); setArticle(null)}}
        />
        {
            article === null ?
            <WikiSearchResults 
                searchQuery={searchQuery}
                article={article}
                disabledRoles={props.disabledRoles}
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
        <button 
            tabIndex={-1}
            className="material-icons-round"
            onClick={() => {
                props.onBack();
            }}
        >
            arrow_back
        </button>
        <input type="text" value={props.searchQuery}
            onChange={(e)=>{props.onSearchChange(e.target.value)}}
            placeholder={translate("menu.wiki.search.placeholder")}
        />
        <button 
            tabIndex={-1}
            className="material-icons-round"
            onClick={props.onClear}
        >
            close
        </button>
    </div>
}

function WikiSearchResults(props: {
    searchQuery: string,
    article: WikiArticleLink | null,
    disabledRoles?: Role[],
    onChooseArticle: (article: WikiArticleLink) => void
}): ReactElement {

    function getSearchResults(search: string): WikiArticleLink[] {
        return ARTICLES.filter(
            (page) => {
                return RegExp(regEscape(search.trim()), 'i').test(getArticleTitle(page)) || 
                getSearchStrings(page).some((str) => RegExp(regEscape(search.trim()), 'i').test(str))
            }
        );
    }

    let results = getSearchResults(props.searchQuery);
    let elements = [];
    let standardHeaderAdded = false;
    let lastArticleRoleFaction = null;
    for(let page of results){

        let articleType = page.split("/")[0];

        if(!standardHeaderAdded && articleType === "standard"){
            elements.push(<h3 key={articleType} className="wiki-search-divider"><StyledText>{translate(articleType)}</StyledText></h3>);
            standardHeaderAdded = true;
        }

        if(articleType === "role"){
            const role = page.split("/")[1] as Role;
            const faction = getFactionFromRole(role);

            if(faction !== lastArticleRoleFaction){
                elements.push(<h3 key={faction} className="wiki-search-divider"><StyledText>{translate(faction)}</StyledText></h3>);
                lastArticleRoleFaction = faction;
            }
        }

        if(
            props.disabledRoles !== undefined && 
            props.disabledRoles.map((role)=>{return "role/"+role}).includes(page)
        ){
            elements.push(<button key={page} onClick={() => props.onChooseArticle(page)}><span className="keyword-dead">{getArticleTitle(page)}</span></button>);
        }else{
            elements.push(<button key={page} onClick={() => props.onChooseArticle(page)}><StyledText>{getArticleTitle(page)}</StyledText></button>);
        }
    }

    return <div className="wiki-results">
        {elements}
    </div>
}