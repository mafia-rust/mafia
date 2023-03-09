import React from "react";
import "../index.css"

export class GameScreen extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {
            header: undefined,
            content: undefined,
        };
    }
    componentDidMount() {
        GameScreen.instance = this;
    }
    componentWillUnmount() {
        //GameScreen.instance = undefined;
    }

    render(){return(<div style={{
        height: "100vh"
    }}>
        {this.renderHeader(this.state.header)}
        {this.renderGrid(this.state.content)}
    </div>)}

    renderHeader(header){return(<div class="header">
        <div style={{
            height : "100%",
            width: "100%",
        }}>
            {header}
        </div>
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
