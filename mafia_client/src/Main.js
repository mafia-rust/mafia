import React from "react";
import "./index.css"
import { StartMenu } from "./openMenus/StartMenu";
import { PhaseRowMenu } from "./gameMenus/PhaseRowMenu";

export class Main extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {
            navigationRows: [<PhaseRowMenu/>],
            panels: [<StartMenu/>]
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
        {this.renderNavigation(this.state.navigationRows)}
        {this.renderGrid(this.state.panels)}
    </div>)}

    renderNavigation(navigationRows){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",
        
        height: "10%",
        width: "100%",

        backgroundColor: "black",
        gridGap: "5px",
    }}>
        {
            navigationRows.map((row, index)=>{
                return(<div
                key={index}
                style={{
                    gridColumn: 1,
                    gridRow: (index+1),
                    
                    overflowX: "hidden",
                    overflowY: "hidden",

                    height : "100%",
                    width: "100%",
                    
                    backgroundColor: "green",
                }}>
                    {row}
                </div>)
            })
        }
    </div>)}
    renderGrid(panels){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",

        height: "90%",
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
