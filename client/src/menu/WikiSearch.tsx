import React from "react";
import ROLES from "./../resources/roles.json";
import translate, { styleText } from "../game/lang";
import { FactionAlignment, Role, getRoleListEntryFromFactionAlignment, renderRoleListEntry } from "../game/gameState.d";
import "./wikiSearch.css";

interface WikiSearchState {
    wikiSearch: string,
    expandedRole: Role | null,
}

export default class WikiSearch extends React.Component<{}, WikiSearchState> {
    constructor(props: {}) {
        super(props);

        this.state = {
            wikiSearch: "",
            expandedRole: null,
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    getRolesFromSearch(search: string): Role[] {
        search = search.toLowerCase();
        let out: Role[] = [];
        for(let role in ROLES){
            if(
                translate("role."+role+".name").toLowerCase().includes(search) ||
                search.includes(translate("role."+role+".name").toLowerCase())
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        for(let role in ROLES){
            if(
                translate("role."+role+".abilities").toLowerCase().includes(search) ||
                search.includes(translate("role."+role+".name").toLowerCase())
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        for(let role in ROLES){
            if(
                translate("role."+role+".attributes").toLowerCase().includes(search) ||
                search.includes(translate("role."+role+".name").toLowerCase())
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        return out;
    }
    renderRoleWiki(role: Role){
        let defenseString = "";
        switch(ROLES[role].defense){
            case 0:
                defenseString = translate("none");
                break;
            case 1:
                defenseString = translate("basic");
                break;
            case 2:
                defenseString = translate("powerful");
                break;
            case 3:
                defenseString = translate("invincible");
                break;
        }

        return <div>
            <div>
                <div>{styleText(translate("role."+role+".name"))}</div>
                <div>{renderRoleListEntry(getRoleListEntryFromFactionAlignment(ROLES[role as keyof typeof ROLES].factionAlignment as FactionAlignment))}</div>
                <br/>
                {styleText(translate("menu.wiki.abilities"))}
                {this.renderRoleText(translate("role."+role+".abilities"))}
                <br/>
                {styleText(translate("menu.wiki.attributes"))}
                {this.renderRoleText(translate("role."+role+".attributes"))}
                <br/>
                {(() => {
                    let maxCount = ROLES[role as keyof typeof ROLES].maxCount;
                    if (maxCount == null) return maxCount;
                    return styleText(translate("menu.wiki.maxCount", maxCount));
                })()}<br/>
                {styleText(translate("menu.wiki.suspicious", ROLES[role as keyof typeof ROLES].suspicious?"suspicious":"innocent"))}<br/>
                {styleText(translate("menu.wiki.defense", defenseString))}<br/>
            </div>
        </div>
    }
    renderRoleText(string: string): JSX.Element{
        let split = string.split("*");
        let out = [];
        for(let i = 1; i < split.length; i++){
            out.push(<li key={i}>{split[i]}</li>);
        }
        return <div>{out}</div>
    }
    renderExpandedRoleOrSearch(){
        if(this.state.expandedRole != null){
            return this.renderRoleWiki(this.state.expandedRole);
        }else{
            return this.getRolesFromSearch(this.state.wikiSearch).map((role)=>{
                return <button key={role} 
                    onClick={()=>{this.setState({wikiSearch: translate("role."+role+".name"),expandedRole: role})}}
                >{
                    styleText(translate("role."+role+".name"))
                }</button>
            })
        }
    }
    render() {return (<div className="wiki-search">
        <div className="wiki-search-bar">
            <input type="text" value={this.state.wikiSearch}
                onChange={(e)=>{this.setState({wikiSearch: e.target.value, expandedRole: null})}}
                placeholder={translate("menu.wiki.search.placeholder")}
            />
            {this.state.wikiSearch !== null && this.state.wikiSearch !== "" ? 
                <button onClick={() => {
                    this.setState({wikiSearch: "", expandedRole: null})
                }}>{translate("menu.wiki.search.clear")}</button> 
            : null}
        </div>
        <div className="wiki-results">
            {this.renderExpandedRoleOrSearch()}
        </div>
    </div>);}
}