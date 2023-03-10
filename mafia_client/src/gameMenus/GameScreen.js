import React from "react";
import "../index.css"
import { LoadingMenu } from "../openMenus/LoadingMenu";
import { PhaseRowMenu } from "./PhaseRowMenu";
import { GraveyardMenu } from "./GraveyardMenu";
import { ChatMenu } from "./ChatMenu";
import { PlayerListMenu } from "./PlayerListMenu";
import { WillMenu } from "./WillMenu";

export class GameScreen extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {
            header: <LoadingMenu/>,
            content: [<LoadingMenu/>],
        };
    }
    componentDidMount() {
        GameScreen.instance = this;
        this.setState({
            header : <PhaseRowMenu/>,
            content : [<GraveyardMenu/> ,<ChatMenu/>, <PlayerListMenu/>, <WillMenu/>],
        });
    }
    componentWillUnmount() {
        //GameScreen.instance = undefined;
    }

    render(){return(<div style={{
        height: "100vh"
    }}>
        {this.renderHeader(this.state.header)}
        {this.renderContent(this.state.content)}
    </div>)}

    renderHeader(header){return(<div>
        {header}
    </div>)}
    renderContent(content){return(<div style={{
        display: "grid",

        gridAutoColumns: "1fr",
        gridAutoRows: "1fr",

        height: "85%",
        width: "100%",

        backgroundColor: "black",
        gridGap: "5px",
    }}>
        {
            content.map((panel, index)=>{
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
