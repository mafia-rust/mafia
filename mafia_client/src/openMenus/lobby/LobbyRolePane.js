import React from "react";
import ROLES from "../../resources/roles.json"
// import gameManager from "../../index.js";

export class LobbyRolePane extends React.Component {
    // constructor(props){
    //     super(props);
    // }
    render(){return(<div>
        <RolePicker/>
    </div>);}

}

class RolePicker extends React.Component {
    constructor(props) {
      super(props);
        this.state = {
            faction: "Any",
            alignment: null
        };
    }
    allFactions(){
        let factions = [];

        for(let role in ROLES){
            if( !factions.includes(ROLES[role].faction) ){
                factions.push(ROLES[role].faction);
            }
        }

        return factions;
    }
    allAlignments(faction){
        let alignments = [];
        let roles = [];

        for(let role in ROLES){
            if(ROLES[role].faction !== faction) 
                continue;

            if( !alignments.includes(ROLES[role].alignment) ){
                alignments.push(ROLES[role].alignment);
            }
            if( !roles.includes(role) ){
                roles.push(role);
            }
        }

        return alignments.concat(roles);
    }
    render() {
        return (<div>
            <select onChange={(e)=>{this.setState({faction: e.target.value})}}>
                {this.allFactions().map((faction)=>{
                    return <option key={faction}>{faction}</option>
                }).concat([<option key={"Any"}>Any</option>])}
            </select>
            <select>
                {
                    (this.state.faction !== "Any") && 
                    this.allAlignments(this.state.faction).map((alignment)=>{
                        return <option key={alignment}>{alignment}</option>
                    }).concat([<option key={"Random"}>Random</option>]) 
                }
            </select>
        </div>);
    }
}