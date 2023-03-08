import React from "react";
import "./index.css"
import { StartMenu } from "./openMenus/StartMenu";
import { TitleMenu } from "./openMenus/TitleMenu";

export class Main extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {
            rows: [<TitleMenu/>],
            panels: [<StartMenu/>],
        };
    }
    componentDidMount() {
        Main.instance = this;
    }
    componentWillUnmount() {
        //Main.instance = undefined;
    }

    render(){return(<div style={{
        height: "100vh"
    }}>
        {this.renderNavigation(this.state.rows)}
        {this.renderGrid(this.state.panels)}
    </div>)}

    renderNavigation(panels){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",

        height: "15%",
        width: "100%",

        overflowY:"hidden",

        backgroundColor: "black",
        gridGap: "5px",
    }}>
        {
            panels.map((panel, index)=>{
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
                    {panel}
                </div>)
            })
        }
    </div>)}
    renderGrid(panels){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",

        height: "85%",
        width: "100%",

        backgroundColor: "black",
        gridGap: "5px",
    }}>
        {
            panels.map((panel, index)=>{
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
