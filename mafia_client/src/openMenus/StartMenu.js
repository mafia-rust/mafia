import React from "react";
import {Main} from "../Main";
import { JoinGameMenu } from "./JoinGameMenu";

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
        <button style={{width: "90%"}} onClick={()=>{
            Main.instance.setState({panels : [<JoinGameMenu/>]});
        }}>Play</button><br/>
        <br/>
        <button style={{width: "90%"}}>Login</button><br/>
    </div>)}
}