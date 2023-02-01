import React from "react";
import { JoinGameMenu } from "./openMenus/JoinGameMenu";
import { LobbyMenu } from "./openMenus/LobbyMenu";
import gameManager from "./game/gameManager";
import "./index.css"

export class Main extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {
            panels: [<JoinGameMenu/>, <LobbyMenu/>, ]
        };
    }
    componentDidMount() {
        Main.instance = this;
    }
    componentWillUnmount() {
        Main.instance = undefined;
    }

    render(){return(<div>
        {this.renderNavigation()}
        {this.renderGrid()}
    </div>)}

    renderNavigation(){return(
    <div style={{
        height: "6vh"
    }}>
        <br/>
        Navigation<br/>
        <br/>
    </div>)}
    renderGrid(){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",

        height: "94vh",
        width: "100%",

        backgroundColor: "black",

        gridGap: "5px",
    }}>
        {
            this.state.panels.map((panel, index)=>{
                return (<div 
                key={index}
                style={{
                    gridColumn: (index+1),
                    gridRow: 1,
                    
                    overflowX: "hidden",
                    height : "100%",
                    width: "100%",
                    
                    backgroundColor: "green",
                }}>
                    <br/>
                    <br/>
                    <br/>
                    <br/>
                    {panel}
                    <br/>
                    <br/>
                    <br/>
                    <br/>
                </div>)
            })
        }
    </div>)}
}
