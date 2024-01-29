import React from "react";
import ROLES from "./../resources/roles.json";
import translate, { langText, translateChecked } from "../game/lang";
import "./wikiSearch.css";
import { Role } from "../game/roleState.d";
import { ROLE_SETS, RoleSet, getRolesFromRoleSet } from "../game/roleListState.d";
import StyledText from "../components/StyledText";
import { HistoryQueue } from "../history";
import { regEscape } from "..";

type WikiSearchProps = {
    page?: WikiPage,
    excludedRoles?: Role[]
    pageChangeCallback?: (page: WikiPage) => void
}

type WikiSearchState = ({
    type: "search",
} | {
    type: "page",
    page: WikiPage,
}) & {
    searchQuery: string,
}

export type WikiPage = 
    | `role/${Role}`
    | `article/${Article}`;

const ARTICLES = ["how_to_play", "phases_and_timeline", "priority", "all_language", "role_sets"] as const;
type Article = typeof ARTICLES[number];

const PAGES: WikiPage[] = Object.keys(ROLES).map(role => `role/${role}`)
    .concat(ARTICLES.map(article => `article/${article}`)) as WikiPage[];

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

    static setPage(page: WikiPage) {
        WikiSearch.activeWikis.forEach(wiki => wiki.setPage(page));
    }

    componentDidMount() {
        WikiSearch.activeWikis.push(this);
    }
    componentWillUnmount() {
        WikiSearch.activeWikis.splice(WikiSearch.activeWikis.findIndex(wiki => wiki === this), 1);
    }

    setPage(page: WikiPage) {
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

    renderOpenPageButton(page: WikiPage) {

        let greyedOutRoles = this.props.excludedRoles
        if(greyedOutRoles === undefined){greyedOutRoles = [];}

        if(!greyedOutRoles.map((role)=>{return `role/${role}`}).includes(page)){
            return <button key={page} onClick={()=>{this.setPage(page)}}>
                <StyledText noLinks={true} markdown={true}>{getPageTitle(page)}</StyledText>
            </button>
        }else{
            //TODO ill fix it says jack keyword-dead
            return <button key={page} onClick={()=>{this.setPage(page)}}>
                <span className="keyword-dead">{getPageTitle(page)}</span>
            </button>
        }
    }

    getSearchResults(search: string): WikiPage[] {
        return PAGES.filter(page => RegExp(regEscape(search), 'i').test(getPageText(page)))
    }
    
    renderPageOrSearch(){
        if (this.state.type === "page") {
            if (this.state.page === "article/all_language") {
                return langText;
            }else if(this.state.page === "article/role_sets"){
                let mainElements = [];
                for(let set of ROLE_SETS){
                    mainElements.push(<StyledText key={set} className="wiki-content-body" markdown={true}>
                        ### {translate(set)}
                    </StyledText>);
                    
                    let elements = getRolesFromRoleSet(set as RoleSet).map((role)=>{
                        return <button key={role}>
                            <StyledText key={set} className="wiki-content-body">
                                {translate("role."+role+".name")}
                            </StyledText>
                        </button>
                    });
                    mainElements.push(<blockquote>
                        {elements}
                    </blockquote>);
                }
                return <div className="wiki-content-body">{mainElements}</div>;
            }else{
                return <StyledText className="wiki-content-body" markdown={true}>
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
            const keywords = roleData.keywords.map(key => {
                return `<details><summary>${translate("keyword."+key)}</summary>${translate("wiki.keyword." + key)}</details>`;
            }).join('\n');

            return translate("wiki.entry.role",
                translate("role."+role+".name"),
                // translateRoleOutline(getRoleOutlineFromFactionAlignment(roleData.factionAlignment as FactionAlignment)) || '',
                translateChecked("wiki.entry.role."+role+".guide") ?? translate("wiki.entry.role.noBasics"),
                translateChecked("wiki.entry.role."+role+".abilities") ?? translate("wiki.entry.role.noAbilities"),
                translateChecked("wiki.entry.role."+role+".attributes") ?? translate("wiki.entry.role.noAttributes"),
                translateChecked("wiki.entry.role."+role+".extra") ?? translate("wiki.entry.role.noExtra"),
                roleData.maxCount === null ? translate("none") : roleData.maxCount,
                translate("defense."+roleData.defense),
                keywords
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
