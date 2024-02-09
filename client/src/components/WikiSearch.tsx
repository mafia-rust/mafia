import React from "react";
import translate from "../game/lang";
import "./wikiSearch.css";
import { Role } from "../game/roleState.d";
import StyledText from "../components/StyledText";
import { HistoryQueue } from "../history";
import { regEscape } from "..";
import WikiArticle, { ARTICLES, WikiArticleLink, getArticleTitle } from "./WikiArticle";

type WikiSearchProps = {
    page?: WikiArticleLink,
    excludedRoles?: Role[]
    pageChangeCallback?: (page: WikiArticleLink) => void
}

type WikiSearchState = ({
    type: "search",
} | {
    type: "page",
    page: WikiArticleLink,
}) & {
    searchQuery: string,
}

export default class WikiSearch extends React.Component<WikiSearchProps, WikiSearchState> {
    
    private static activeWikis: WikiSearch[] = [];
    history: HistoryQueue<WikiSearchState> = new HistoryQueue(20);

    constructor(props: WikiSearchProps) {
        super(props);

        if (props.page !== undefined) {
            this.history.push({
                type: "search",
                searchQuery: "",
            });
            this.state = {
                type: "page",
                searchQuery: "",
                page: props.page,
            }
        } else {
            this.state = {
                type: "search",
                searchQuery: "",
            };
        }
    }

    static setPage(page: WikiArticleLink) {
        WikiSearch.activeWikis.forEach(wiki => wiki.setPage(page));
    }

    componentDidMount() {
        WikiSearch.activeWikis.push(this);
    }
    componentWillUnmount() {
        WikiSearch.activeWikis.splice(WikiSearch.activeWikis.findIndex(wiki => wiki === this), 1);
    }

    setPage(page: WikiArticleLink) {
        if (this.state.type === "page" && this.state.page === page) {
            return;
        }
        this.history.push(this.state);
        this.setState({
            type: "page",
            searchQuery: this.state.searchQuery,
            page
        }, () => {
            if (this.props.pageChangeCallback !== undefined) {
                this.props.pageChangeCallback(page); 
            }
        });
    }

    renderOpenPageButton(page: WikiArticleLink) {

        let greyedOutRoles = this.props.excludedRoles
        if(greyedOutRoles === undefined){greyedOutRoles = [];}

        if(!greyedOutRoles.map((role)=>{return `role/${role}`}).includes(page)){
            return <button key={page} onClick={()=>{this.setPage(page)}}>
                <StyledText noLinks={true} markdown={true}>{getArticleTitle(page)}</StyledText>
            </button>
        }else{
            //TODO ill fix it says jack keyword-dead
            return <button key={page} onClick={()=>{this.setPage(page)}}>
                <span className="keyword-dead">{getArticleTitle(page)}</span>
            </button>
        }
    }

    getSearchResults(search: string): WikiArticleLink[] {
        return ARTICLES.filter(page => RegExp(regEscape(search), 'i').test(getArticleTitle(page)))
    }
    
    renderPageOrSearch(){
        if (this.state.type === "page") {
            return <WikiArticle article={this.state.page}/>
        } else {
            return this.getSearchResults(this.state.searchQuery)
                .map(this.renderOpenPageButton.bind(this));
        }
    }

    renderSearchBar() {
        return <div className="wiki-search-bar">
            {this.history.length() !== 0 ? 
                <button
                    className="material-icons-round"
                    onClick={() => this.setState(this.history.pop()!)}
                    aria-label={translate("menu.wiki.search.back")}
                >
                    arrow_back
                </button> 
            : null}
            <input type="text" value={this.state.searchQuery}
                onChange={(e)=>{
                    if (this.state.type === "page" || this.history.length() === 0) {
                        this.history.push(this.state);
                    }
                    this.setState({type: "search", searchQuery: e.target.value})
                }}
                placeholder={translate("menu.wiki.search.placeholder")}
            />
            {<button 
                tabIndex={-1}
                className="material-icons-round clear"
                onClick={() => {
                    this.history.push(this.state)
                    this.setState({ type: "search", searchQuery: "" });
                }}
                aria-label={translate("menu.wiki.search.clear")}
            >
                backspace
            </button>}
        </div>
    }
    
    render() {
        return (<div className="wiki-search">
        {this.renderSearchBar()}
        <div className="wiki-results">
            {this.renderPageOrSearch()}
        </div>
    </div>);}
}

