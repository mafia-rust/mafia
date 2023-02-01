import React from "react";

export class StartMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }

    render(){return(<div>
        Mafia<br/>
        <br/>
        <button style={{width: "90%"}}>Play</button><br/>
        <br/>
        <button style={{width: "90%"}}>Login</button><br/>
    </div>)}
}