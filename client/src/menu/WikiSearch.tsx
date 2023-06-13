import React from "react";
import ROLES from "./../resources/roles.json";
import translate, { styleText } from "../game/lang";
import { Role } from "../game/gameState.d";
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
                if(!out.includes(role)){
                    out.push(role);
                }
            }
        }
        for(let role in ROLES){
            if(
                translate("role."+role+".abilities").toLowerCase().includes(search) ||
                search.includes(translate("role."+role+".name").toLowerCase())
            ){
                if(!out.includes(role)){
                    out.push(role);
                }
            }
        }
        for(let role in ROLES){
            if(
                translate("role."+role+".attributes").toLowerCase().includes(search) ||
                search.includes(translate("role."+role+".name").toLowerCase())
            ){
                if(!out.includes(role)){
                    out.push(role);
                }
            }
        }
        return out;
    }
    renderRoleWiki(role: Role){
        let defenseString = "";
        switch(ROLES[role as keyof typeof ROLES].defense){
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
                return <button key={role} onClick={()=>{this.setState({expandedRole: role})}}>{styleText(translate("role."+role+".name"))}</button>
            })
        }
    }
    render() {return (<div className="wiki-search">
        <input type="text" value={this.state.wikiSearch}
            onChange={(e)=>{this.setState({wikiSearch: e.target.value, expandedRole: null})}}
            placeholder={translate("menu.wiki.search.placeholder")}
        />
        <div>
            <div>
                {this.renderExpandedRoleOrSearch()}
            </div>
        </div>
    </div>);}
}