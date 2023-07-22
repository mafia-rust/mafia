import React from "react";
import ROLES from "./../resources/roles.json";
import translate, { langText } from "../game/lang";
import "./wikiSearch.css";
import { Role } from "../game/roleState.d";
import { FactionAlignment, getRoleListEntryFromFactionAlignment, translateRoleListEntry } from "../game/roleListState.d";
import StyledText from "../components/StyledText";
import { HistoryQueue } from "../history";
import { regEscape } from "..";

type WikiSearchProps = {
    setPageController?: (pageController: (page: WikiPage) => void) => void
}

type WikiSearchState = ({
    type: "search"
} | {
    type: "page",
    page: WikiPage,
}) & {
    searchQuery: string,
}

export type WikiPage = 
    | `role/${Role}`
    | `article/${Article}`;

const ARTICLES = ["help", "roles_and_teams", "phases_and_timeline", "controls", "wills_and_notes", "visit", "all_language"] as const;
type Article = typeof ARTICLES[number];

const PAGES: WikiPage[] = Object.keys(ROLES).map(role => `role/${role}`)
    .concat(ARTICLES.map(article => `article/${article}`)) as WikiPage[];

export default class WikiSearch extends React.Component<WikiSearchProps, WikiSearchState> {
    history: HistoryQueue<WikiSearchState> = new HistoryQueue(10);
    constructor(props: WikiSearchProps) {
        super(props);

        // Let parent components change the wiki page
        if (this.props.setPageController !== undefined) {
            this.props.setPageController(this.setPage.bind(this));
        }

        this.state = {
            type: "search",
            searchQuery: "",
        };
    }

    setPage(page: WikiPage) {
        this.history.push(this.state);
        this.setState({
            type: "page",
            searchQuery: this.state.searchQuery,
            page
        });
    }

    renderOpenPageButton(page: WikiPage) {
        return <button key={page} onClick={()=>{this.setPage(page)}}>
            <StyledText>{getPageTitle(page)}</StyledText>
        </button>
    }

    getSearchResults(search: string): WikiPage[] {
        return PAGES.filter(page => RegExp(regEscape(search), 'i').test(getPageText(page)))
    }
    
    renderPageOrSearch(){
        if (this.state.type === "page") {
            if (this.state.page === "article/all_language") {
                return langText;
            } else {
                return <StyledText className="wiki-content-body">
                    {getPageText(this.state.page)}
                </StyledText>;
            }
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
                    onClick={() => {this.setState(this.history.pop()!)}}
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
            {this.state.searchQuery && <button 
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

function getPageTitle(page: WikiPage): string {
    const path = page.split('/');

    switch (path[0]) {
        case "role":
            return translate(`role.${path[1]}.name`);
        default:
            return translate(`wiki.entry.${page.replace('/', '.')}.title`)
    }
}

function getPageText(page: WikiPage): string {
    const path = page.split('/');

    switch (path[0]) {
        case "role":
            const role = path[1] as Role;
            const roleData = ROLES[role];
            return translate("wiki.entry.role",
                translate("role."+role+".name"),
                translateRoleListEntry(getRoleListEntryFromFactionAlignment(roleData.factionAlignment as FactionAlignment)) || '',
                translate("wiki.entry.role."+role+".abilities"),
                translate("wiki.entry.role."+role+".attributes"),
                roleData.maxCount === null ? translate("none") : roleData.maxCount,
                ROLES[role as keyof typeof ROLES].suspicious ? translate("suspicious") : translate("verdict.innocent"),
                translate("defense."+roleData.defense)
            )
        case "article":
            const article = path[1] as Article;
            return translate("wiki.entry.article",
                translate("wiki.entry.article."+article+".title"),
                translate("wiki.entry.article."+article+".text")
            )
        default:
            console.error(`Tried to get nonexistent wiki page at ${page}`);
            return translate("wiki.entry.article",
                translate("wiki.entry.article.404.title"),
                translate("wiki.entry.article.404.text")
            )
    }
}
